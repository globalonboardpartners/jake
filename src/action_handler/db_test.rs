use serde_json;
use crate::db::QueryBuilder;
use crate::data_types::structs::Id;

pub async fn execute() -> String {
    let rows = QueryBuilder::query(QueryBuilder::new("SELECT * FROM employee;".to_string(), None)).await.unwrap();

    let id: i32 = rows[0].get(0);
    let new_id: Id = Id {id};
    let serialized = serde_json::to_string(&new_id).unwrap();

    serialized
}
