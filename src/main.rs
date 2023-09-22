use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::cookie::Key;
use actix_session::SessionMiddleware;
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use std::env;
use std::io::Result;

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64])
    )
    .cookie_name(String::from("my-kata-cookie")) // arbitrary name
    .cookie_secure(false) // https only
    .session_lifecycle(BrowserSession::default()) // expire at end of session
    .cookie_content_security(CookieContentSecurity::Private) // set the policy to Private
    .cookie_domain(Some(String::from("http://localhost:3000")))
    .cookie_path(String::from("/blog"))
    .build()
}

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {
        dotenv::dotenv().ok();
        let mut allowed_origin = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set");

        if cfg!(debug_assertions) {
            allowed_origin = env::var("DEV_FRONTEND_URL").expect("DEV_FRONTEND_URL is not set");
        }

        let cors = Cors::permissive()
            .allowed_origin(allowed_origin.as_str())
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            // .wrap(session_middleware())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), Key::from(&[0; 64])))
            .service(
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

            .default_service(web::to(|| HttpResponse::Ok()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
