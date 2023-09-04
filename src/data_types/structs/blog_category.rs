use serde::{Serialize, Deserialize};
use crate::data_types::traits::SerializeStruct;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogCategory {
    pub id: i32,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlogCategory {
    pub category: String,
}

impl SerializeStruct for BlogCategory {
    fn name() -> &'static str {
        "blog_category"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        BlogCategory { id: row.get(0), category: row.get(1) }
    }
}
