use serde::Deserialize;

use crate::app::{
    config::Config,
    entities::respondent::Respondent,
    errors::BaseError,
    traits::repositories::{respondent::TRespondentRepositories, user::TUserRepositories},
    utils::validate::validate,
};

use self::{create_data::CreateData, update_data::UpdateData};

use super::user::UserService;
pub mod create_data;
pub mod update_data;

#[derive(Debug, Deserialize)]
pub struct MergeData {
    #[serde(rename = "formId")]
    from_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetQuery {
    name: Option<String>,
    #[serde(rename = "passportId")]
    passport_id: Option<String>,
}

pub struct RespondentService<'a> {
    respondent_repo: &'a (dyn TRespondentRepositories + Send + Sync),
    user_service: UserService<'a>,
}

impl<'a> RespondentService<'a> {
    pub fn new(
        config: &'a Config,
        respondent_repo: &'a (dyn TRespondentRepositories + Send + Sync),
        user_repo: &'a (dyn TUserRepositories + Send + Sync),
        token: &'a str,
    ) -> Self {
        Self {
            respondent_repo: respondent_repo,
            user_service: UserService::new(config, user_repo, token),
        }
    }

    pub async fn create(self, data: &CreateData) -> Result<String, BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let result = self.respondent_repo.insert(
            &data.first_name,
            &data.last_name,
            &data.passport_id,
            &data.phone,
            &data.region,
            data.children as i16,
            &data.idp_code,
        );

        match result.await {
            Ok(id) => Ok(id),
            Err(e) => Err(BaseError::new(e)),
        }
    }

    pub async fn update(self, id: String, data: &UpdateData) -> Result<(), BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let children = data.children.map(|v| v as i16);
        let result = self.respondent_repo.update(
            &id,
            &data.first_name,
            &data.last_name,
            &data.passport_id,
            &data.phone,
            &data.region,
            &children,
            &data.idp_code,
        );
        match result.await {
            Ok(()) => Ok(()),
            Err(err) => Err(BaseError::new(err.to_string())),
        }
    }

    pub async fn delete(&self, id: String) -> Result<(), BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.respondent_repo.delete(&id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err.to_string())),
        }
    }

    pub async fn get(&self, query: GetQuery) -> Result<Vec<Respondent>, BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        Ok(self
            .respondent_repo
            .find(query.name, query.passport_id)
            .await)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Respondent, BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };
        match self.respondent_repo.find_by_id(&id).await {
            Some(respondent) => Ok(respondent),
            None => Err(BaseError::new("Respondent not found".to_string())),
        }
    }

    pub async fn merge(&self, id: &str, data: &MergeData) -> Result<(), BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        if data.from_id == id {
            return Ok(());
        }

        let _ = match self.get_by_id(id).await {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let _ = match self.get_by_id(&data.from_id).await {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        Ok(())
    }
}
