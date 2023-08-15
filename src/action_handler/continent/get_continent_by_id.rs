use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, Continent, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let row = get("continent", None, Some(vec!["id"]), Some(&[&id.id])).await;

    data.push(Blog {
        id: row[0].get(0),
        name: row[0].get(1),
        slug: row[0].get(2),
        description_long: row[0].get(3),
        description_short: row[0].get(4),
        image_link: row[0].get(5),
        thumbnail_link: row[0].get(6),
        special_offer_image_link: row[0].get(7),
        video_link: row[0].get(8),
        gallery: row[0].get(9),
        tags: row[0].get(10),
        created: row[0].get(11),
        edited: row[0].get(12),
    });

    let return_data: Return<Vec<Continent>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
