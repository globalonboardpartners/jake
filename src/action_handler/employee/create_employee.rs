use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::{NewEmployee};

pub async fn execute(new_product: Json<NewEmployee>) {
    let name: &String = &new_product.name;
    let position: &String = &new_product.position;
    let bio: &String = &new_product.bio;
    let image_url: &String = &new_product.image_url;

    QueryBuilder::insert("employee",
        vec!["name", "position", "bio", "image_url"],
        Some(&[
            &name,
            &position,
            &bio,
            &image_url,
        ]
    )).await;
}

