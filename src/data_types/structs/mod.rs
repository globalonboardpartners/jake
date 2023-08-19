use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEmployee {
    pub name: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogCategory {
    pub id: i32,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlogCategory {
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub category_id: i32,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub publish_date: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlog {
    pub title: String,
    pub slug: String,
    pub category_id: i32,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub publish_date: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub publish_date: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewJobListing {
    pub title: String,
    pub description: String,
    pub publish_date: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductFeature {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductFeature {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Return<T> {
    pub data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColumnValue {
    Integer(i32),
    Float(f64),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateColumn {
    pub id: Option<i32>,
    pub col_name: String,
    pub col_value: ColumnValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continent {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: Vec<String>,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewContinent {
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: Vec<String>,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCountry {
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRegion {
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: String,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: String,
    pub featured_city: f32,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCity {
    pub name: String,
    pub slug: String,
    pub description_long: String,
    pub description_short: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub special_offer_image_link: String,
    pub video_link: String,
    pub gallery: String,
    pub featured_city: f32,
    pub tags: String,
    pub created: SystemTime,
    pub edited: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartnerVendor {
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
    pub featured_partner_vendor: f32,
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
pub struct NewPartnerVendor {
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: String,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub featured_partner_vendor: f32,
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
pub struct Restaurant {
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
    pub menu_gallery: Vec<String>,
    pub featured_restaurant: f32,
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
pub struct NewRestaurant {
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: String,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub menu_gallery: Vec<String>,
    pub featured_restaurant: f32,
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
pub struct NewHotel {
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
pub struct NewHotelRoom {
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
pub struct NewActivity {
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
pub struct NewEvent {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEventDetails {
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
