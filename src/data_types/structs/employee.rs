use serde::{Serialize, Deserialize};
use crate::data_types::traits::SerializeStruct;

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

impl SerializeStruct for Employee {
    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        Employee {
            id: row.get(0),
            name: row.get(1),
            position: row.get(2),
            bio: row.get(3),
            image_url: row.get(4),
        }
    }
    fn name() -> &'static str {
        "employee"
    }
}
