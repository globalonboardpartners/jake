use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::Id;
use crate::data_types::traits::SerializeStruct;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use serde::Serialize;


pub async fn execute<T>(id: Json<Id>) -> Result<Json<T>, InternalError<String>>
where
    T: SerializeStruct + Serialize
{
    let res = QueryBuilder::get(T::name(), None, Some(vec!["id"]), Some(&[&id.id])).await;

    let rows = res.map_err(|e| {
        InternalError::new(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    if rows.is_empty() {
        return Err(InternalError::new(
            format!("row with id of {} not found", id.id),
            StatusCode::NOT_FOUND
        ));
    }

    let data = T::new_from_row(&rows[0]);

    Ok(Json(data))
}
