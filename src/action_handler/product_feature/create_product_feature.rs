use actix_web::web::Json;
use crate::db::insert;
use crate::data_types::structs::NewProductFeature;

pub async fn execute(new_product_feature: Json<NewProductFeature>) {
    let title: &String = &new_product_feature.title;
    let description: &String = &new_product_feature.description;

    insert("product_feature",
        vec!["title", "description"],
        Some(&[
            &title,
            &description,
        ]
    )).await;
}

