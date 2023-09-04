use serde::{Serialize, Deserialize};
use crate::data_types::traits::SerializeStruct;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductFeature {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductFeature {
    pub title: String,
    pub description: String,
}

impl SerializeStruct for ProductFeature {
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
}
