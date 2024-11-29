use crate::app::{config::Config, entities::user::User};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaimType {
    Refresh,
    Login,
}

impl fmt::Display for ClaimType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClaimType::Refresh => write!(f, "Refresh"),
            ClaimType::Login => write!(f, "Login"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub claim_type: String,
    pub exp: usize,
    pub iat: SystemTime,
}

pub struct JWT {
    secret: String,
}

impl JWT {
    pub fn new(config: &Config) -> Self {
        Self {
            secret: config.jwt_secret_key.clone(),
        }
    }

    pub fn login(&self, user: &User) -> Result<String, String> {
        let claims = Claims {
            sub: user.id.to_owned(),
            claim_type: ClaimType::Login.to_string(),
            exp: self.get_expiration(7),
            iat: SystemTime::now(),
        };
        self.create(&claims)
    }

    pub fn parse(&self, token: &str, claim_type: Option<ClaimType>) -> Result<Claims, String> {
        let token_message = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        let claims = match token_message {
            Ok(data) => data.claims,
            Err(err) => return Err(err.to_string()),
        };

        if claim_type.is_none() || claims.claim_type == claim_type.unwrap().to_string() {
            Ok(claims)
        } else {
            Err("Token is not valid".to_string())
        }
    }

    fn create(&self, claims: &Claims) -> Result<String, String> {
        let token_res = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );

        match token_res {
            Ok(token) => Ok(token),
            Err(err) => Err(err.to_string()),
        }
    }

    fn get_expiration(&self, days: u8) -> usize {
        Utc::now()
            .checked_add_signed(chrono::Duration::days(days as i64))
            .expect("valid timestamp")
            .timestamp() as usize
    }
}
