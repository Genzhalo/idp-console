use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::status::FormStatus;
pub mod status;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub id: String,
    pub name: String,
    pub limit: u16,
    pub status: FormStatus,
    pub time_frame_duration: u16,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub exclude_form_ids: Vec<String>,
}
