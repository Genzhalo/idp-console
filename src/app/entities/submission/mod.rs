use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::status::SubmissionStatus;

use super::{form::Form, respondent::Respondent};
pub mod status;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Submission {
    pub id: String,
    pub form: Form,
    pub respondent: Respondent,
    pub arrival_date: DateTime<Utc>,
    pub sub_order: u32,
    pub status: SubmissionStatus,
    pub created_at: DateTime<Utc>,
}
