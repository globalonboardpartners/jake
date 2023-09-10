use crate::data_types::structs::{BlogCategory, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/blog_category")]
async fn create_blog_category(req: HttpRequest, blog_category: Json<BlogCategory>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<BlogCategory, Error> = sqlx::query_as!(
                BlogCategory,
                "
                    INSERT INTO blog_category
                        (
                            category,
                            slug
                        )
                    VALUES ($1, $2)
                    RETURNING *;
                ",
                blog_category.category,
                blog_category.slug
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[get("/blog_categories")]
async fn get_all_blog_categories(req: HttpRequest) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Vec<BlogCategory>, Error> =
                sqlx::query_as!(BlogCategory, "SELECT * from blog_category;")
                    .fetch_all(&pg)
                    .await;

            match returned {
                Ok(record) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[get("/blog_category")]
async fn get_blog_category_by_id(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<BlogCategory, Error> = sqlx::query_as!(
                BlogCategory,
                "SELECT * FROM blog_category WHERE id = $1;",
                id.id
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[put("/blog_category")]
async fn update_blog_category(req: HttpRequest, blog_category: Json<BlogCategory>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<BlogCategory, Error> = sqlx::query_as!(
                BlogCategory,
                "
                    INSERT INTO blog_category
                        (
                            id,
                            category,
                            slug
                        )
                    VALUES ($1, $2, $3)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                    id = EXCLUDED.id, category = EXCLUDED.category, slug = EXCLUDED.slug
                    RETURNING *;
                ",
                blog_category.id,
                blog_category.category,
                blog_category.slug,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),

                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[delete("/blog_category")]
async fn delete_blog_category(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                BlogCategory,
                "DELETE FROM blog_category WHERE id = $1;",
                id.id
            )
            .execute(&pg)
            .await;

            match returned {
                Ok(_) => HttpResponse::NoContent()
                    .status(StatusCode::NO_CONTENT)
                    .content_type("application/json")
                    .finish(),

                Err(e) => handle_sql_error(e),
            }
        }

        Err(e) => HttpResponse::InternalServerError()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}
