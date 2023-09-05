use serde::{Serialize, Deserialize};
use actix_web::web::Json;
use tokio_postgres::types::ToSql;
use crate::data_types::traits::PgPreparable;
use std::time::SystemTime;

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

impl PgPreparable for Continent {
    fn name() -> &'static str {
        "continent"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        Continent {
            id: row.get(0),
            name: row.get(1),
            slug: row.get(2),
            description_long: row.get(3),
            description_short: row.get(4),
            image_link: row.get(5),
            thumbnail_link: row.get(6),
            special_offer_image_link: row.get(7),
            video_link: row.get(8),
            gallery: row.get(9),
            tags: row.get(10),
            created: row.get(11),
            edited: row.get(12),
        }
    }

    fn columns() -> Vec<&'static str> {
       vec![
           "name",
           "slug",
           "description_long",
           "description_short",
           "image_link",
           "thumbnail_link",
           "special_offer_image_link",
           "video_link",
           "gallery",
           "tags",
           "created",
           "edited",
       ]
    }

    fn values(new_entity: &Json<Self>) -> Option<&'static [&'static (dyn ToSql + Sync + '_)]> where Self: std::marker::Sized + PgPreparable + Serialize {

        let name: &String = &new_entity.name;
        let slug: &String = &new_entity.slug;
        let description_long: &String = &new_entity.description_long;
        let description_short: &String = &new_entity.description_short;
        let image_link: &String = &new_entity.image_link;
        let thumbnail_link: &String = &new_entity.thumbnail_link;
        let special_offer_image_link: &String = &new_entity.special_offer_image_link;
        let video_link: &String = &new_entity.video_link;
        let gallery: &Vec<String> = &new_entity.gallery;
        let tags: &String = &new_entity.tags;
        let created: &SystemTime = &new_entity.created;
        let edited: &SystemTime = &new_entity.edited;

        Some(&[
            &name,
            &slug,
            &description_long,
            &description_short,
            &image_link,
            &thumbnail_link,
            &special_offer_image_link,
            &video_link,
            &gallery,
            &tags,
        ])
    }
}
