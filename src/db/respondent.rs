use async_trait::async_trait;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;

use crate::app::{
    entities::respondent::Respondent, traits::repositories::respondent::TRespondentRepositories,
};

pub struct RespondentRepository {
    pool: Pool,
}

impl RespondentRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TRespondentRepositories for RespondentRepository {
    async fn insert(
        &self,
        first_name: &str,
        last_name: &str,
        passport_id: &str,
        phone: &str,
        region: &str,
        children: i16,
        idp_code: &Option<String>,
    ) -> Result<String, String> {
        let statement ="
            INSERT INTO respondents (first_name, last_name, passport_id, phone, region, children, idp_code) 
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
                    &first_name,
                    &last_name,
                    &passport_id,
                    &phone,
                    &region,
                    &children,
                    &idp_code.as_deref(),
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
    async fn find(&self, by_name: Option<String>, by_passport: Option<String>) -> Vec<Respondent> {
        let mut r#where = String::new();
        let mut conditions: Vec<String> = vec![];
        let mut fields: Vec<&(dyn ToSql + Sync)> = vec![];

        if let Some(ref value) = by_name {
            conditions.push(format!(
                "first_name LIKE '{}%' OR last_name LIKE '{}%'",
                value, value
            ));
        }

        if let Some(ref id) = by_passport {
            fields.push(id);
            conditions.push(format!("passport_id = ${}", fields.len()));
        }

        if conditions.len() > 0 {
            r#where = format!("WHERE {}", conditions.join(" AND "))
        }

        let statement = format!("SELECT * FROM respondents {}", r#where).to_string();
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query(&statement, &fields)
            .await;
        match res {
            Ok(rows) => rows.iter().map(|row| Respondent::from_row(row)).collect(),
            Err(_err) => vec![],
        }
    }

    async fn find_by_id(&self, id: &str) -> Option<Respondent> {
        let statement = "SELECT * FROM respondents WHERE id = $1;";
        let res = self
            .pool
            .get()
            .await
            .unwrap()
            .query_one(statement, &[&id])
            .await;
        match res {
            Ok(row) => Some(Respondent::from_row(&row)),
            Err(_err) => None,
        }
    }

    async fn update(
        &self,
        id: &str,
        first_name: &Option<String>,
        last_name: &Option<String>,
        passport_id: &Option<String>,
        phone: &Option<String>,
        region: &Option<String>,
        children: &Option<i16>,
        idp_code: &Option<String>,
    ) -> Result<(), String> {
        let mut set: Vec<String> = vec![];
        let mut fields: Vec<&(dyn ToSql + Sync)> = vec![&id];

        if let Some(ref value) = passport_id {
            fields.push(value);
            set.push(format!("passport_id = ${}", fields.len()));
        }

        if let Some(ref value) = idp_code {
            fields.push(value);
            set.push(format!("idp_code = ${}", fields.len()));
        };

        if let Some(ref value) = first_name {
            fields.push(value);
            set.push(format!("first_name = ${}", fields.len()));
        }
        if let Some(ref value) = last_name {
            fields.push(value);
            set.push(format!("last_name = ${}", fields.len()));
        }
        if let Some(ref value) = phone {
            fields.push(value);
            set.push(format!("phone = ${}", fields.len()));
        }

        if let Some(ref value) = region {
            fields.push(value);
            set.push(format!("region = ${}", fields.len()));
        }

        if let Some(ref value) = children {
            fields.push(value);
            set.push(format!("children = ${}", fields.len()));
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
                &format!("UPDATE respondents SET {} WHERE id = $1", set.join(",")),
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
            .execute("DELETE FROM respondents WHERE id = $1", &[&id])
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
