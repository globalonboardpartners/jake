use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::env;
use std::io::Result;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        dotenv::dotenv().ok();
        let mut allowed_origin = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set");

        if cfg!(debug_assertions) {
            allowed_origin = env::var("DEV_FRONTEND_URL").expect("DEV_FRONTEND_URL is not set");
        }

        let cors = Cors::permissive()
            .allowed_origin(allowed_origin.as_str())
            .allow_any_method()
            .allow_any_header();
        App::new().wrap(cors).service(
            web::scope("/api/v1")
                .service(routes::employee())
                .service(routes::client())
                .service(routes::job_listing())
                .service(routes::blog())
                .service(routes::blog_category())
                .service(routes::product_feature())
                .service(routes::continent())
                .service(routes::country())
                .service(routes::region())
                .service(routes::city())
                .service(routes::partner_vendor())
                .service(routes::restaurant())
                .service(routes::hotel())
                .service(routes::hotel_room())
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
