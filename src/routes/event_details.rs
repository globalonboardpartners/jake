use crate::data_types::structs::{EventDetails, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/event_details")]
async fn create_event_details(req: HttpRequest, event_details: Json<EventDetails>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<EventDetails, Error> = sqlx::query_as!(
                EventDetails,
                "
                    INSERT INTO event_details
                        (
                            name,
                            slug,
                            event_date,
                            event_time,
                            event_artist_slug,
                            venue_name,
                            continent,
                            country,
                            region,
                            city,
                            ticket_link,
                            gallery,
                            tags
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                    RETURNING
                        id,
                        name,
                        slug,
                        event_date,
                        event_time,
                        event_artist_slug,
                        venue_name,
                        continent,
                        country,
                        region,
                        city,
                        ticket_link,
                        gallery,
                        tags,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY HH12:MI AM'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY HH12:MI AM'))
                        ) as edited
                ",
                event_details.name,
                event_details.slug,
                event_details.event_date,
                event_details.event_time,
                event_details.event_artist_slug,
                event_details.venue_name,
                event_details.continent,
                event_details.country,
                event_details.region,
                event_details.city,
                event_details.ticket_link,
                event_details.gallery.as_slice(),
                event_details.tags
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

#[get("/event_details")]
async fn get_event_details_by_id_or_all(req: HttpRequest, Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<EventDetails, Error> = sqlx::query_as!(
                    EventDetails,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            event_date,
                            event_time,
                            event_artist_slug,
                            venue_name,
                            continent,
                            country,
                            region,
                            city,
                            ticket_link,
                            gallery,
                            tags,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited
                        FROM event_details
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
                let returned: Result<Vec<EventDetails>, Error> = sqlx::query_as!(
                    EventDetails,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            event_date,
                            event_time,
                            event_artist_slug,
                            venue_name,
                            continent,
                            country,
                            region,
                            city,
                            ticket_link,
                            gallery,
                            tags,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited
                        FROM event_details
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

#[put("/event_details")]
async fn update_event_details(req: HttpRequest, event_details: Json<EventDetails>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<EventDetails, Error> = sqlx::query_as!(
                EventDetails,
                "
                    INSERT INTO event_details
                        (
                            id,
                            name,
                            slug,
                            event_date,
                            event_time,
                            event_artist_slug,
                            venue_name,
                            continent,
                            country,
                            region,
                            city,
                            ticket_link,
                            gallery,
                            tags
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug,
                        event_date = EXCLUDED.event_date,
                        event_time = EXCLUDED.event_time,
                        event_artist_slug = EXCLUDED.event_artist_slug,
                        venue_name = EXCLUDED.venue_name,
                        continent = EXCLUDED.continent,
                        country = EXCLUDED.country,
                        region = EXCLUDED.region,
                        city = EXCLUDED.city,
                        gallery = EXCLUDED.gallery,
                        tags = EXCLUDED.tags,
                        edited = NOW()
                    RETURNING
                        id,
                        name,
                        slug,
                        event_date,
                        event_time,
                        event_artist_slug,
                        venue_name,
                        continent,
                        country,
                        region,
                        city,
                        ticket_link,
                        gallery,
                        tags,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY HH12:MI AM'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY HH12:MI AM'))
                        ) as edited
                ",
                event_details.id,
                event_details.name,
                event_details.slug,
                event_details.event_date,
                event_details.event_time,
                event_details.event_artist_slug,
                event_details.venue_name,
                event_details.continent,
                event_details.country,
                event_details.region,
                event_details.city,
                event_details.ticket_link,
                event_details.gallery.as_slice(),
                event_details.tags
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

#[delete("/event_details")]
async fn delete_event_details(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                EventDetails,
                "DELETE FROM event_details WHERE id = $1;",
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
