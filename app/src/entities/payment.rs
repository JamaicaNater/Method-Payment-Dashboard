use serde::Serialize;

#[derive(Serialize)]
pub struct Payment {
    pub amount: u64,
    pub source: String,
    pub destination: String,
    pub description: String,
}
