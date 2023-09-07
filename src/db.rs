use sqlx::PgPool;
use std::env;
use sqlx::Error;
use crate::data_types::types::ErrorMessage;

pub async fn connect() -> Result<PgPool, ErrorMessage> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url).await;

    match pool {
        Ok(db) => Ok(db),
        Err(e) => match e {
            Error::Configuration(e) => Err(format!("Configuration error: {}", e)),
            Error::Database(e) => Err(format!("Database error: {}", e)),
            _ => Err(format!("Unknown Error: {}", e))
        }
    }
}

