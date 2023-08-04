use actix_web::web::Json;
use crate::db::delete;
use crate::data_types::structs::Id;

pub async fn execute(id: Json<Id>) {
    delete("blog", Some(vec!["id"]), Some(&[&id.id])).await;
}
