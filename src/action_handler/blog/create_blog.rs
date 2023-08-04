use actix_web::web::Json;
use std::time::SystemTime;
use crate::db::insert;
use crate::data_types::structs::{NewBlog};

pub async fn execute(new_blog: Json<NewBlog>) {
    let title: &String = &new_blog.title;
    let content: &String = &new_blog.content;
    let publish_date: &SystemTime = &new_blog.publish_date;
    let category_id: &i32 = &new_blog.category_id;

    insert("blog",
        vec!["title", "content", "publish_date", "category_id"],
        Some(&[
            &title,
            &content,
            &publish_date,
            &category_id,
        ]
    )).await;
}

