use crate::data_types::structs::{Id, JobListing};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono::Utc;
use sqlx::Error;

#[post("/job_listing")]
async fn create_job_listing(job_listing: Json<JobListing>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<JobListing, Error> = sqlx::query_as!(
                JobListing,
                "
                    INSERT INTO job_listing
                        (
                            title,
                            description,
                            publish_date
                        )
                    VALUES ($1, $2, $3)
                    RETURNING *;
                ",
                job_listing.title,
                job_listing.description,
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
            .body(e),
    }
}

#[get("/job_listings")]
async fn get_all_job_listings() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<JobListing>, Error> =
                sqlx::query_as!(JobListing, "SELECT * from job_listing;")
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
            .body(e),
    }
}

#[get("/job_listing")]
async fn get_job_listing_by_id(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<JobListing, Error> = sqlx::query_as!(
                JobListing,
                "SELECT * FROM job_listing WHERE id = $1;",
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
            .body(e),
    }
}

#[put("/job_listing")]
async fn update_job_listing(job_listing: Json<JobListing>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<JobListing, Error> = sqlx::query_as!(
                JobListing,
                "
                    INSERT INTO job_listing
                        (
                            id,
                            title,
                            description,
                            publish_date
                        )
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                    id = EXCLUDED.id, title = EXCLUDED.title, description = EXCLUDED.description, publish_date = EXCLUDED.publish_date
                    RETURNING *;
                ",
                job_listing.id,
                job_listing.title,
                job_listing.description,
                Utc::now().naive_utc(),
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
            .body(e),
    }
}

#[delete("/job_listing")]
async fn delete_job_listing(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =
                sqlx::query_as!(JobListing, "DELETE FROM employee WHERE id = $1;", id.id)
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
            .body(e),
    }
}
