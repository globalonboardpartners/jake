use actix_web::web::Json;
use crate::db::update;
use crate::data_types::structs::{UpdateColumn, ColumnValue};

pub async fn execute(product_feature_update: Json<UpdateColumn<>>) -> Vec<tokio_postgres::Row> {
    let value: String = match &product_feature_update.col_value {
        ColumnValue::Integer(num) => format!("{}", num),
        ColumnValue::Float(num) => format!("{}", num),
        ColumnValue::Text(text) => text.clone(),
    };

    update(
        "product_feature",
        Some(vec![&product_feature_update.col_name]),
        Some(vec!["id"]),
        Some(&[&value, &product_feature_update.id.unwrap()]),
    ).await
}
