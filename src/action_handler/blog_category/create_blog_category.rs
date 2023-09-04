use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::NewBlogCategory;

pub async fn execute(new_blog_category: Json<NewBlogCategory>) {
    let category: &String = &new_blog_category.category;

    QueryBuilder::insert("blog_category",
        vec!["category"],
        Some(&[
            &category,
        ]
    )).await;
}

