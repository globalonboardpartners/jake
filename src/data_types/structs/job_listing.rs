use serde::{Serialize, Deserialize};
use crate::data_types::traits::SerializeStruct;
use crate::utils::format_unix_timestamp;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub publish_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewJobListing {
    pub title: String,
    pub description: String,
}

impl SerializeStruct for JobListing {
    fn name() -> &'static str {
        "job_listing"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        JobListing {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            publish_date: format_unix_timestamp(row.get(3), None),
        }
    }
}
