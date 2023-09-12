use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PartnerVendor {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub video_link: Option<String>,
    pub image_link: String,
    pub image_link_2: String,
    pub thumbnail_link: String,
    pub gallery: Vec<String>,
    pub featured_partner_vendor: f64,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
    pub website_link: String,
    pub tags: Option<String>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

