use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::{Id, ProductFeature, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let res = QueryBuilder::get("product_feature", None, Some(vec!["id"]), Some(&[&id.id])).await;

    match res {
        Ok(row) => {
            if row.is_empty() {
                return format!(r#"{{"error": "row with id of {} not found"}}"#, &id.id);
            }
            data.push(ProductFeature {
                id: row[0].get(0),
                title: row[0].get(1),
                description: row[0].get(2),
            });

            let return_data: Return<Vec<ProductFeature>> = Return {data};

            serde_json::to_string(&return_data).unwrap()
        }
        Err(e) => {
            format!(r#"{{"error": "{}"}}"#, e)
        }
    }
}
