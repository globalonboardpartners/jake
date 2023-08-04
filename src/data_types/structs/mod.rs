use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub position: String,
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
    pub publish_date: SystemTime,
    pub category_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlog {
    pub title: String,
    pub content: String,
    pub publish_date: SystemTime,
    pub category_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobListing {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub publish_date: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewJobListing {
    pub title: String,
    pub description: String,
    pub publish_date: SystemTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductFeature {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductFeature {
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ColumnValue {
    Integer(i32),
    Float(f64),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateColumn {
    pub id: Option<i32>,
    pub col_name: String,
    pub col_value: ColumnValue,
}
