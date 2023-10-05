use crate::data_types::structs::{Continent, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/continent")]
async fn create_continent(continent: Json<Continent>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Continent, Error> = sqlx::query_as!(
                Continent,
                "
                    INSERT INTO continent
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
                                WHERE trt.assoc_table_row_id = continent.id AND trt.assoc_table = 'continent'
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
                continent.name,
                continent.slug,
                continent.description_long,
                continent.description_short,
                continent.image_link,
                continent.thumbnail_link,
                continent.special_offer_image_link,
                continent.video_link,
                continent.gallery.as_slice(),
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

#[get("/continent")]
async fn get_continent_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Continent, Error> = sqlx::query_as!(
                    Continent,
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
                                    WHERE trt.assoc_table_row_id = continent.id AND trt.assoc_table = 'continent'
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
                        FROM continent
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
                let returned: Result<Vec<Continent>, Error> = sqlx::query_as!(
                    Continent,
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
                                    WHERE trt.assoc_table_row_id = continent.id AND trt.assoc_table = 'continent'
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
                        FROM continent
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

#[put("/continent")]
async fn update_continent(continent: Json<Continent>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Continent, Error> = sqlx::query_as!(
                Continent,
                "
                    INSERT INTO continent
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
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug,
                        description_long = EXCLUDED.description_long,
                        description_short = EXCLUDED.description_short,
                        image_link = EXCLUDED.image_link,
                        thumbnail_link = EXCLUDED.thumbnail_link,
                        special_offer_image_link = EXCLUDED.special_offer_image_link,
                        video_link = EXCLUDED.video_link,
                        gallery = EXCLUDED.gallery
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
                                WHERE trt.assoc_table_row_id = continent.id AND trt.assoc_table = 'continent'
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
                continent.id,
                continent.name,
                continent.slug,
                continent.description_long,
                continent.description_short,
                continent.image_link,
                continent.thumbnail_link,
                continent.special_offer_image_link,
                continent.video_link,
                continent.gallery.as_slice(),
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

#[delete("/continent")]
async fn delete_continent(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                Continent,
                "DELETE FROM continent WHERE id = $1;",
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
