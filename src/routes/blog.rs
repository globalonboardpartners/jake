use crate::data_types::structs::{Blog, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono::Utc;
use sqlx::Error;

#[post("/blog")]
async fn create_blog(req: HttpRequest, blog: Json<Blog>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                "
                    INSERT INTO blog
                        (
                            title,
                            slug,
                            category_id,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            publish_date
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING *;
                ",
                blog.title,
                blog.slug,
                blog.category_id,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured,
                Utc::now().naive_utc(),
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

#[get("/blog")]
async fn get_blog_by_id_or_all(req: HttpRequest, Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Blog, Error> =
                    sqlx::query_as!(Blog, "SELECT * FROM blog WHERE id = $1;", id.id)
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
    } else {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Vec<Blog>, Error> =
                    sqlx::query_as!(Blog, "SELECT * from blog;")
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
}

#[put("/blog")]
async fn update_blog(req: HttpRequest, blog: Json<Blog>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                "
                    INSERT INTO blog (id, title, slug, category_id, content, image_link, thumbnail_link, featured, publish_date)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        title = EXCLUDED.title,
                        slug = EXCLUDED.slug,
                        category_id = EXCLUDED.category_id,
                        content = EXCLUDED.content,
                        image_link = EXCLUDED.image_link,
                        thumbnail_link = EXCLUDED.thumbnail_link,
                        featured = EXCLUDED.featured,
                        publish_date = EXCLUDED.publish_date
                    RETURNING *;
                ",
                blog.id,
                blog.title,
                blog.slug,
                blog.category_id,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured,
                blog.publish_date,
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

#[delete("/blog")]
async fn delete_blog(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =
                sqlx::query_as!(Blog, "DELETE FROM blog WHERE id = $1;", id.id)
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
