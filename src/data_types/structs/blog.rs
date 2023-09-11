use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub category_id: Option<i32>,
    pub category: Option<String>,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub featured: bool,
    pub created: Option<NaiveDateTime>,
}
