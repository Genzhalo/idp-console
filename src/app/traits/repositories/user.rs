use async_trait::async_trait;

use crate::app::entities::user::{User, UserToken};
#[async_trait]
pub trait TUserRepositories {
    async fn insert(&self, email: &str, p_hash: &str, p_alg: &str) -> Result<String, String>;
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn find_by_id(&self, id: &str) -> Option<User>;
    async fn upsert_user_token(
        &self,
        user_id: &str,
        token: &str,
        used_for: &str,
    ) -> Result<bool, String>;

    async fn remove_user_tokens(&self, user_id: &str, tokens: Vec<&str>) -> Result<(), String>;

    async fn find_tokens(&self, user_id: &str) -> Vec<UserToken>;
}
