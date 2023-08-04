use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub position: f64,
    pub bio: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEmployee {
    pub name: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogCategory {
    pub id: i32,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub publish_date: String,
    pub category_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub publish_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductFeature {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Return<T> {
    pub data: T
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: i32
}