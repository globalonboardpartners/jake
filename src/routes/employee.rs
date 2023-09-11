use crate::data_types::structs::{Employee, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/employee")]
async fn create_employee(req: HttpRequest, employee: Json<Employee>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Employee, Error> = sqlx::query_as!(
                Employee,
                "
                    INSERT INTO employee
                        (
                            name,
                            slug,
                            position,
                            bio,
                            image_url,
                            twitter_link,
                            linkedin_link,
                            email
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING
                        id,
                        name,
                        slug,
                        position,
                        bio,
                        image_url,
                        twitter_link,
                        linkedin_link,
                        email,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY'))
                        ) as edited
                ",
                employee.name,
                employee.slug,
                employee.position,
                employee.bio,
                employee.image_url,
                employee.twitter_link,
                employee.linkedin_link,
                employee.email,
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

#[get("/employee")]
async fn get_employee_by_id_or_all(req: HttpRequest, Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Employee, Error> = sqlx::query_as!(
                    Employee,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            position,
                            bio,
                            image_url,
                            twitter_link,
                            linkedin_link,
                            email,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY'))
                            ) as edited
                        FROM employee
                        WHERE id = $1
                        LIMIT 1;
                    ",
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
    } else {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Vec<Employee>, Error> = sqlx::query_as!(
                    Employee,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            position,
                            bio,
                            image_url,
                            twitter_link,
                            linkedin_link,
                            email,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY'))
                            ) as edited
                        FROM employee
                    "
                )
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

#[put("/employee")]
async fn update_employee(req: HttpRequest, employee: Json<Employee>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Employee, Error> = sqlx::query_as!(
                Employee,
                "
                    INSERT INTO employee
                        (
                            id,
                            name,
                            slug,
                            position,
                            bio,
                            image_url,
                            twitter_link,
                            linkedin_link,
                            email
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug,
                        position = EXCLUDED.position,
                        bio = EXCLUDED.bio,
                        image_url = EXCLUDED.image_url,
                        twitter_link = EXCLUDED.twitter_link,
                        linkedin_link = EXCLUDED.linkedin_link,
                        email = EXCLUDED.email
                    RETURNING
                        id,
                        name,
                        slug,
                        position,
                        bio,
                        image_url,
                        twitter_link,
                        linkedin_link,
                        email,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY'))
                        ) as edited
                ",
                employee.id,
                employee.name,
                employee.slug,
                employee.position,
                employee.bio,
                employee.image_url,
                employee.twitter_link,
                employee.linkedin_link,
                employee.email,
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

#[delete("/employee")]
async fn delete_employee(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =
                sqlx::query_as!(Employee, "DELETE FROM employee WHERE id = $1;", id.id)
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
