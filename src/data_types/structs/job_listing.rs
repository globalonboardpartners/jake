use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub location: String,
    pub employment_basis: String,
    pub created: Option<String>,
    pub edited: Option<String>,
}
