use actix_web::web::Json;
use std::time::SystemTime;
use crate::db::insert;
use crate::data_types::structs::NewJobListing;

pub async fn execute(new_product: Json<NewJobListing>) {
    let title: &String = &new_product.title;
    let description: &String = &new_product.description;

    let publish_date: &SystemTime = &SystemTime::now();

    insert("job_listing",
        vec!["title", "description", "publish_date"],
        Some(&[
            &title,
            &description,
            &publish_date,
        ]
    )).await;
}

