use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Employee {
    pub dunkin_id: Option<String>,
    pub method_id: Option<String>,
    pub dunkin_branch: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub dob: Option<String>,
    pub phone_number: Option<String>,
}

impl Employee {
    pub const XML_IDENTIFIER: &'static str = "Employee";
    pub fn new() -> Self {
        Employee {
            dunkin_id: None,
            method_id: None,
            dunkin_branch: None,
            first_name: None,
            last_name: None,
            dob: None,
            phone_number: None,
        }
    }
}
