use crate::data_types::structs::{Country, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/country")]
async fn create_country(country: Json<Country>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Country, Error> = sqlx::query_as!(
                Country,
                "
                    INSERT INTO country
                        (
                            name,
                            slug,
                            description_long,
                            description_short,
                            image_link,
                            thumbnail_link,
                            special_offer_image_link,
                            video_link,
                            gallery
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
                        (
                            SELECT json_agg(tg)
                            FROM (
                                SELECT
                                    t.name,
                                    t.description
                                FROM tag t
                                INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                WHERE trt.assoc_table_row_id = country.id AND trt.assoc_table = 'country'
                            ) as tg
                        ) as tags,
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
                country.name,
                country.slug,
                country.description_long,
                country.description_short,
                country.image_link,
                country.thumbnail_link,
                country.special_offer_image_link,
                country.video_link,
                country.gallery.as_slice(),
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

#[get("/country")]
async fn get_country_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Country, Error> = sqlx::query_as!(
                    Country,
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
                            (
                                SELECT json_agg(tg)
                                FROM (
                                    SELECT
                                        t.name,
                                        t.description
                                    FROM tag t
                                    INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                    WHERE trt.assoc_table_row_id = country.id AND trt.assoc_table = 'country'
                                ) as tg
                            ) as tags,
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
                        FROM country
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
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Vec<Country>, Error> = sqlx::query_as!(
                    Country,
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
                            (
                                SELECT json_agg(tg)
                                FROM (
                                    SELECT
                                        t.name,
                                        t.description
                                    FROM tag t
                                    INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                    WHERE trt.assoc_table_row_id = country.id AND trt.assoc_table = 'country'
                                ) as tg
                            ) as tags,
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
                        FROM country
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

#[put("/country")]
async fn update_country(country: Json<Country>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Country, Error> = sqlx::query_as!(
                Country,
                "
                    INSERT INTO country
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
                            gallery
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
                        (
                            SELECT json_agg(tg)
                            FROM (
                                SELECT
                                    t.name,
                                    t.description
                                FROM tag t
                                INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                WHERE trt.assoc_table_row_id = country.id AND trt.assoc_table = 'country'
                            ) as tg
                        ) as tags,
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
                country.id,
                country.name,
                country.slug,
                country.description_long,
                country.description_short,
                country.image_link,
                country.thumbnail_link,
                country.special_offer_image_link,
                country.video_link,
                country.gallery.as_slice(),
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

#[delete("/country")]
async fn delete_country(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                Country,
                "DELETE FROM country WHERE id = $1;",
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
