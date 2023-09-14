use crate::data_types::structs::Auth;
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{http, post, HttpRequest, HttpResponse};
use sqlx::Row;

#[post("/auth")]
async fn login(req: HttpRequest, auth: Json<Auth>) -> HttpResponse {
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

