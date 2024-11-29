use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct UserToken {
    pub token: String,
    pub used_for: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_alg: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
