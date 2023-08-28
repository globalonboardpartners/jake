use actix_web::web::Json;
use std::time::SystemTime;
use crate::db::insert;
use crate::data_types::structs::{NewBlog};

pub async fn execute(new_blog: Json<NewBlog>) {
    let title: &String = &new_blog.title;
    let slug: &String = &new_blog.slug;
    let category_id: &i32 = &new_blog.category_id;
    let content: &String = &new_blog.content;
    let image_link: &String = &new_blog.image_link;
    let thumbnail_link: &String = &new_blog.thumbnail_link;
    let featured: &bool = &new_blog.featured;

    insert("blog",
        vec!["title", "slug", "category_id","content", "image_link", "thumbnail_link", "featured"],
        Some(&[
            &title,
            &slug,
            &category_id,
            &content,
            &image_link,
            &thumbnail_link,
        ]
    )).await;
}

