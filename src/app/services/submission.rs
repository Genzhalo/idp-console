use std::str::FromStr;

use serde::Deserialize;

use crate::app::{
    config::Config,
    entities::submission::{status::SubmissionStatus, Submission},
    errors::BaseError,
    traits::repositories::{
        form::TFormRepositories, respondent::TRespondentRepositories,
        submission::TSubmissionRepositories, user::TUserRepositories,
    },
    utils::arrival_date::calculate_arrival_date,
};

use super::{form::FormService, respondent::RespondentService, user::UserService};

#[derive(Debug, Deserialize)]
pub struct GetQuery {
    pub form_id: Option<String>,
    pub respondent_id: Option<String>,
}

pub struct SubmissionService<'a> {
    sub_rep: &'a (dyn TSubmissionRepositories + Send + Sync),
    user_service: UserService<'a>,
    form_service: FormService<'a>,
    respondent_service: RespondentService<'a>,
}

impl<'a> SubmissionService<'a> {
    pub fn new(
        config: &'a Config,
        sub_rep: &'a (dyn TSubmissionRepositories + Send + Sync),
        user_rep: &'a (dyn TUserRepositories + Send + Sync),
        form_rep: &'a (dyn TFormRepositories + Send + Sync),
        resp_rep: &'a (dyn TRespondentRepositories + Send + Sync),
        token: &'a str,
    ) -> Self {
        Self {
            sub_rep,
            respondent_service: RespondentService::new(config, resp_rep, user_rep, token),
            user_service: UserService::new(config, user_rep, token),
            form_service: FormService::new(&config, form_rep, user_rep, &token),
        }
    }

    pub async fn create(&self, form_id: &str, respondent_id: &str) -> Result<String, BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let form = match self.form_service.get_by_id(&form_id).await {
            Ok(form) => form,
            Err(err) => return Err(err),
        };

        let _ = match self.respondent_service.get_by_id(&respondent_id).await {
            Ok(form) => form,
            Err(err) => return Err(err),
        };

        let submissions = self.sub_rep.find(Some(form_id.to_string()), None).await;
        let respondent_sub = submissions
            .iter()
            .find(|s| s.respondent.id == respondent_id);

        if respondent_sub.is_some() {
            return Err(BaseError::new(
                "This respondent already have submission".to_string(),
            ));
        }

        let sub_order = submissions.len() + 1;

        if form.limit < sub_order as u16 {
            return Err(BaseError::new("Forbidden".to_string()));
        }

        let arrival_date = calculate_arrival_date(&form, sub_order as u16);

        let insert_result = self
            .sub_rep
            .insert(
                &form_id,
                &respondent_id,
                arrival_date.naive_utc(),
                sub_order as i32,
                &SubmissionStatus::Received.to_string(),
            )
            .await;

        match insert_result {
            Ok(id) => Ok(id),
            Err(err) => Err(BaseError::new(err)),
        }
    }

    pub async fn delete(&self, id: &str) -> Result<(), BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        match self.sub_rep.delete(&id).await {
            Ok(_) => Ok(()),
            Err(err) => return Err(BaseError::new(err)),
        }
    }

    pub async fn status(&self, id: &str, status: &str) -> Result<(), BaseError> {
        let sub_status = match SubmissionStatus::from_str(&status) {
            Ok(value) => value,
            Err(_) => return Err(BaseError::new("Status is not valid".to_string())),
        };

        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let submission = match self.sub_rep.find_by_id(&id).await {
            Some(sub) => sub,
            None => return Err(BaseError::new("Submission not found".to_string())),
        };

        if submission.status == sub_status {
            return Ok(());
        }

        match self.sub_rep.update(id, &Some(status.to_string())).await {
            Ok(_) => Ok(()),
            Err(err) => return Err(BaseError::new(err)),
        }
    }

    pub async fn get(&self, query: GetQuery) -> Result<Vec<Submission>, BaseError> {
        let _ = match self.user_service.get_current_user().await {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        Ok(self.sub_rep.find(query.form_id, query.respondent_id).await)
    }
}
