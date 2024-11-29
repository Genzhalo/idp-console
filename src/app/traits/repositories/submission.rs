use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::app::entities::submission::Submission;

#[async_trait]
pub trait TSubmissionRepositories {
    async fn insert(
        &self,
        form_id: &str,
        respondent_id: &str,
        arrival_date: NaiveDateTime,
        sub_order: i32,
        status: &str,
    ) -> Result<String, String>;
    async fn find(&self, by_form: Option<String>, by_respondent: Option<String>)
        -> Vec<Submission>;
    async fn find_by_id(&self, id: &str) -> Option<Submission>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn update(&self, id: &str, status: &Option<String>) -> Result<(), String>;
}
