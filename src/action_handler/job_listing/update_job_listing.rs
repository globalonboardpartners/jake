use actix_web::web::Json;
use crate::db::update;
use crate::data_types::structs::{UpdateColumn, ColumnValue};

pub async fn execute(job_listing_update: Json<UpdateColumn<>>) -> Vec<tokio_postgres::Row> {
    let value: String = match &job_listing_update.col_value {
        ColumnValue::Integer(num) => format!("{}", num),
        ColumnValue::Float(num) => format!("{}", num),
        ColumnValue::Text(text) => text.clone(),
    };

    update(
        "job_listing",
        Some(vec![&job_listing_update.col_name]),
        Some(vec!["id"]),
        Some(&[&value, &job_listing_update.id.unwrap()]),
    ).await
}
