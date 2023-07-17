use crate::schema::address::Address;
use crate::schema::employee::Employee;
use crate::schema::payee::Payee;
use crate::schema::payor::Payor;
use crate::schema::transaction::Transaction;
use crate::schema::xml_parse::XmlParse;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use log::debug;
use sql_builder::SqlBuilder;
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySql, Pool};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::time::SystemTime;

pub mod address;
pub(crate) mod db;
pub mod employee;
pub mod payee;
pub mod payor;
pub mod transaction;
pub mod xml_parse;

impl From<SqlString> for String {
    fn from(value: SqlString) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SqlString(String);

impl Display for SqlString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SqlString {
    fn from(value: String) -> Self {
        Self {
            0: format!("{}", value),
        }
    }
}

impl From<&str> for SqlString {
    fn from(value: &str) -> Self {
        let string = value.to_string();
        SqlString::from(string)
    }
}

impl From<u64> for SqlString {
    fn from(value: u64) -> Self {
        Self {
            0: format!("{}", value),
        }
    }
}

impl<T: Into<SqlString>> From<Option<T>> for SqlString {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => val.into(),
            None => Self {
                0: "NULL".to_string(),
            },
        }
    }
}

#[async_trait]
pub trait CRUD<KeyType>: Sized + Send + Unpin + for<'r> FromRow<'r, MySqlRow>
where
    KeyType: Display,
    SqlString: From<KeyType>,
{
    const TABLE_NAME: &'static str;

    const ID_FIELD: &'static str;

    fn get_id(&self) -> KeyType;

    fn get_all_fields() -> Vec<&'static str>;

    fn get_all_values(&self) -> Vec<SqlString>;

    async fn get_by(
        pool: &Pool<MySql>,
        where_clauses: HashMap<&str, SqlString>,
    ) -> Result<Vec<Self>, sqlx::Error>
    where
        KeyType: Display + Into<SqlString>,
    {
        let mut query_builder = SqlBuilder::select_from(Self::TABLE_NAME);

        for where_clause in where_clauses {
            query_builder.and_where_eq(where_clause.0, format!("'{}'", where_clause.1));
        }

        let query = query_builder.fields(&["*"]).sql().unwrap();
        debug!("Executing query: {}", query);

        let result: Vec<Self> = sqlx::query_as(query.as_str()).fetch_all(pool).await?;
        Ok(result)
    }

    async fn get_in(
        pool: &Pool<MySql>,
        where_in_clauses: HashMap<&str, Vec<String>>,
    ) -> Result<Vec<Self>, sqlx::Error>
    where
        KeyType: Display + Into<SqlString>,
    {
        let mut query_builder = SqlBuilder::select_from(Self::TABLE_NAME);

        for where_clause in where_in_clauses {
            query_builder
                .and_where_in_query(where_clause.0, format!("'{}'", where_clause.1.join("', '")));
        }

        let query = query_builder.fields(&["*"]).sql().unwrap();
        debug!("Executing query: {}", query);

        let result: Vec<Self> = sqlx::query_as(query.as_str()).fetch_all(pool).await?;
        Ok(result)
    }

    fn insert_query(&self) -> String {
        let fields = Self::get_all_fields();
        let placeholder = vec!["?"; fields.len()];

        SqlBuilder::insert_into(Self::TABLE_NAME)
            .fields(fields.as_slice())
            .values(placeholder.as_slice())
            .sql()
            .unwrap()
    }

    fn update_query(&self) -> String {
        let fields = Self::get_all_fields();
        let values = self.get_all_values();

        SqlBuilder::update_table(Self::TABLE_NAME)
            .and_where_eq(Self::ID_FIELD, SqlString::from(self.get_id()))
            .fields(fields.as_slice())
            .values(values.as_slice())
            .sql()
            .unwrap()
    }

    // Switch to KeyType
    async fn insert(&self, pool: &Pool<MySql>) -> Result<u64, sqlx::Error> {
        let query = self.insert_query();
        let values = self.get_all_values();

        debug!("Executing query: {}, with bindings {:?}", query, values);
        let mut query_builder = sqlx::query(&query);

        for value in &values {
            query_builder = query_builder.bind(value.clone().0);
        }

        let result = query_builder.execute(pool).await?;
        debug!(
            "result id: {}, rows {}",
            result.last_insert_id(),
            result.rows_affected()
        );
        Ok(result.last_insert_id())
    }
}

impl CRUD<u64> for XmlParse {
    const TABLE_NAME: &'static str = "XmlParse";

    const ID_FIELD: &'static str = "Id";

    fn get_id(&self) -> u64 {
        self.id.expect("Id was set")
    }

    fn get_all_fields() -> Vec<&'static str> {
        vec!["Filename", "Status", "StartedAt", "FinishedAt"]
    }

    fn get_all_values(&self) -> Vec<SqlString> {
        vec![
            SqlString::from(self.clone().filename),
            SqlString::from(self.clone().status),
            SqlString::from(self.clone().started_at),
            SqlString::from(self.clone().finished_at),
        ]
    }
}

impl XmlParse {
    pub async fn set_finished(
        &mut self,
        pool: &Pool<MySql>,
        status: String,
    ) -> Result<(), sqlx::Error> {
        let time: DateTime<Utc> = SystemTime::now().into();
        self.finished_at = Some(time.format("%d/%m/%Y %T").to_string());
        self.status = status;

        let query = SqlBuilder::update_table(Self::TABLE_NAME)
            // Todo: Fix this
            .set(
                "Status",
                format!("'{}'", SqlString::from(self.status.clone())),
            )
            .set(
                "FinishedAt",
                format!("'{}'", SqlString::from(self.finished_at.clone())),
            )
            .and_where_eq("Id", format!("'{}'", SqlString::from(self.id.clone())))
            .sql()
            .unwrap();

        debug!("Executing query: {}", query);

        sqlx::query(query.as_str()).execute(pool).await?;
        Ok(())
    }

    // TODO: make generic, include in CRUD
    pub async fn get_all_transactions_by_xml_id(
        pool: &Pool<MySql>,
        xml_id: u64,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let query = SqlBuilder::select_from(Transaction::TABLE_NAME)
            .fields(&["*"])
            .and_where_eq("XmlId", SqlString::from(xml_id))
            .sql()
            .unwrap();

        let result: Vec<Transaction> = sqlx::query_as(query.as_str()).fetch_all(pool).await?;
        Ok(result)
    }
}

impl CRUD<String> for Transaction {
    const TABLE_NAME: &'static str = "Transactions";

    const ID_FIELD: &'static str = "MethodId";

    fn get_id(&self) -> String {
        self.method_id.clone().expect("MethodId was set")
    }

    fn get_all_fields() -> Vec<&'static str> {
        vec![
            "MethodId",
            "EmployeeId",
            "PayorId",
            "PayeeId",
            "XmlId",
            "Amount",
        ]
    }

    fn get_all_values(&self) -> Vec<SqlString> {
        vec![
            SqlString::from(self.clone().method_id),
            SqlString::from(self.clone().employee_id),
            SqlString::from(self.clone().payor_id),
            SqlString::from(self.clone().payee_id),
            SqlString::from(self.clone().xml_id),
            SqlString::from(self.clone().amount),
        ]
    }
}

impl CRUD<String> for Employee {
    const TABLE_NAME: &'static str = "Employees";

    const ID_FIELD: &'static str = "MethodId";

    fn get_id(&self) -> String {
        self.method_id.clone().expect("MethodId was set")
    }

    fn get_all_fields() -> Vec<&'static str> {
        vec![
            "MethodId",
            "DunkinId",
            "DunkinBranch",
            "FirstName",
            "LastName",
            "Dob",
            "PhoneNumber",
        ]
    }

    fn get_all_values(&self) -> Vec<SqlString> {
        vec![
            SqlString::from(self.clone().method_id),
            SqlString::from(self.clone().dunkin_id),
            SqlString::from(self.clone().dunkin_branch),
            SqlString::from(self.clone().first_name),
            SqlString::from(self.clone().last_name),
            SqlString::from(self.clone().dob),
            SqlString::from(self.clone().phone_number),
        ]
    }
}

impl Employee {
    pub async fn get_by_id(pool: &Pool<MySql>, id: String) -> Result<Option<Self>, sqlx::Error> {
        let query = SqlBuilder::select_from(Employee::TABLE_NAME)
            .fields(&["*"])
            .and_where_eq("MethodId", SqlString::from(id))
            .sql()
            .unwrap();
        debug!("Executing query: {}", query);
        sqlx::query_as(query.as_str()).fetch_optional(pool).await
    }
}

impl CRUD<String> for Payee {
    const TABLE_NAME: &'static str = "Payees";

    const ID_FIELD: &'static str = "MethodId";

    fn get_id(&self) -> String {
        self.method_id.clone().expect("MethodId was set")
    }

    fn get_all_fields() -> Vec<&'static str> {
        vec!["MethodId", "PlaidId"]
    }

    fn get_all_values(&self) -> Vec<SqlString> {
        vec![
            SqlString::from(self.clone().method_id),
            SqlString::from(self.clone().plaid_id),
        ]
    }
}

impl CRUD<String> for Payor {
    const TABLE_NAME: &'static str = "Payors";

    const ID_FIELD: &'static str = "MethodId";

    fn get_id(&self) -> String {
        self.method_id.clone().expect("MethodId was set")
    }

    fn get_all_fields() -> Vec<&'static str> {
        vec![
            "MethodId",
            "DunkinId",
            "PayorName",
            "DBA",
            "EIN",
            "AddressId",
        ]
    }

    fn get_all_values(&self) -> Vec<SqlString> {
        vec![
            SqlString::from(self.clone().method_id),
            SqlString::from(self.clone().dunkin_id),
            SqlString::from(self.clone().payor_name),
            SqlString::from(self.clone().dba),
            SqlString::from(self.clone().ein),
            SqlString::from(self.clone().address_id),
        ]
    }
}

impl CRUD<u64> for Address {
    const TABLE_NAME: &'static str = "Addresses";

    const ID_FIELD: &'static str = "Id";

    fn get_id(&self) -> u64 {
        self.id.clone().expect("Id was set")
    }

    fn get_all_fields() -> Vec<&'static str> {
        vec!["Line1", "City", "StateName", "Zip"]
    }

    fn get_all_values(&self) -> Vec<SqlString> {
        vec![
            SqlString::from(self.clone().line1),
            SqlString::from(self.clone().city),
            SqlString::from(self.clone().state),
            SqlString::from(self.clone().zip),
        ]
    }
}
