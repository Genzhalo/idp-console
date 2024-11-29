use std::{str::FromStr, time::SystemTime};

use tokio_postgres::Row;

use crate::app::entities::{
    form::{status::FormStatus, Form},
    respondent::Respondent,
    submission::{status::SubmissionStatus, Submission},
};

impl Submission {
    pub fn from_row(row: &Row) -> Self {
        Submission {
            id: row.get::<&str, String>("id"),
            sub_order: row.get::<&str, i32>("sub_order") as u32,
            status: SubmissionStatus::from_str(row.get::<&str, String>("status").as_str()).unwrap(),
            arrival_date: row.get::<&str, SystemTime>("arrival_date").into(),
            created_at: row.get::<&str, SystemTime>("created_at").into(),
            form: Form {
                id: row.get::<&str, String>("form_id"),
                name: row.get::<&str, String>("form_name"),
                limit: row.get::<&str, i32>("form_limit") as u16,
                status: FormStatus::from_str(row.get::<&str, String>("form_status").as_str())
                    .unwrap(),
                start_date: row
                    .get::<&str, SystemTime>("form_scheduled_start_date")
                    .into(),
                end_date: row
                    .get::<&str, SystemTime>("form_scheduled_end_date")
                    .into(),
                created_at: row.get::<&str, SystemTime>("form_created_at").into(),
                time_frame_duration: row.get::<&str, i32>("form_time_frame_duration") as u16,
                exclude_form_ids: row.get::<&str, Vec<String>>("form_exclude_form_ids"),
            },
            respondent: Respondent {
                id: row.get::<&str, String>("res_id"),
                passport_id: row.get::<&str, String>("res_passport_id"),
                first_name: row.get::<&str, String>("res_first_name"),
                last_name: row.get::<&str, String>("res_last_name"),
                phone: row.get::<&str, String>("res_phone"),
                region: row.get::<&str, String>("res_region"),
                children: row.get::<&str, i16>("res_children") as u8,
                idp_code: row.get::<&str, Option<String>>("res_idp_code"),
                created_at: row.get::<&str, SystemTime>("res_created_at").into(),
            },
        }
    }
}

impl Respondent {
    pub fn from_row(row: &Row) -> Self {
        Respondent {
            id: row.get::<&str, String>("id"),
            passport_id: row.get::<&str, String>("passport_id"),
            first_name: row.get::<&str, String>("first_name"),
            last_name: row.get::<&str, String>("last_name"),
            phone: row.get::<&str, String>("phone"),
            region: row.get::<&str, String>("region"),
            children: row.get::<&str, i16>("children") as u8,
            idp_code: row.get::<&str, Option<String>>("idp_code"),
            created_at: row.get::<&str, SystemTime>("created_at").into(),
        }
    }
}

impl Form {
    pub fn from_row(row: &Row) -> Self {
        Form {
            id: row.get::<&str, String>("id"),
            name: row.get::<&str, String>("name"),
            limit: row.get::<&str, i32>("form_limit") as u16,
            status: FormStatus::from_str(row.get::<&str, String>("status").as_str()).unwrap(),
            start_date: row.get::<&str, SystemTime>("scheduled_start_date").into(),
            end_date: row.get::<&str, SystemTime>("scheduled_end_date").into(),
            created_at: row.get::<&str, SystemTime>("created_at").into(),
            time_frame_duration: row.get::<&str, i32>("time_frame_duration") as u16,
            exclude_form_ids: row.get::<&str, Vec<String>>("exclude_form_ids"),
        }
    }
}
