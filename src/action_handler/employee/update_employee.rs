use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::{UpdateColumn, ColumnValue};

pub async fn execute(employee_update: Json<UpdateColumn<>>) -> Vec<tokio_postgres::Row> {
    let value: String = match &employee_update.col_value {
        ColumnValue::Integer(num) => format!("{}", num),
        ColumnValue::Float(num) => format!("{}", num),
        ColumnValue::Text(text) => text.clone(),
    };

    QueryBuilder::update(
        "employee",
        Some(vec![&employee_update.col_name]),
        Some(vec!["id"]),
        Some(&[&value, &employee_update.id.unwrap()]),
    ).await
}
