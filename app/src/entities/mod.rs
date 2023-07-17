use crate::entities::account::DestAccount;
use crate::entities::entity::Entity;
use crate::entities::payment::Payment;
use crate::entities::Error::{DatabaseError, HTTPError};
use crate::schema::employee::Employee;
use crate::schema::payee::Payee;
use crate::schema::payor::Payor;
use crate::schema::transaction::Transaction;
use crate::schema::{address, SqlString, CRUD};
use crate::utility::method_client;
use crate::utility::method_client::{post_dest_account, post_entity, post_payment};
use async_trait::async_trait;
use log::{info, warn};
use sqlx::{MySql, Pool};
use std::collections::HashMap;

pub mod account;
pub mod account_response;
pub mod entity;
pub mod entity_response;
pub mod payment;
pub mod payment_response;

pub type BoxDynError = Box<dyn std::error::Error + 'static + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http Error: {0}")]
    HTTPError(#[source] method_client::Error),
    #[error("Database Error: {0}")]
    DatabaseError(#[source] sqlx::Error),
    #[error("Invalid Data Error: {0}")]
    InvalidDataError(String),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        DatabaseError(value)
    }
}

impl From<method_client::Error> for Error {
    fn from(value: method_client::Error) -> Self {
        HTTPError(value)
    }
}

#[async_trait]
pub trait Persist {
    type Dependencies;

    async fn persist(
        &mut self,
        pool: &Pool<MySql>,
        dependency: Self::Dependencies,
    ) -> Result<(), Error>;
}

#[async_trait]
impl Persist for Employee {
    type Dependencies = ();

    async fn persist(&mut self, pool: &Pool<MySql>, _: Self::Dependencies) -> Result<(), Error> {
        info!(
            "Persisting Employee {}",
            self.dunkin_id.clone().expect("DunkinId was set")
        );
        let employees = Employee::get_by(
            &pool,
            HashMap::from([("DunkinId", SqlString::from(self.dunkin_id.clone()))]),
        )
        .await?;

        match employees.len() {
            0 => {
                let entity = Entity::from(self.clone());
                let entity_response = post_entity(entity).await?;
                self.method_id = Some(entity_response.id);

                self.insert(pool).await?;
            }

            1 => {
                let e = employees.first().unwrap().clone();
                info!(
                "Skipping persistence of existing payor entry with dunkin_id: {} and method_id {}",
                e.dunkin_id.unwrap(),
                e.method_id.clone().unwrap()
            );
                self.method_id = e.method_id;
            }

            _ => {
                let e = employees.first().unwrap().clone();
                warn!(
                    "Unexpected Duplicate Entry for {} with dunkin_id {}, and method_id {}",
                    Self::TABLE_NAME,
                    e.dunkin_id.unwrap(),
                    e.method_id.clone().unwrap()
                );
                self.method_id = e.method_id;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Persist for Payor {
    type Dependencies = ();

    async fn persist(&mut self, pool: &Pool<MySql>, _: Self::Dependencies) -> Result<(), Error> {
        // // Todo: store in Db to begin with
        // let ceo_entity = Entity {
        //     entity_type: "individual".to_string(),
        //     individual: Individual {
        //         first_name: "John".to_string(),
        //         last_name: "Dunkin".to_string(),
        //         dob: "1999-09-20".to_string(),
        //         email: "john.dunkin@dunkin.com".to_string(),
        //         phone: "+15121231111".to_string(),
        //     },
        //     address: entity::Address {
        //         line1: "3300 N Interstate 35".to_string(),
        //         line2: "".to_string(),
        //         city: "Austin".to_string(),
        //         state: "TX".to_string(),
        //         zip: "78705".to_string(),
        //     },
        // };
        // let ceo_entity_response = post_entity(ceo_entity).await?;
        // // Todo Store in DB
        // let payor_owner = ceo_entity_response.id;

        let payors = Payor::get_by(
            pool,
            HashMap::from([("DunkinId", SqlString::from(self.dunkin_id.clone()))]),
        )
        .await?;

        match payors.len() {
            0 => {
                // let mut account = SourceAccount::from(self.clone());
                // account.holder_id = payor_owner.clone();
                // let account_response = post_source_account(account).await?;
                // self.method_id = Some(account_response.id);
                //
                // self.insert(pool).await?;

                return Err(Error::InvalidDataError(format!(
                    "Payor {} is not on of the 5(4) valid payors",
                    SqlString::from(self.dunkin_id.clone())
                )));
            }

            1 => {
                let p = payors.first().unwrap().clone();
                info!(
                "Skipping persistence of existing payor entry with dunkin_id: {} and method_id {}",
                p.dunkin_id.unwrap(),
                p.method_id.clone().unwrap()
            );
                self.method_id = p.method_id;
            }

            _ => {
                let p = payors.first().unwrap().clone();
                warn!(
                    "Unexpected Duplicate Entry for {} with dunkin_id {}, and method_id {}",
                    Self::TABLE_NAME,
                    p.dunkin_id.unwrap(),
                    p.method_id.clone().unwrap()
                );
                self.method_id = p.method_id;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Persist for Payee {
    type Dependencies = String;

    async fn persist(
        &mut self,
        pool: &Pool<MySql>,
        dependency: Self::Dependencies,
    ) -> Result<(), Error> {
        let payees = Payee::get_by(
            pool,
            HashMap::from([("PlaidId", SqlString::from(self.plaid_id.clone()))]),
        )
        .await?;

        match payees.len() {
            0 => {
                let employee_method_id = dependency;
                let mut account = DestAccount::from(self.clone());
                // TODO Pass in employee
                account.holder_id = employee_method_id;
                let account_response = post_dest_account(account).await?;
                self.method_id = Some(account_response.id);

                self.insert(pool).await?;
            }

            1 => {
                let p = payees.first().unwrap().clone();
                info!(
                "Skipping persistence of existing payee entry with plaid_id: {} and method_id {}",
                p.plaid_id.unwrap(),
                p.method_id.clone().unwrap()
            );
                self.method_id = p.method_id
            }

            _ => {
                let p = payees.first().unwrap().clone();
                warn!(
                    "Unexpected Duplicate Entry for {} with plaid_id {}, and method_id {}",
                    Self::TABLE_NAME,
                    p.plaid_id.unwrap(),
                    p.method_id.clone().unwrap()
                );
                self.method_id = p.method_id
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Persist for Transaction {
    type Dependencies = ();

    async fn persist(&mut self, pool: &Pool<MySql>, _: Self::Dependencies) -> Result<(), Error> {
        // We dont do the same checks here sine its theoretically possible for use to have a
        // transaction for the same amount, payee, payor, xml_id, and employee id to occur

        let payment = Payment {
            amount: self.amount.clone().expect("Amount was set"),
            source: self.payor_id.clone().expect("Payor id was set"),
            destination: self.payee_id.clone().expect("Payee id was set"),
            // Todo add desc
            description: self.xml_id.expect("Xml Id was set").to_string(),
        };
        let payment_response = post_payment(payment).await?;

        self.method_id = Some(payment_response.id.clone());

        self.insert(pool).await?;
        Ok(())
    }
}

#[async_trait]
impl Persist for address::Address {
    type Dependencies = ();

    async fn persist(
        &mut self,
        pool: &Pool<MySql>,
        _dependency: Self::Dependencies,
    ) -> Result<(), Error> {
        let addresses = address::Address::get_by(
            pool,
            HashMap::from([
                ("Line1", SqlString::from(self.line1.clone())),
                ("City", SqlString::from(self.city.clone())),
                ("StateName", SqlString::from(self.state.clone())),
                ("Zip", SqlString::from(self.zip.clone())),
            ]),
        )
        .await?;

        match addresses.len() {
            0 => {
                self.id = Some(self.insert(pool).await?);
            }

            1 => {
                let a = addresses.first().unwrap().clone();
                self.id = a.id;
            }

            _ => {
                let a = addresses.first().unwrap().clone();
                warn!(
                    "Unexpected Duplicate Entry for {} with address_id {}",
                    Self::TABLE_NAME,
                    a.id.unwrap(),
                );
                self.id = a.id;
            }
        };

        Ok(())
    }
}
