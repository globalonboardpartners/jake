use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub location: String,
    pub employment_basis: String,
    pub publish_date: Option<NaiveDateTime>,
}
