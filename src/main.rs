use actix_web::{web, App, HttpServer};
use std::io::Result;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;
pub mod middleware;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {

        App::new()
            .wrap(middleware::handle_cors())
            .service(
                web::scope("/api/v1/s")
                    .wrap(middleware::CaptureUri)
                    .wrap(middleware::JWTAuth)
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
                    .service(routes::activity())
                    .service(routes::event())
                    .service(routes::event_details())
            )
            .service(
                web::scope("/api/v1")
                    .wrap(middleware::CaptureUri)
                    .service(routes::auth())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
