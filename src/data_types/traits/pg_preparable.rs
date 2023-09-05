use actix_web::web::Json;
use tokio_postgres::types::ToSql;
use serde::Serialize;

pub trait PgPreparable {
    fn name() -> &'static str;
    fn new_from_row(row: &tokio_postgres::Row) -> Self;
    fn columns() -> Vec<&'static str>;
    fn values(&self) -> Vec<&(dyn ToSql + Sync + '_)>;
}
