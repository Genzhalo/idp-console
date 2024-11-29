use async_trait::async_trait;
use chrono::NaiveDateTime;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;

use crate::app::{
    entities::submission::Submission, traits::repositories::submission::TSubmissionRepositories,
};

pub struct SubmissionsRepository {
    pool: Pool,
}

impl SubmissionsRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TSubmissionRepositories for SubmissionsRepository {
    async fn insert(
        &self,
        form_id: &str,
        respondent_id: &str,
        arrival_date: NaiveDateTime,
        sub_order: i32,
        status: &str,
    ) -> Result<String, String> {
        let statement = "
            INSERT INTO submissions (form_id, respondent_id, arrival_date, sub_order, status) 
            VALUES ($1, $2, $3, $4, $5) RETURNING *
        ";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(
                statement,
                &[&form_id, &respondent_id, &arrival_date, &sub_order, &status],
            )
            .await;

        match res {
            Ok(row) => Ok(row.get::<&str, String>("id")),
            Err(err) => match err.as_db_error() {
                Some(err) => Err(err.message().to_string()),
                None => Err(err.to_string()),
            },
        }
    }
    async fn find(
        &self,
        by_form: Option<String>,
        by_respondent: Option<String>,
    ) -> Vec<Submission> {
        let mut r#where = String::new();
        let mut conditions: Vec<String> = vec![];
        let mut fields: Vec<&(dyn ToSql + Sync)> = vec![];

        if let Some(ref id) = by_form {
            fields.push(id);
            conditions.push(format!("sub.form_id = ${}", fields.len()));
        }

        if let Some(ref id) = by_respondent {
            fields.push(id);
            conditions.push(format!("sub.respondent_id = ${}", fields.len()));
        }

        if conditions.len() > 0 {
            r#where = format!("WHERE {}", conditions.join(" AND "))
        }

        let statement = format!(
            "
            SELECT sub.*,
                form.id AS form_id,
                form.created_at AS form_created_at,
                form.name AS form_name,
                form.form_limit AS form_limit,
                form.status AS form_status,
                form.scheduled_start_date AS form_scheduled_start_date,
                form.scheduled_end_date AS form_scheduled_end_date,
                form.time_frame_duration AS form_time_frame_duration,
                form.created_at AS form_created_at,
                form.exclude_form_ids AS form_exclude_form_ids,
                res.id AS res_id,
                res.created_at AS res_created_at,
                res.id AS res_id,
                res.passport_id AS res_passport_id,
                res.first_name AS res_first_name,
                res.last_name AS res_last_name,
                res.phone AS res_phone,
                res.region AS res_region,
                res.children AS res_children,
                res.idp_code AS res_idp_code,
                res.created_at AS res_created_at
            FROM submissions AS sub
            JOIN forms AS form ON form.id = sub.form_id
            JOIN respondents AS res ON res.id = sub.respondent_id
            {}
            ",
            r#where
        )
        .to_string();
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query(&statement, &fields)
            .await;
        match res {
            Ok(rows) => rows.iter().map(|row| Submission::from_row(row)).collect(),
            Err(_err) => vec![],
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<Submission> {
        let statement = "SELECT sub.*,
                form.id AS form_id,
                form.created_at AS form_created_at,
                form.name AS form_name,
                form.form_limit AS form_limit,
                form.status AS form_status,
                form.scheduled_start_date AS form_scheduled_start_date,
                form.scheduled_end_date AS form_scheduled_end_date,
                form.time_frame_duration AS form_time_frame_duration,
                form.created_at AS form_created_at,
                form.exclude_form_ids AS form_exclude_form_ids,
                res.id AS res_id,
                res.passport_id AS res_passport_id,
                res.first_name AS res_first_name,
                res.last_name AS res_last_name,
                res.phone AS res_phone,
                res.region AS res_region,
                res.children AS res_children,
                res.idp_code AS res_idp_code,
                res.created_at AS res_created_at
         FROM submissions AS sub 
            JOIN forms AS form ON form.id = sub.form_id
            JOIN respondents AS res ON res.id = sub.respondent_id
            WHERE sub.id = $1";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(statement, &[&id])
            .await;

        match res {
            Ok(row) => Some(Submission::from_row(&row)),
            Err(_err) => None,
        }
    }

    async fn update(&self, id: &str, status: &Option<String>) -> Result<(), String> {
        let mut set: Vec<String> = vec![];
        let mut fields: Vec<&(dyn ToSql + Sync)> = vec![&id];

        if let Some(ref value) = status {
            fields.push(value);
            set.push(format!("status = ${}", fields.len()));
        }

        if set.len() == 0 {
            return Ok(());
        }

        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .execute(
                &format!("UPDATE submissions SET {} WHERE id = $1", set.join(",")),
                &fields,
            )
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .execute("DELETE FROM submissions WHERE id = $1", &[&id])
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
