use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::env;
use std::io::Result;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    error::InternalError,
    middleware, Error, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let signing_key = Key::generate();
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        dotenv::dotenv().ok();
        let mut allowed_origin = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set");

        if cfg!(debug_assertions) {
            allowed_origin = env::var("DEV_FRONTEND_URL").expect("DEV_FRONTEND_URL is not set");
        }

        let cors = Cors::permissive()
            .allowed_origin(allowed_origin.as_str())
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(middleware::Logger::default())
            // cookie session middleware
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    signing_key.clone(),
                )
                // allow the cookie to be accessed from javascript
                .cookie_http_only(false)
                // allow the cookie only from the current domain
                .cookie_same_site(SameSite::Strict)
                .build(),
            )
            .wrap(cors).service(
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
                .service(routes::activity())
                .service(routes::event())
                .service(routes::event_details())
                .service(routes::auth())
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
