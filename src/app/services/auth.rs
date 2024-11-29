use crate::app::{
    config::Config,
    errors::BaseError,
    traits::repositories::user::TUserRepositories,
    utils::{
        hash::{hash_pwd, verify_pwd},
        jwt::JWT,
        validate::validate,
    },
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateInputData {
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password is invalid"))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginInputData {
    #[validate(email(message = "Email is invalid"))]
    email: String,
    #[validate(length(min = 6, message = "Password is invalid"))]
    password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct EmailInputData {
    #[validate(email(message = "Email is invalid"))]
    email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct PasswordInputData {
    #[validate(length(min = 6, message = "Password is invalid"))]
    password: String,
}

pub struct AuthService<'a> {
    config: &'a Config,
    user_rep: &'a (dyn TUserRepositories + Send + Sync),
}

impl<'a> AuthService<'a> {
    pub fn new(config: &'a Config, user_rep: &'a (dyn TUserRepositories + Send + Sync)) -> Self {
        Self { user_rep, config }
    }

    pub async fn create(&self, signup_data: CreateInputData) -> Result<String, BaseError> {
        match validate(&signup_data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let user = self.user_rep.find_by_email(&signup_data.email).await;

        if user.is_some() {
            return Err(BaseError::new("The email already using".to_string()));
        }

        let (password_alg, password_hash) = match hash_pwd(&signup_data.password) {
            Ok(res) => res,
            Err(e) => return Err(BaseError::new(e)),
        };

        let result = self
            .user_rep
            .insert(&signup_data.email, &password_hash, &password_alg);

        match result.await {
            Ok(id) => Ok(id),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn login(&self, data: LoginInputData) -> Result<String, BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let user_result = self.user_rep.find_by_email(&data.email).await;

        let user = match user_result {
            Some(user) => user,
            None => return Err(BaseError::new("User not found".to_string())),
        };

        if !verify_pwd(&user.password_hash, &data.password) {
            return Err(BaseError::new("Password is incorrect".to_string()));
        }

        let access_token = match JWT::new(&self.config).login(&user) {
            Ok(token) => token,
            Err(err) => return Err(BaseError::new(err)),
        };

        let res = self
            .user_rep
            .upsert_user_token(&user.id, &access_token, "WEB");

        match res.await {
            Ok(_) => Ok(access_token),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), BaseError> {
        let user_id = match JWT::new(&self.config).parse(token, None) {
            Ok(claim) => claim.sub,
            Err(e) => return Err(BaseError::new(e)),
        };

        match self
            .user_rep
            .remove_user_tokens(&user_id, vec![token])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => return Err(BaseError::new(e.to_string())),
        }
    }
}
