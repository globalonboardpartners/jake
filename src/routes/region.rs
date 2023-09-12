use crate::data_types::structs::{Region, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/region")]
async fn create_region(req: HttpRequest, region: Json<Region>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Region, Error> = sqlx::query_as!(
                Region,
                "
                    INSERT INTO region
                        (
                            name,
                            slug,
                            description_long,
                            description_short,
                            image_link,
                            thumbnail_link,
                            special_offer_image_link,
                            video_link,
                            gallery,
                            tags
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                    RETURNING
                        id,
                        name,
                        slug,
                        description_long,
                        description_short,
                        image_link,
                        thumbnail_link,
                        special_offer_image_link,
                        video_link,
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
                region.name,
                region.slug,
                region.description_long,
                region.description_short,
                region.image_link,
                region.thumbnail_link,
                region.special_offer_image_link,
                region.video_link,
                region.gallery.as_slice(),
                region.tags
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

#[get("/region")]
async fn get_region_by_id_or_all(req: HttpRequest, Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Region, Error> = sqlx::query_as!(
                    Region,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            description_long,
                            description_short,
                            image_link,
                            thumbnail_link,
                            special_offer_image_link,
                            video_link,
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
                        FROM region
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
                let returned: Result<Vec<Region>, Error> = sqlx::query_as!(
                    Region,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            description_long,
                            description_short,
                            image_link,
                            thumbnail_link,
                            special_offer_image_link,
                            video_link,
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
                        FROM region
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

#[put("/region")]
async fn update_region(req: HttpRequest, region: Json<Region>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Region, Error> = sqlx::query_as!(
                Region,
                "
                    INSERT INTO region
                        (
                            id,
                            name,
                            slug,
                            description_long,
                            description_short,
                            image_link,
                            thumbnail_link,
                            special_offer_image_link,
                            video_link,
                            gallery,
                            tags
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug,
                        description_long = EXCLUDED.description_long,
                        description_short = EXCLUDED.description_short,
                        image_link = EXCLUDED.image_link,
                        thumbnail_link = EXCLUDED.thumbnail_link,
                        special_offer_image_link = EXCLUDED.special_offer_image_link,
                        video_link = EXCLUDED.video_link,
                        gallery = EXCLUDED.gallery,
                        tags = EXCLUDED.tags,
                        edited = NOW()
                    RETURNING
                        id,
                        name,
                        slug,
                        description_long,
                        description_short,
                        image_link,
                        thumbnail_link,
                        special_offer_image_link,
                        video_link,
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
                region.id,
                region.name,
                region.slug,
                region.description_long,
                region.description_short,
                region.image_link,
                region.thumbnail_link,
                region.special_offer_image_link,
                region.video_link,
                region.gallery.as_slice(),
                region.tags
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

#[delete("/region")]
async fn delete_region(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                region,
                "DELETE FROM region WHERE id = $1;",
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
