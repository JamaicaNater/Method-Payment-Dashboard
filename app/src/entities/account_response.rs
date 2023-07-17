use crate::entities::account::ACH;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AccountResponse {
    pub id: String,
    // Dest accounts have these
    pub entity_type: Option<String>,
    // Source accounts have these
    pub holder_id: Option<String>,
    #[serde(rename = "type")]
    pub acc_type: String,
    pub ach: Option<ACH>,
    pub liability: Option<Liability>,
    // TODO: what is this type?
    pub clearing: Option<String>,
    // TODO: what is this type?
    pub metadata: Option<HashMap<String, String>>,
    pub status: String,
    pub capabilities: Vec<String>,
    // TODO: what is this type?
    pub error: Option<String>,
    pub updated_at: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct Liability {
    pub mch_id: String,
    pub mask: String,
    #[serde(rename = "type")]
    pub liability_type: String,
    pub data_status: String,
    // TODO: can this be a datetime?
    pub data_last_successful_sync: Option<String>,
    // TODO: deserialized name can vary
    pub loan: Option<Loan>,
}

#[derive(Deserialize, Serialize)]
pub struct Loan {
    pub name: String,
    pub sub_name: String,
    pub sequence: u64,
    pub balance: u64,
    pub last_payment_amount: u64,
    pub last_payment_date: String,
    pub next_payment_minimum_amount: u64,
    pub dispersed_at: String,
    pub interest_rate_percentage: u16,
    pub interest_rate_type: String,
}
