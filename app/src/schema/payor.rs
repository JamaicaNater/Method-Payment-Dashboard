use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Payor {
    pub dunkin_id: Option<String>,
    pub method_id: Option<String>,
    #[sqlx(default)]
    pub aba_routing: Option<u64>,
    #[sqlx(default)]
    pub account_number: Option<u64>,
    pub payor_name: Option<String>,
    pub dba: Option<String>,
    pub ein: Option<String>,
    pub address_id: Option<u64>,
}

impl Payor {
    pub const XML_IDENTIFIER: &'static str = "Payor";
    pub fn new() -> Self {
        Self {
            dunkin_id: None,
            method_id: None,
            aba_routing: None,
            account_number: None,
            payor_name: None,
            dba: None,
            ein: None,
            address_id: None,
        }
    }
}
