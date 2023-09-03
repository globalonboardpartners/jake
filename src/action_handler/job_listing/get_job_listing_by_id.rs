use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, JobListing, Return};
use crate::utils::format_unix_timestamp;

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let res = get("job_listing", None, Some(vec!["id"]), Some(&[&id.id])).await;

    match res {
        Ok(row) => {
            data.push(JobListing {
                id: row[0].get(0),
                title: row[0].get(1),
                description: row[0].get(2),
                publish_date: format_unix_timestamp(row[0].get(3), None),
            });

            let return_data: Return<Vec<JobListing>> = Return {data};

            serde_json::to_string(&return_data).unwrap()
        }
        Err(e) => {
            format!(r#"{{"error": "{}"}}"#, e)
        }
    }
}
