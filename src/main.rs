use actix_web::{App, HttpServer};
use std::io::Result;

pub mod action_handler;
pub mod db;
pub mod db2;
pub mod routes;
pub mod data_types;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new()
        .service(routes::employee())
        .service(routes::job_listing())
        .service(routes::blog())
        .service(routes::blog_category())
        .service(routes::product_feature())
        .service(routes::product_feature())
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .expect("\nERROR: src/main.rs: server initialization fail\n");

        Ok(())
}

