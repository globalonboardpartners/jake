use actix_web::{get, post, put, delete, http, HttpResponse};
use actix_web::web::Json;
use crate::utils::handle_sql_error;
use crate::db;
use crate::data_types::structs::{Id, BlogCategory};
use sqlx::Error;
use actix_web::http::StatusCode;
use sqlx::postgres::PgQueryResult;

#[post("/blog_category")]
async fn create_blog_category(blog_category: Json<BlogCategory>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<BlogCategory, Error> = sqlx::query_as!(
                BlogCategory,
                "
                    INSERT INTO blog_category
                        (
                            category
                        )
                    VALUES ($1)
                    RETURNING *;
                ",
                blog_category.category,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Created()
                        .status(StatusCode::CREATED)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

#[get("/blog_categories")]
async fn get_all_blog_categories() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<BlogCategory>, Error> = sqlx::query_as!(
                BlogCategory,
                "SELECT * from blog_category;"
            )
            .fetch_all(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

#[get("/blog_category")]
async fn get_blog_category_by_id(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<BlogCategory, Error> = sqlx::query_as!(
                BlogCategory,
                "SELECT * FROM blog_category WHERE id = $1;",
                id.id
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

#[put("/blog_category")]
async fn update_blog_category(blog_category: Json<BlogCategory>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<BlogCategory, Error> = sqlx::query_as!(
                BlogCategory,
                "
                    INSERT INTO blog_category
                        (
                            id,
                            category
                        )
                    VALUES ($1, $2)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                    id = EXCLUDED.id, category = EXCLUDED.category
                    RETURNING *;
                ",
                blog_category.id,
                blog_category.category,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

#[delete("/blog_category")]
async fn delete_blog_category(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                BlogCategory,
                "
                    DELETE FROM blog_category 
                    WHERE id = $1
                    AND NOT EXISTS (
                        SELECT 1 FROM blog WHERE category_id = $1
                    );
                ",
                id.id
            )
            .execute(&pg)
            .await;

            match returned {
                Ok(_) => {
                    HttpResponse::NoContent()
                        .status(StatusCode::NO_CONTENT)
                        .content_type("application/json") 
                        .finish()
                },

                Err(e) => handle_sql_error(e)
            }
        },

        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

