use actix_web::web::Json;
use crate::db::QueryBuilder;
use crate::data_types::structs::Id;

pub async fn execute(id: Json<Id>) {
    QueryBuilder::delete("employee", Some(vec!["id"]), Some(&[&id.id])).await;
}
