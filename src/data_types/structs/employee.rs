use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
}
