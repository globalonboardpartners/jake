use actix_web::{App, HttpServer};
use std::io::Result;
use actix_cors::Cors;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .service(routes::employee())
            .service(routes::client())
            .service(routes::job_listing())
            .service(routes::blog())
            .service(routes::blog_category())
            .service(routes::product_feature())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
