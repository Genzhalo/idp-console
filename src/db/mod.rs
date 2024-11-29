use deadpool_postgres::{Config, ManagerConfig, Object, RecyclingMethod, Runtime};
use std::fs;
use tokio_postgres::NoTls;

use crate::app::{
    self,
    services::auth::{AuthService, CreateInputData},
    traits::repositories::{
        form::TFormRepositories, respondent::TRespondentRepositories,
        submission::TSubmissionRepositories, user::TUserRepositories,
    },
};

use self::{
    forms::FormRepository, respondent::RespondentRepository, submissions::SubmissionsRepository,
    users::UserRepository,
};
mod forms;
mod from_row;
mod respondent;
mod submissions;
mod users;

pub struct DB {
    pub users: Box<dyn TUserRepositories + Sync + Send>,
    pub forms: Box<dyn TFormRepositories + Sync + Send>,
    pub respondents: Box<dyn TRespondentRepositories + Sync + Send>,
    pub submissions: Box<dyn TSubmissionRepositories + Sync + Send>,
}

impl DB {
    async fn run_migration(client: &Object) {
        if let Ok(path) = std::env::var("DATABASE_SCHEMA_FILE_PATH") {
            if let Ok(migration) = fs::read_to_string(path) {
                if let Err(err) = client.batch_execute(&migration).await {
                    eprintln!("Failed to initialize DB: {}", err);
                }
            } else {
                eprintln!("Failed to read migration file");
            }
        } else {
            eprintln!("DATABASE_SCHEMA_FILE_PATH environment variable not set");
        }
    }

    pub async fn init_default_user(&self, config: &app::config::Config) {
        if let Ok(email) = std::env::var("DEFAULT_USER_EMAIL") {
            if let Ok(password) = std::env::var("DEFAULT_USER_PASSWORD") {
                match self.users.find_by_email(&email).await {
                    Some(_) => return,
                    None => {
                        let service = AuthService::new(&config, self.users.as_ref());
                        let _ = service.create(CreateInputData { email, password }).await;
                    }
                }
            }
        }
    }

    pub async fn connect() -> Self {
        let url = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

        let mut cfg = Config::new();
        cfg.url = Some(url);

        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        let client = pool.get().await.unwrap();

        DB::run_migration(&client).await;

        DB {
            users: Box::new(UserRepository::new(pool.clone())),
            forms: Box::new(FormRepository::new(pool.clone())),
            respondents: Box::new(RespondentRepository::new(pool.clone())),
            submissions: Box::new(SubmissionsRepository::new(pool.clone())),
        }
    }
}
