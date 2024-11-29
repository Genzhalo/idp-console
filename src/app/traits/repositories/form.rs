use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::app::entities::form::Form;
#[async_trait]
pub trait TFormRepositories {
    async fn insert(
        &self,
        name: &str,
        form_limit: i32,
        status: &str,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        time_frame_duration: i32,
        exclude_form_ids: Vec<String>,
    ) -> Result<String, String>;
    async fn find(&self) -> Vec<Form>;
    async fn find_by_id(&self, id: &str) -> Option<Form>;
    async fn update(
        &self,
        id: &str,
        name: Option<String>,
        form_limit: Option<i32>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        time_frame_duration: Option<i32>,
        status: Option<String>,
        exclude_form_ids: Option<Vec<String>>,
    ) -> Result<(), String>;

    async fn delete(&self, id: &str) -> Result<(), String>;
}
