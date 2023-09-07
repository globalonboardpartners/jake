use serde::{Serialize, Deserialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub category_id: i32,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub featured: bool,
    pub publish_date: Option<NaiveDateTime>,
}
