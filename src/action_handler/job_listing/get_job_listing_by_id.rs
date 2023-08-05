use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, JobListing, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let row = get("job_listing", None, Some(vec!["id"]), Some(&[&id.id])).await;

    data.push(JobListing {
        id: row[0].get(0),
        title: row[0].get(1),
        description: row[0].get(2),
        publish_date: row[0].get(3),
    });

    let return_data: Return<Vec<JobListing>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}