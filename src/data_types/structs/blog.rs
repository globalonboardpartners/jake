use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use crate::utils::format_unix_timestamp;
use std::time::SystemTime;
use actix_web::web::Json;
use tokio_postgres::types::ToSql;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub category_id: i32,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub featured: bool,
    pub publish_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlog {
    pub title: String,
    pub slug: String,
    pub category_id: i32,
    pub content: String,
    pub image_link: String,
    pub thumbnail_link: String,
    pub featured: bool,
}

impl PgPreparable for Blog {
    fn name() -> &'static str {
        "blog"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        Blog {
            id: row.get(0),
            title: row.get(1),
            slug: row.get(2),
            category_id: row.get(3),
            content: row.get(4),
            image_link: row.get(5),
            thumbnail_link: row.get(6),
            featured: row.get(7),
            publish_date: format_unix_timestamp(row.get(8), None),
        }
    }

    fn columns() -> Vec<&'static str> {
        vec![
            "title",
            "slug",
            "category_id",
            "content",
            "image_link",
            "thumbnail_link",
            "featured",
            "publish_date"
        ]
    }

    fn values(new_entity: &Json<Self>) -> Option<&'static [&'static (dyn ToSql + Sync + '_)]> where Self: std::marker::Sized + PgPreparable + Serialize {

        // let id: &String = &new_entity.id;
        let title: &String = &new_entity.title;
        let slug: &String = &new_entity.slug;
        let category_id: &i32 = &new_entity.category_id;
        let content: &String = &new_entity.content;
        let image_link: &String = &new_entity.image_link;
        let thumbnail_link: &String = &new_entity.thumbnail_link;
        let featured: &bool = &new_entity.featured;

        let publish_date: &SystemTime = &SystemTime::now();

        Some(&[
            &title,
            &slug,
            &category_id,
            &content,
            &image_link,
            &thumbnail_link,
            &featured,
            &publish_date,
        ])
    }
}
