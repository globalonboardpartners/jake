pub trait SerializeStruct {
    fn name() -> &'static str;
    fn new_from_row(row: &tokio_postgres::Row) -> Self;
}
