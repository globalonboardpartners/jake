use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HotelRoom {
    pub id: Option<i32>,
    pub name: String,
    pub hotel_id: i32,
    pub description_short: String,
    pub description_long: String,
    pub video_link: Option<String>,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub tags: Option<String>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

