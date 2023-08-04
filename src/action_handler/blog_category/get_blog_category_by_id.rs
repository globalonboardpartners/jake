use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, BlogCategory, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let row = get("blog_category", None, Some(vec!["id"]), Some(&[&id.id])).await;

    data.push(BlogCategory {
        id: row[0].get(0),
        category: row[0].get(1),
    });

    let return_data: Return<Vec<BlogCategory>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
