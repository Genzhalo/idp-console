use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Respondent {
    pub id: String,
    pub passport_id: String,
    #[serde(rename = "IDPCode")]
    pub idp_code: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub region: String,
    pub children: u8,
    pub created_at: DateTime<Utc>,
}
