use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "hotel_category")]
pub enum HotelCategory {
    FamilyHotelsAndResorts,
    BeachAndVacationResorts,
    HolidayCondoResorts,
    BoutiqueHotelProperties,
    LuxuryAndHighEndHotels,
    HotelsOnPrivateIslands,
    HotelsThatFloat,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "room_category")]
pub enum RoomCategory {
    FamilyRooms,
    BeachAndVacationRooms,
    HolidayRooms,
    BoutiqueRooms,
    LuxuryAndHighEndRooms,
    RoomsOnPrivateIslands,
    RoomsThatFloat,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hotel {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub hotel_category: Option<HotelCategory>,
    pub description_short: String,
    pub description_long: String,
    pub video_link: Option<String>,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub featured_hotel: f64,
    pub partner_vendor: i32,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub postal_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
    pub website_link: String,
    pub tags: Option<JsonValue>,
    pub created: Option<String>,
    pub edited: Option<String>,
    pub hotel_room: Option<JsonValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotelRoom {
    pub id: Option<i32>,
    pub hotel_id: i32,
    pub name: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: Option<String>,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub price: Option<f64>,
    pub gallery: Vec<String>,
    pub amenities: Vec<String>,
    pub inclusions: Vec<String>,
    pub guest_services: Vec<String>,
    pub room_category: Option<RoomCategory>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

