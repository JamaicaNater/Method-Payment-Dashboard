use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct XmlParse {
    pub id: Option<u64>,
    pub filename: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
}

impl XmlParse {
    pub const COLUMN_NAME: &'static str = "XmlParse";

    pub fn new(filename: String, status: String) -> Self {
        let time: DateTime<Utc> = SystemTime::now().into();

        Self {
            id: None,
            filename,
            status,
            started_at: time.format("%d/%m/%Y %T").to_string(),
            finished_at: None,
        }
    }
}
