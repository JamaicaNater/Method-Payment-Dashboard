use crate::entities::entity::{Address, Individual};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct EntityResponse {
    pub id: String,
    // Dest accounts have these
    #[serde(rename = "type")]
    pub entity_type: String,
    // Source accounts have these
    pub individual: Individual,
    pub corporation: Option<String>,
    pub receive_only: Option<String>,
    pub address: Address,
    pub capabilities: Vec<String>,
    // TODO: what is this type?
    pub error: Option<String>,
    pub status: String,
    pub metadata: Option<HashMap<String, String>>,
    pub updated_at: String,
    pub created_at: String,
}
