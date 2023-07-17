use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Address {
    pub id: Option<u64>,
    pub line1: Option<String>,
    // Todo: add line2
    pub city: Option<String>,
    #[sqlx(rename = "StateName")]
    pub state: Option<String>,
    pub zip: Option<u64>,
}

impl Address {
    pub const XML_IDENTIFIER: &'static str = "Address";

    pub fn new() -> Self {
        Address {
            id: None,
            line1: None,
            city: None,
            state: None,
            zip: None,
        }
    }
}
