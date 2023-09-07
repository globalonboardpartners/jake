use actix_web::{App, HttpServer};
use std::io::Result;
use actix_web::middleware::DefaultHeaders;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Origin", "http://localhost:3000")))
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")))
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Headers", "Content-Type, Authorization")))
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
