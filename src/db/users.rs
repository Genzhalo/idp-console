use crate::app::{
    entities::user::{User, UserToken},
    traits::repositories::user::TUserRepositories,
};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::time::SystemTime;
use tokio_postgres::Row;

pub struct UserRepository {
    pool: Pool,
}

impl User {
    fn from_row(row: Row) -> Self {
        User {
            id: row.get::<&str, String>("id"),
            email: row.get::<&str, String>("email"),
            password_alg: row.get::<&str, String>("password_alg"),
            password_hash: row.get::<&str, String>("password_hash"),
            created_at: row.get::<&str, SystemTime>("created_at").into(),
        }
    }
}

impl UserToken {
    fn from_row(row: &Row) -> Self {
        UserToken {
            token: row.get::<&str, String>("token"),
            used_for: row.get::<&str, String>("type"),
        }
    }
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TUserRepositories for UserRepository {
    async fn insert(&self, email: &str, p_hash: &str, p_alg: &str) -> Result<String, String> {
        let statement = "
            INSERT INTO users (password_alg, password_hash, email) 
            VALUES ($1, $2, $3) RETURNING *
        ";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(statement, &[&p_alg, &p_hash, &email])
            .await;

        match res {
            Ok(row) => Ok(row.get::<&str, String>("id")),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        let statement = "SELECT * FROM users AS u WHERE lower(u.email) = lower($1)";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(statement, &[&email])
            .await;
        match res {
            Ok(row) => Some(User::from_row(row)),
            Err(_) => None,
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        let statement = "SELECT * FROM users AS u WHERE u.id = $1";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(statement, &[&id])
            .await;
        match res {
            Ok(row) => Some(User::from_row(row)),
            Err(_) => None,
        }
    }

    async fn upsert_user_token(
        &self,
        user_id: &str,
        token: &str,
        used_for: &str,
    ) -> Result<bool, String> {
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .execute("
                    WITH upsert AS (UPDATE user_tokens SET token = $2 WHERE user_id = $1 AND type = $3 RETURNING *)
                    INSERT INTO user_tokens (user_id, token, type) SELECT $1,$2,$3 WHERE NOT EXISTS (SELECT * FROM upsert);
            ", &[&user_id, &token, &used_for],
            )
            .await;

        match res {
            Ok(row) => Ok(row != 0),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn remove_user_tokens(&self, user_id: &str, tokens: Vec<&str>) -> Result<(), String> {
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .execute(
                "DELETE FROM user_tokens WHERE user_id = $1 AND token = any($2);",
                &[&user_id, &tokens],
            )
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }

    async fn find_tokens(&self, user_id: &str) -> Vec<UserToken> {
        let statement = "
            SELECT t.* FROM users AS u 
            INNER JOIN user_tokens AS t ON u.id = t.user_id 
            WHERE u.id = $1;
        ";

        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query(statement, &[&user_id])
            .await;

        match res {
            Ok(rows) => rows.iter().map(|data| UserToken::from_row(data)).collect(),
            Err(_err) => vec![],
        }
    }
}
