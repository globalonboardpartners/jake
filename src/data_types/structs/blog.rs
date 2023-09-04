use serde::{Serialize, Deserialize};
use crate::data_types::traits::SerializeStruct;
use crate::utils::format_unix_timestamp;

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

impl SerializeStruct for Blog {
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
}
