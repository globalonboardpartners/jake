use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::{Id, Employee, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let res = QueryBuilder::get("employee", None, Some(vec!["id"]), Some(&[&id.id])).await;

    match res {
        Ok(row) => {
            if row.is_empty() {
                return format!(r#"{{"error": "row with id of {} not found"}}"#, &id.id);
            }
            data.push(Employee {
                id: row[0].get(0),
                name: row[0].get(1),
                position: row[0].get(2),
                bio: row[0].get(3),
                image_url: row[0].get(4),
            });

            let return_data: Return<Vec<Employee>> = Return {data};

            serde_json::to_string(&return_data).unwrap()
        }
        Err(e) => {
            format!(r#"{{"error": "{}"}}"#, e)
        }
    }
}
