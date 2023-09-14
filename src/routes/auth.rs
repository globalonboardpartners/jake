use crate::data_types::structs::Auth;
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{http, post, HttpRequest, HttpResponse};
use sqlx::Row;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[post("/auth")]
async fn login(req: HttpRequest, auth: Json<Auth>) -> HttpResponse {
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

            match returned {
                Ok(record) => {
                    HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
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

