use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, Blog, Return};
use crate::utils::format_unix_timestamp;

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let res = get("blog", None, Some(vec!["id"]), Some(&[&id.id])).await;

    match res {
        Ok(row) => {
            data.push(Blog {
                id: row[0].get(0),
                title: row[0].get(1),
                slug: row[0].get(2),
                category_id: row[0].get(3),
                content: row[0].get(4),
                image_link: row[0].get(5),
                thumbnail_link: row[0].get(6),
                featured: row[0].get(7),
                publish_date: format_unix_timestamp(row[0].get(8), None),
            });

            let return_data: Return<Vec<Blog>> = Return {data};

            serde_json::to_string(&return_data).unwrap()
        }
        Err(e) => {
            format!(r#"{{"error": "{}"}}"#, e)
        }
    }
}
