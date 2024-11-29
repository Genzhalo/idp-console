use crate::app::{
    config::Config,
    entities::user::User,
    errors::BaseError,
    traits::repositories::user::TUserRepositories,
    utils::jwt::{ClaimType, JWT},
};

pub struct UserService<'a> {
    config: &'a Config,
    user_rep: &'a (dyn TUserRepositories + Send + Sync),
    token: &'a str,
}

impl<'a> UserService<'a> {
    pub fn new(
        config: &'a Config,
        user_rep: &'a (dyn TUserRepositories + Send + Sync),
        token: &'a str,
    ) -> Self {
        Self {
            config,
            user_rep,
            token,
        }
    }

    pub async fn get_current_user(&self) -> Result<User, BaseError> {
        let id = match self.id_from_token(self.token) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };
        let user = match self.user_rep.find_by_id(id.as_str()).await {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };
        let tokens = self.user_rep.find_tokens(&id).await;

        match tokens.iter().find(|t| t.token == self.token) {
            Some(_) => Ok(user),
            None => Err(BaseError::new("Token is expired".to_string())),
        }
    }

    fn id_from_token(&self, token: &str) -> Result<String, BaseError> {
        match JWT::new(&self.config).parse(token, Some(ClaimType::Login)) {
            Ok(claim) => Ok(claim.sub),
            Err(e) => return Err(BaseError::new(e)),
        }
    }
}
