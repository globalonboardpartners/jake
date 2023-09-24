use crate::data_types::structs::Auth;
use std::env;
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::{StatusCode, header};
use actix_web::web::Json;
use actix_web::{http, post, HttpRequest, HttpResponse};
use sqlx::Row;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: u64
}

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString
    },
    Argon2
};

#[post("/auth")]
async fn login(req: HttpRequest, auth: Json<Auth>) -> HttpResponse {
    dotenv::dotenv().ok();

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    
    // Add 1 hour (3600 seconds) to the current Unix timestamp
    let exp = since_the_epoch.as_secs() + 3600;

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

    let my_claims = Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp };

    let encoding_key = EncodingKey::from_base64_secret(&jwt_secret)
        .expect("Failed to decode base64 secret");

    let decoding_key = DecodingKey::from_base64_secret(&jwt_secret)
        .expect("Failed to decode base64 secret");

    // Encoding
    let token = encode(&Header::new(Algorithm::HS256), &my_claims, &encoding_key).unwrap();

    // Decoding
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(&token, &decoding_key, &validation).unwrap();

    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
    /////////////////////////////////////////////////////////////////
    let password = b"hunter42"; // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);
    
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();
    dbg!(&password_hash);
    
    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    dbg!(parsed_hash);
    match db::connect(req).await {
        Ok(pg) => {
            let returned = sqlx::query(
                "
                    SELECT EXISTS (
                        SELECT 1
                        FROM auth
                        WHERE username = $1 AND password = $2
                        LIMIT 1
                    );
                "
            )
            .bind(&auth.username)
            .bind(&auth.password)
            .fetch_one(&pg)
            .await;

            /*

            NOTE FOR WHEN I RETURN:
            I think I send the JWT back via an auth header. I need to next turn this JWT portion of code into it's own middleware (just the reading part).
            I also need to allow every single route to be protected by the JWT middleware except for the login route.
            IF the user's auth token is invalid (or does not exist), return the user back to the login page
            
            gonna have to think about how I'll return the user to the login page

            */

            match returned {
                Ok(record) => {
                    HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .append_header((header::AUTHORIZATION, format!("Bearer {}", token)))
                    .body(
                        serde_json::to_string(&Json(record.get::<bool, usize>(0)))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    )},

                Err(e) => {
                    handle_sql_error(e)
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

