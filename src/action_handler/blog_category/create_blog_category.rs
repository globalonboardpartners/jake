use actix_web::web::Json;
use crate::db::insert;
use crate::data_types::structs::NewBlogCategory;

pub async fn execute(new_blog_category: Json<NewBlogCategory>) {
    let category: &String = &new_blog_category.category;

    insert("blog_category",
        vec!["category"],
        Some(&[
            &category,
        ]
    )).await;
}

