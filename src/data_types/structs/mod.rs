use serde::{Deserialize, Serialize};

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

pub mod hotel;
pub use self::hotel::Hotel;
pub use self::hotel::HotelRoom;

pub mod activity;
pub use self::activity::Activity;

pub mod event;
pub use self::event::Event;

pub mod event_details;
pub use self::event_details::EventDetails;

pub mod auth;
pub use self::auth::Auth;
pub use self::auth::Status;

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

