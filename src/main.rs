use actix_cors::Cors;
use actix_web::{error::InternalError, Error, HttpResponse, HttpMessage, web, App, HttpServer, dev::{ServiceResponse, Transform, Service}};
use std::env;
use std::io::Result;
use jsonwebtoken::{decode, Validation, Algorithm, DecodingKey};
use serde::{Serialize, Deserialize};
use futures::future::{ok, Ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

pub mod data_types;
pub mod db;
pub mod routes;
pub mod utils;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct Auth;

impl<S, B, Req> Transform<S, Req> for Auth
where
    S: Service<Req, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    Req: actix_web::HttpMessage + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<actix_web::Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B, Req: HttpMessage + 'static> Service<Req> for AuthMiddleware<S>
where
    S: Service<Req, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = actix_web::Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<actix_web::Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: Req) -> Self::Future {
        // Clone the headers from the request
        let headers = req.headers().clone();

        let fut = self.service.call(req);

        dotenv::dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

        Box::pin(async move {
            if let Some(auth_header) = headers.get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];

                        let decoding_key_result = DecodingKey::from_base64_secret(&jwt_secret);
                        
                        match decoding_key_result {
                            Ok(decoding_key) => {
                                let validation = Validation::new(Algorithm::HS256);
                                match decode::<Claims>(token, &decoding_key, &validation) {
                                    Ok(_) => {
                                        return fut.await;
                                    },
                                    Err(_) => {
                                        return Err(InternalError::from_response("UNAUTHORIZED", HttpResponse::Unauthorized().finish()).into());
                                    }
                                }
                            },
                            Err(_) => {
                                // Redirect to login route if decoding_key is an error
                                return Err(InternalError::from_response("UNAUTHORIZED", HttpResponse::Found().append_header(("Location", "/api/v1/auth")).finish()).into());
                            }
                        }
                    }
                }
            }
            Err(InternalError::from_response("UNAUTHORIZED", HttpResponse::Unauthorized().finish()).into())
        })
    }
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
            .allow_any_header();

        App::new()
            .wrap(cors)
            // Apply Auth middleware only to the scope "/api/v1/secure"
            .service(
                web::scope("/api/v1/s")
                    .wrap(Auth)
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
            // No Auth middleware applied to this route
            .service(
                web::scope("/api/v1")
                    .service(routes::auth())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
