use crate::app::entities::form::Form;
use chrono::{DateTime, Utc};

pub fn calculate_arrival_date(form: &Form, order: u16) -> DateTime<Utc> {
    let index =
        form.end_date.timestamp() - form.start_date.timestamp() - form.time_frame_duration as i64;
    let t = order as f64 / form.limit as f64;
    let exact_secs = (t * index as f64 + form.start_date.timestamp() as f64) as i64 - 1;
    let secs = exact_secs - (exact_secs % form.time_frame_duration as i64);
    DateTime::from_timestamp(secs, 0).unwrap()
}
