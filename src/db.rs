use crate::data_types::structs::ErrorMessage;
use actix_web::http::header;
use actix_web::http::header::HeaderMap;
use actix_web::HttpRequest;
use actix_web::Result;
use sqlx::Row;
use sqlx::{Error, PgPool};
use std::env;

pub async fn connect(req: HttpRequest) -> Result<PgPool, ErrorMessage> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool_result = PgPool::connect(&database_url).await;

    let headers: &HeaderMap = req.headers();
    let bearer_token: Vec<&str> = headers
        .get(header::AUTHORIZATION)
        .ok_or(ErrorMessage::new("Missing API Key"))?
        .to_str()
        .map_err(|_| ErrorMessage::new("Invalid format"))?
        .split_whitespace()
        .collect();

    let api_key: &str = bearer_token[1];

    match pool_result {
        Ok(db) => {
            let row = sqlx::query("SELECT COUNT(*) FROM auth WHERE api_key = $1::UUID")
                .bind(api_key)
                .fetch_one(&db)
                .await;

            match row {
                Ok(record) => {
                    let count: i64 = record.get("count");
                    if count <= 0 {
                        return Err(ErrorMessage::new("API Key is not valid"));
                    }
                    Ok(db)
                }
                Err(_) => Err(ErrorMessage::new("Database query failed")),
            }
        }
        Err(e) => {
            let err_msg = match e {
                Error::Configuration(e) => format!("Configuration error: {}", e),
                Error::Database(e) => format!("Database error: {}", e),
                _ => format!("Unknown Error: {}", e),
            };
            Err(ErrorMessage::new(&err_msg))
        }
    }
}
