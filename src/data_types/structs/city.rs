use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: Option<String>,
    pub video_link: Option<String>,
    pub gallery: Vec<String>,
    pub featured_city: f64,
    pub getting_around: Vec<String>,
    pub food_and_drink: Vec<String>,
    pub facts: Vec<String>,
    pub hotel: Option<Vec<String>>,
    pub restaurant: Option<Vec<String>>,
    pub attraction: Option<Vec<String>>,
    pub shopping: Option<Vec<String>>,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub tags: Option<JsonValue>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

