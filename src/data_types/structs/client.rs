use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub title: String,
    pub description_short: Option<String>,
    pub description_long: Option<String>,
    pub logo: Option<String>,
    pub image_link: Option<String>,
    pub quote: Option<String>,
    pub quote_author: Option<String>,
    pub quote_author_position: Option<String>,
    pub number_of_employees: Option<String>,
    pub industry: Option<String>,
    pub website_link: Option<String>,
    pub features_used: Option<String>,
    pub featured: Option<bool>,
    pub publish_date: Option<NaiveDateTime>,
}
