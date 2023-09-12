use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventDetails {
    pub id: Option<i32>,
    pub name: String,
    pub slug: String,
    pub event_date: Option<NaiveDate>,
    pub event_artist_slug: String,
    pub venue_name: String,
    pub event_time: Option<NaiveTime>,
    pub continent: i32,
    pub country: i32,
    pub region: i32,
    pub city: i32,
    pub ticket_link: String,
    pub gallery: Vec<String>,
    pub tags: Option<String>,
    pub created: Option<String>,
    pub edited: Option<String>,
}
