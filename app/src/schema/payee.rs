use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Payee {
    pub plaid_id: Option<String>,
    pub method_id: Option<String>,
    #[sqlx(default)]
    pub loan_account_number: Option<u64>,
}

impl Payee {
    pub const XML_IDENTIFIER: &'static str = "Payee";
    pub fn new() -> Self {
        Payee {
            plaid_id: None,
            method_id: None,
            loan_account_number: None,
        }
    }
}
