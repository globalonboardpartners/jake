use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use actix_web::web::Json;
use tokio_postgres::types::ToSql;

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

impl PgPreparable for Employee {
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

    fn columns() -> Vec<&'static str> {
        vec!["name", "position", "bio", "image_url"]
    }

    fn values(new_entity: &Json<Self>) -> Option<&'static [&'static (dyn ToSql + Sync + '_)]> where Self: std::marker::Sized + PgPreparable + Serialize {
        let name: &String = &new_entity.name;
        let position: &String = &new_entity.position;
        let bio: &String = &new_entity.bio;
        let image_url: &String = &new_entity.image_url;

        Some(&[
            &name,
            &position,
            &bio,
            &image_url,
        ])
    }
}
