use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ProductFeature {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
}
