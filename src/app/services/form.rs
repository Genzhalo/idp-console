use crate::app::{
    config::Config,
    entities::form::{status::FormStatus, Form},
    errors::BaseError,
    traits::repositories::{form::TFormRepositories, user::TUserRepositories},
    utils::validate::{validate, validate_date_not_past},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

use super::user::UserService;

#[derive(Debug, Validate, Deserialize)]
pub struct CreateFromData {
    #[validate(length(min = 1, message = "The first name length should be min 1 symbols"))]
    name: String,
    #[validate(range(min = 0))]
    limit: u16,
    #[validate(range(min = 0))]
    #[serde(rename = "timeFrameDuration")]
    pub time_frame_duration: u16,
    #[validate(custom(function = validate_date_not_past, message = "Date is not valid"))]
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    #[validate(custom(function = validate_date_not_past, message = "Date is not valid"))]
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
    #[serde(rename = "excludeFormIds")]
    pub exclude_form_ids: Vec<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateFromData {
    name: Option<String>,
    limit: Option<u16>,
    #[serde(rename = "timeFrameDuration")]
    pub time_frame_duration: Option<u16>,
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(rename = "excludeFormIds")]
    pub exclude_form_ids: Option<Vec<String>>,
}

pub struct FormService<'a> {
    form_repo: &'a (dyn TFormRepositories + Send + Sync),
    user_service: UserService<'a>,
}

impl<'a> FormService<'a> {
    pub fn new(
        config: &'a Config,
        from_repo: &'a (dyn TFormRepositories + Send + Sync),
        user_repo: &'a (dyn TUserRepositories + Send + Sync),
        token: &'a str,
    ) -> Self {
        Self {
            form_repo: from_repo,
            user_service: UserService::new(config, user_repo, token),
        }
    }

    pub async fn create(self, data: CreateFromData) -> Result<String, BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let status = FormStatus::Draft.to_string();

        let result = self.form_repo.insert(
            &data.name,
            data.limit as i32,
            &status,
            data.start_date.naive_utc(),
            data.end_date.naive_utc(),
            data.time_frame_duration as i32,
            data.exclude_form_ids,
        );

        match result.await {
            Ok(id) => Ok(id),
            Err(e) => Err(BaseError::new(e)),
        }
    }

    pub async fn update(self, id: String, data: UpdateFromData) -> Result<(), BaseError> {
        match validate(&data) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let form = match self.get_by_id(&id).await {
            Ok(form) => form,
            Err(err) => return Err(err),
        };

        if form.status != FormStatus::Draft {
            return Err(BaseError::new("Forbidden".to_owned()));
        }

        let result = self.form_repo.update(
            &id,
            data.name,
            data.limit.map(|x| x as i32),
            data.start_date.map(|x| x.naive_utc()),
            data.end_date.map(|x| x.naive_utc()),
            data.time_frame_duration.map(|x| x as i32),
            None,
            data.exclude_form_ids,
        );
        match result.await {
            Ok(()) => Ok(()),
            Err(err) => Err(BaseError::new(err.to_string())),
        }
    }

    pub async fn delete(&self, id: &str) -> Result<(), BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let form = match self.get_by_id(&id).await {
            Ok(form) => form,
            Err(err) => return Err(err),
        };

        if form.status != FormStatus::Draft {
            return Err(BaseError::new("Forbidden".to_owned()));
        }

        match self.form_repo.delete(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err.to_string())),
        }
    }

    pub async fn status(&self, id: String, value: FormStatus) -> Result<(), BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let form = match self.get_by_id(&id).await {
            Ok(form) => form,
            Err(err) => return Err(err),
        };

        if form.status == value {
            return Ok(());
        }

        if form.status == FormStatus::Draft && value != FormStatus::Open {
            return Err(BaseError::new("Forbidden".to_owned()));
        }

        if form.status == FormStatus::Open && value != FormStatus::Close {
            return Err(BaseError::new("Forbidden".to_owned()));
        }

        match self
            .form_repo
            .update(
                &id,
                None,
                None,
                None,
                None,
                None,
                Some(value.to_string()),
                None,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(BaseError::new(err.to_string())),
        }
    }

    pub async fn get(&self) -> Result<Vec<Form>, BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };
        Ok(self.form_repo.find().await)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Form, BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };
        match self.form_repo.find_by_id(&id).await {
            Some(form) => Ok(form),
            None => Err(BaseError::new("Form not foound".to_string())),
        }
    }
}
