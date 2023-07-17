use axum::body::Bytes;
use hyper::body::Buf;
use log::{debug, error, info, trace, warn};
use sqlx::{MySql, Pool};
use std::fmt;
use std::io::{BufReader, Read};

use crate::entities;
use crate::entities::Persist;
use crate::schema::employee::Employee;
use crate::schema::payee::Payee;
use crate::schema::payor::Payor;
use crate::schema::transaction::Transaction;
use crate::schema::{address, transaction};
use crate::utility::parser::ParseError::IOError;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedElement,
    IOError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedElement => write!(f, "Unexpected element error"),
            ParseError::IOError => write!(f, "IO error"),
        }
    }
}

impl From<entities::Error> for ParseError {
    fn from(value: entities::Error) -> Self {
        error!("Error : {}", value);
        IOError
    }
}

pub async fn parse(
    pool: &Pool<MySql>,
    file: Bytes,
    xml_id: u64,
) -> Result<Vec<Transaction>, ParseError> {
    let file = BufReader::new(file.reader()); // Buffering is important for performance
    let mut transactions: Vec<Transaction> = vec![];
    let mut parser = EventReader::new(file);
    loop {
        match parser.next() {
            Ok(XmlEvent::EndDocument) => {
                info!("End of document");
                break;
            }
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == Transaction::XML_IDENTIFIER.to_string() {
                    match parse_transaction(&mut parser, pool, xml_id).await {
                        Ok(transaction) => {
                            transactions.push(transaction);
                        }
                        Err(e) => {
                            error!("Transaction failed due to {}, skipping", e)
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error parsing document due to {e}");
                break;
            }
            _ => {}
        }
    }
    Ok(transactions)
}

async fn parse_transaction<R: Read>(
    parser: &mut EventReader<BufReader<R>>,
    pool: &Pool<MySql>,
    xml_id: u64,
) -> Result<transaction::Transaction, ParseError> {
    info!("Parsing Transaction");
    let mut transaction = transaction::Transaction::new();
    let mut cur_element = String::from("");

    let mut payor: Option<Payor> = None;
    let mut employee: Option<Employee> = None;
    let mut payee: Option<Payee> = None;

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                cur_element = name.local_name.clone();
                trace!("Current Element: {}", cur_element);
                if name.local_name.as_str() == Employee::XML_IDENTIFIER {
                    employee = Some(parse_employee(parser));
                }
                if name.local_name.as_str() == Payor::XML_IDENTIFIER {
                    payor = Some(parse_payor(parser, pool).await?);
                }
                if name.local_name.as_str() == Payee::XML_IDENTIFIER {
                    payee = Some(parse_payee(parser));
                }
            }

            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name.as_str() == Transaction::XML_IDENTIFIER {
                    break;
                }
            }

            Ok(XmlEvent::Characters(text)) => match cur_element.to_lowercase().as_str() {
                "amount" => {
                    let raw = text.replace("$", "").parse::<f64>().unwrap() * 100.0;
                    transaction.amount = Some(raw as u64);
                }
                _ => {
                    error!(
                        "Transaction: Failed to match identifier '{}' with value '{}'",
                        cur_element, text
                    );
                }
            },

            Ok(XmlEvent::Whitespace(..)) => {}
            Err(e) => {
                error!("Error: {e}");
                break;
            }
            Ok(element) => {
                warn!("Unexpected Element '{:?}'", element);
                break;
            }
        }
    }

    let mut employee = employee.unwrap();
    employee.persist(pool, ()).await?;
    transaction.employee_id = employee.method_id.clone();

    let mut payor = payor.unwrap();
    payor.persist(pool, ()).await?;
    transaction.payor_id = payor.method_id;

    let mut payee = payee.unwrap();
    payee.persist(pool, employee.method_id.unwrap()).await?;
    transaction.payee_id = payee.method_id;

    transaction.xml_id = Some(xml_id);
    transaction.persist(pool, ()).await?;

    info!("Transaction {:?}", transaction);
    return Ok(transaction);
}

fn parse_employee<R: Read>(parser: &mut EventReader<BufReader<R>>) -> Employee {
    info!("Parsing Employee");
    let mut employee = Employee::new();
    let mut cur_element = String::from("");
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                cur_element = name.local_name;
            }

            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name.as_str() == Employee::XML_IDENTIFIER {
                    break;
                }
            }

            Ok(XmlEvent::Characters(text)) => match cur_element.to_lowercase().as_str() {
                "dunkinid" => {
                    employee.dunkin_id = Some(text);
                }
                "dunkinbranch" => {
                    employee.dunkin_branch = Some(text);
                }
                "firstname" => {
                    employee.first_name = Some(text);
                }
                "lastname" => {
                    employee.last_name = Some(text);
                }
                "dob" => {
                    employee.dob = Some(text);
                }
                "phonenumber" => {
                    employee.phone_number = Some(text);
                }
                _ => {
                    error!(
                        "Employee: Failed to match identifier '{}' with value '{}'",
                        cur_element, text
                    );
                }
            },

            Ok(XmlEvent::Whitespace(..)) => {}
            Err(e) => {
                error!("Error: {e}");
                break;
            }
            Ok(element) => {
                warn!("Unexpected Element '{:?}'", element);
                break;
            }
        }
    }
    info!("Finished parsing employee: {:?}", employee);
    return employee;
}

fn parse_payee<R: Read>(parser: &mut EventReader<BufReader<R>>) -> Payee {
    info!("Parsing Payee");
    let mut payee = Payee::new();
    let mut cur_element = String::from("");
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                cur_element = name.local_name;
            }

            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name.as_str() == Payee::XML_IDENTIFIER {
                    break;
                }
            }

            Ok(XmlEvent::Characters(text)) => match cur_element.to_lowercase().as_str() {
                "plaidid" => {
                    payee.plaid_id = Some(text);
                }
                "loanaccountnumber" => {
                    payee.loan_account_number = Some(text.parse::<u64>().unwrap());
                }
                _ => {
                    error!(
                        "Payee: Failed to match identifier '{}' with value '{}'",
                        cur_element, text
                    );
                }
            },

            Ok(XmlEvent::Whitespace(..)) => {}
            Err(e) => {
                error!("Error: {e}");
                break;
            }
            Ok(element) => {
                warn!("Unexpected Element '{:?}'", element);
                break;
            }
        }
    }
    info!("Finished parsing payee {:?}", payee);
    return payee;
}

async fn parse_payor<R: Read>(
    parser: &mut EventReader<BufReader<R>>,
    pool: &Pool<MySql>,
) -> Result<Payor, ParseError> {
    info!("Parsing payor");
    let mut payor = Payor::new();
    let mut cur_element = String::from("");
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == address::Address::XML_IDENTIFIER.to_string() {
                    let mut address = parse_address(parser);
                    address.persist(pool, ()).await?;

                    debug!("Address id {}", address.id.expect("Address Id was set"));
                    payor.address_id = address.id;
                }
                cur_element = name.local_name;
            }

            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name.as_str() == Payor::XML_IDENTIFIER {
                    break;
                }
            }

            Ok(XmlEvent::Characters(text)) => match cur_element.to_lowercase().as_str() {
                "dunkinid" => {
                    payor.dunkin_id = Some(text);
                }
                "name" => {
                    payor.payor_name = Some(text);
                }
                "dba" => {
                    payor.dba = Some(text);
                }
                "ein" => {
                    payor.ein = Some(text);
                }
                "accountnumber" => {
                    payor.account_number = Some(text.parse::<u64>().unwrap());
                }
                "abarouting" => {
                    payor.aba_routing = Some(text.parse::<u64>().unwrap());
                }
                _ => {
                    error!(
                        "Payor: Failed to match identifier '{}' with value '{}'",
                        cur_element, text
                    );
                }
            },

            Ok(XmlEvent::Whitespace(..)) => {}
            Err(e) => {
                error!("Error: {e}");
                break;
            }
            Ok(element) => {
                warn!("Unexpected Element '{:?}'", element);
                break;
            }
        }
    }
    info!("Finished parsing payor {:?}", payor);
    return Ok(payor);
}

fn parse_address<R: Read>(parser: &mut EventReader<BufReader<R>>) -> address::Address {
    info!("Parsing address");
    let mut address = address::Address::new();
    let mut cur_element = String::from("");
    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                cur_element = name.local_name;
            }

            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name.as_str() == address::Address::XML_IDENTIFIER {
                    break;
                }
            }

            Ok(XmlEvent::Characters(text)) => match cur_element.to_lowercase().as_str() {
                "line1" => {
                    address.line1 = Some(text);
                }
                "city" => {
                    address.city = Some(text);
                }
                "state" => {
                    address.state = Some(text);
                }
                "zip" => {
                    address.zip = Some(text.parse::<u64>().unwrap());
                }
                _ => {
                    error!(
                        "Address: Failed to match identifier '{}' with value '{}'",
                        cur_element, text
                    );
                }
            },

            Ok(XmlEvent::Whitespace(..)) => {}
            Err(e) => {
                error!("Error: {e}");
                break;
            }
            Ok(element) => {
                warn!("Unexpected Element '{:?}'", element);
                break;
            }
        }
    }
    info!("Finished parsing address {:?}", address);
    return address;
}
