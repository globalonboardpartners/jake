use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, Blog, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let row = get("blog", None, Some(vec!["id"]), Some(&[&id.id])).await;

    data.push(Blog {
        id: row[0].get(0),
        title: row[0].get(1),
        content: row[0].get(2),
        publish_date: row[0].get(3),
        category_id: row[0].get(4),
    });

    let return_data: Return<Vec<Blog>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
