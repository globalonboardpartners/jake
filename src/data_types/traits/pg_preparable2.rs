use actix_web::web::Json;

pub trait PgPreparable2 {
    fn name() -> &'static str;
    fn write_update_sql(update_body: &Self, id: String) -> String;
    fn id(&self) -> Option<i32>;
    fn into_id(&self) -> String;
}
