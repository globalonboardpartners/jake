use serde::{Serialize, Deserialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub publish_date: Option<NaiveDateTime>,
}
