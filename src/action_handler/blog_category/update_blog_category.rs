use actix_web::web::Json;
use crate::db::update;
use crate::data_types::structs::{UpdateColumn, ColumnValue};

pub async fn execute(blog_category_update: Json<UpdateColumn<>>) -> Vec<tokio_postgres::Row> {
    let value: String = match &blog_category_update.col_value {
        ColumnValue::Integer(num) => format!("{}", num),
        ColumnValue::Float(num) => format!("{}", num),
        ColumnValue::Text(text) => text.clone(),
    };

    update(
        "blog_category",
        Some(vec![&blog_category_update.col_name]),
        Some(vec!["id"]),
        Some(&[&value, &blog_category_update.id.unwrap()]),
    ).await
}
