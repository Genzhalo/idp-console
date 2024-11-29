use async_trait::async_trait;
use chrono::NaiveDateTime;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;

use crate::app::{entities::form::Form, traits::repositories::form::TFormRepositories};

pub struct FormRepository {
    pool: Pool,
}

impl FormRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TFormRepositories for FormRepository {
    async fn insert(
        &self,
        name: &str,
        form_limit: i32,
        status: &str,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        time_frame_duration: i32,
        exclude_form_ids: Vec<String>,
    ) -> Result<String, String> {
        let statement ="
            INSERT INTO forms (name, form_limit, status, scheduled_start_date, scheduled_end_date, time_frame_duration, exclude_form_ids) 
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *
        ";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(
                statement,
                &[
                    &name,
                    &form_limit,
                    &status,
                    &start_date,
                    &end_date,
                    &time_frame_duration,
                    &exclude_form_ids,
                ],
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
    async fn find(&self) -> Vec<Form> {
        let statement = "SELECT * FROM forms;";
        let res = self.pool.get().await.unwrap().query(statement, &[]).await;
        match res {
            Ok(rows) => rows.iter().map(|row| Form::from_row(row)).collect(),
            Err(_err) => vec![],
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<Form> {
        let statement = "SELECT * FROM forms WHERE id = $1;";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(statement, &[&id])
            .await;
        match res {
            Ok(row) => Some(Form::from_row(&row)),
            Err(_err) => None,
        }
    }

    async fn update(
        &self,
        id: &str,
        name: Option<String>,
        form_limit: Option<i32>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        time_frame_duration: Option<i32>,
        status: Option<String>,
        exclude_form_ids: Option<Vec<String>>,
    ) -> Result<(), String> {
        let mut set: Vec<String> = vec![];
        let mut fields: Vec<&(dyn ToSql + Sync)> = vec![&id];

        if let Some(ref name) = name {
            fields.push(name);
            set.push(format!("name = ${}", fields.len()));
        }

        if let Some(ref limit) = form_limit {
            fields.push(limit);
            set.push(format!("form_limit = ${}", fields.len()));
        };

        if let Some(ref duration) = time_frame_duration {
            fields.push(duration);
            set.push(format!("time_frame_duration = ${}", fields.len()));
        }
        if let Some(ref start) = start_date {
            fields.push(start);
            set.push(format!("scheduled_start_date = ${}", fields.len()));
        }
        if let Some(ref end) = end_date {
            fields.push(end);
            set.push(format!("scheduled_end_date = ${}", fields.len()));
        }

        if let Some(ref status) = status {
            fields.push(status);
            set.push(format!("status = ${}", fields.len()));
        }

        if let Some(ref ids) = exclude_form_ids {
            fields.push(ids);
            set.push(format!("exclude_form_ids = ${}", fields.len()));
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
                &format!("UPDATE forms SET {} WHERE id = $1", set.join(",")),
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
            .execute("DELETE FROM forms WHERE id = $1", &[&id])
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
