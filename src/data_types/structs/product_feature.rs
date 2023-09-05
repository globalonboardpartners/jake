use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use actix_web::web::Json;
use tokio_postgres::types::ToSql;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductFeature {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductFeature {
    pub title: String,
    pub description: String,
}

impl PgPreparable for ProductFeature {
    fn name() -> &'static str {
        "product_feature"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        ProductFeature {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
        }
    }

    fn columns() -> Vec<&'static str> {
        vec!["title", "description"]
    }

    fn values(new_entity: &Json<Self>) -> Option<&'static [&'static (dyn ToSql + Sync + '_)]> where Self: std::marker::Sized + PgPreparable + Serialize {
        let title: &String = &new_entity.title;
        let description: &String = &new_entity.description;

        Some(&[&title, &description])
    }
}

