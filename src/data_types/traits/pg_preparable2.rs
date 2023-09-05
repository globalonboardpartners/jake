use actix_web::web::Json;

pub trait PgPreparable2 {
    fn name() -> &'static str;
    fn prepare_update(update_body: Json<&Self>) -> String;
}
