use crate::data_types::structs::{Id, JobListing};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono::Utc;
use sqlx::Error;

#[post("/job_listing")]
async fn create_job_listing(req: HttpRequest, job_listing: Json<JobListing>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<JobListing, Error> = sqlx::query_as!(
                JobListing,
                "
                    INSERT INTO job_listing
                        (
                            title,
                            slug,
                            description,
                            location,
                            employment_basis,
                            publish_date
                        )
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING *;
                ",
                job_listing.title,
                job_listing.slug,
                job_listing.description,
                job_listing.location,
                job_listing.employment_basis,
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

#[get("/job_listings")]
async fn get_all_job_listings(req: HttpRequest) -> HttpResponse {
    match db::connect(req).await {
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
            .body(e.message),
    }
}

#[get("/job_listing")]
async fn get_job_listing_by_id(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
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
            .body(e.message),
    }
}

#[put("/job_listing")]
async fn update_job_listing(req: HttpRequest, job_listing: Json<JobListing>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<JobListing, Error> = sqlx::query_as!(
                JobListing,
                "
                    INSERT INTO job_listing
                        (
                            id,
                            title,
                            slug,
                            description,
                            location,
                            employment_basis,
                            publish_date
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        title = EXCLUDED.title,
                        slug = EXCLUDED.slug,
                        description = EXCLUDED.description,
                        location = EXCLUDED.location,
                        employment_basis = EXCLUDED.employment_basis,
                        publish_date = EXCLUDED.publish_date
                    RETURNING *;
                ",
                job_listing.id,
                job_listing.title,
                job_listing.slug,
                job_listing.description,
                job_listing.location,
                job_listing.employment_basis,
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
            .body(e.message),
    }
}

#[delete("/job_listing")]
async fn delete_job_listing(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
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
            .body(e.message),
    }
}
