use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PaymentResponse {
    pub id: String,
    pub reversal_id: Option<String>,
    pub source_trace_id: Option<String>,
    pub destination_trace_id: Option<String>,
    pub source: String,
    pub destination: String,
    pub amount: u64,
    pub description: String,
    pub status: String,
    pub error: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub estimated_completion_date: String,
    pub source_settlement_date: String,
    pub destination_settlement_date: String,
    pub fee: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
}
