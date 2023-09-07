use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ProductFeature {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub image_link: String,
    pub video_link: String,
    pub icon: String,
    pub quote: String,
    pub quote_author: String,
    pub quote_author_position: String,
    pub order_number: i32,
}
