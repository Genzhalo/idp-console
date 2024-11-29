use async_trait::async_trait;

use crate::app::entities::respondent::Respondent;

#[async_trait]
pub trait TRespondentRepositories {
    async fn insert(
        &self,
        first_name: &str,
        last_name: &str,
        passport_id: &str,
        phone: &str,
        region: &str,
        children: i16,
        idp_code: &Option<String>,
    ) -> Result<String, String>;
    async fn find(&self, by_name: Option<String>, by_passport: Option<String>) -> Vec<Respondent>;
    async fn find_by_id(&self, id: &str) -> Option<Respondent>;
    async fn delete(&self, id: &str) -> Result<(), String>;
    async fn update(
        &self,
        id: &str,
        first_name: &Option<String>,
        last_name: &Option<String>,
        passport_id: &Option<String>,
        region: &Option<String>,
        phone: &Option<String>,
        children: &Option<i16>,
        idp_code: &Option<String>,
    ) -> Result<(), String>;
}
