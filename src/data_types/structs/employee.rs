use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
    pub twitter_link: Option<String>,
    pub linkedin_link: Option<String>,
    pub email: Option<String>,
    pub created: Option<String>,
    pub edited: Option<String>,
}
