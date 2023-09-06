use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use crate::utils::format_unix_timestamp;
use tokio_postgres::types::ToSql;
use std::time::SystemTime;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub publish_date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewJobListing {
    pub title: String,
    pub description: String,
}

impl PgPreparable for JobListing {
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

    fn columns() -> Vec<&'static str> {
        vec!["title", "description", "publish_date"]
    }

    fn values(&self) -> Vec<&(dyn ToSql + Sync + '_)> {
        let title: &String = &self.title;
        let description: &String = &self.description;

        vec![
            title,
            description,
        ]
    }
}

// impl JobListing {
//     fn values(&self) -> Option<&'static [&'static (dyn ToSql + Sync + '_)]> where Self: std::marker::Sized + PgPreparable + Serialize {
//         let title: &String = &self.title;
//         let description: &String = &self.description;

//         let publish_date: &SystemTime = &SystemTime::now();

//         Some(&[
//             &title,
//             &description,
//             &publish_date,
//         ])
//     }
// }
