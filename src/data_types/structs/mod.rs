use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub mod blog_category;
pub use self::blog_category::BlogCategory;

pub mod blog;
pub use self::blog::Blog;

pub mod employee;
pub use self::employee::Employee;

pub mod job_listing;
pub use self::job_listing::JobListing;

pub mod product_feature;
pub use self::product_feature::ProductFeature;

pub mod continent;
pub use self::continent::Continent;

pub mod client;
pub use self::client::Client;

pub mod error_message;
pub use self::error_message::ErrorMessage;

pub mod country;
pub use self::country::Country;

pub mod region;
pub use self::region::Region;

pub mod city;
pub use self::city::City;

pub mod partner_vendor;
pub use self::partner_vendor::PartnerVendor;

pub mod restaurant;
pub use self::restaurant::Restaurant;

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: Option<i32>,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(id) = self.id {
            return write!(f, "Id: {}", id);
        }
        write!(f, "Id: {}", "None")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hotel {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: String,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub featured_hotel: f32,
    pub partner_vendor: i32,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub website_link: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HotelRoom {
    pub id: i32,
    pub name: String,
    pub hotel_id: i32,
    pub description_short: String,
    pub description_long: String,
    pub video_link: String,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: String,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub featured_activity: f32,
    pub partner_vendor: i32,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub website_link: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: String,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub featured_event: f32,
    pub ticket_link: String,
    pub partner_vendor: i32,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub website_link: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventDetails {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub event_date: SystemTime,
    pub event_artist_slug: String,
    pub venue_name: String,
    pub event_time: SystemTime,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub ticket_link: String,
    pub gallery: Vec<String>,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}
