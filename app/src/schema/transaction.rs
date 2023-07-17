use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Transaction {
    pub method_id: Option<String>,
    pub employee_id: Option<String>,
    pub payee_id: Option<String>,
    pub payor_id: Option<String>,
    pub xml_id: Option<u64>,
    pub amount: Option<u64>, // Amount in cents
}

impl Transaction {
    pub const XML_IDENTIFIER: &'static str = "row";

    pub fn new() -> Self {
        Transaction {
            method_id: None,
            employee_id: None,
            payee_id: None,
            payor_id: None,
            xml_id: None,
            amount: None,
        }
    }
}
