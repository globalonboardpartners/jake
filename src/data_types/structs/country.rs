use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
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
    pub tags: Option<JsonValue>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

