use crate::data_types::structs::{Activity, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/activity")]
async fn create_activity(activity: Json<Activity>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Activity, Error> = sqlx::query_as!(
                Activity,
                "
                    INSERT INTO activity
                        (
                            name,
                            slug,
                            description_short,
                            description_long,
                            video_link,
                            image_link,
                            image_link_2,
                            thumbnail_link,
                            gallery,
                            featured_activity,
                            partner_vendor,
                            continent,
                            country,
                            region,
                            city,
                            latitude,
                            longitude,
                            email,
                            phone,
                            address,
                            website_link
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
                    RETURNING
                        id,
                        name,
                        slug,
                        description_short,
                        description_long,
                        video_link,
                        image_link,
                        image_link_2,
                        thumbnail_link,
                        gallery,
                        featured_activity,
                        partner_vendor,
                        continent,
                        country,
                        region,
                        city,
                        latitude,
                        longitude,
                        email,
                        phone,
                        address,
                        website_link,
                        (
                            SELECT json_agg(tg)
                            FROM (
                                SELECT
                                    t.name,
                                    t.description
                                FROM tag t
                                INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                WHERE trt.assoc_table_row_id = activity.id AND trt.assoc_table = 'activity'
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
                activity.name,
                activity.slug,
                activity.description_short,
                activity.description_long,
                activity.video_link,
                activity.image_link,
                activity.image_link_2,
                activity.thumbnail_link,
                activity.gallery.as_slice(),
                activity.featured_activity,
                activity.partner_vendor,
                activity.continent,
                activity.country,
                activity.region,
                activity.city,
                activity.latitude,
                activity.longitude,
                activity.email,
                activity.phone,
                activity.address,
                activity.website_link,
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

#[get("/activity")]
async fn get_activity_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Activity, Error> = sqlx::query_as!(
                    Activity,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            description_short,
                            description_long,
                            video_link,
                            image_link,
                            image_link_2,
                            thumbnail_link,
                            gallery,
                            featured_activity,
                            partner_vendor,
                            continent,
                            country,
                            region,
                            city,
                            latitude,
                            longitude,
                            email,
                            phone,
                            address,
                            website_link,
                            (
                                SELECT json_agg(tg)
                                FROM (
                                    SELECT
                                        t.name,
                                        t.description
                                    FROM tag t
                                    INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                    WHERE trt.assoc_table_row_id = activity.id AND trt.assoc_table = 'activity'
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
                        FROM activity
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
                let returned: Result<Vec<Activity>, Error> = sqlx::query_as!(
                    Activity,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            description_short,
                            description_long,
                            video_link,
                            image_link,
                            image_link_2,
                            thumbnail_link,
                            gallery,
                            featured_activity,
                            partner_vendor,
                            continent,
                            country,
                            region,
                            city,
                            latitude,
                            longitude,
                            email,
                            phone,
                            address,
                            website_link,
                            (
                                SELECT json_agg(tg)
                                FROM (
                                    SELECT
                                        t.name,
                                        t.description
                                    FROM tag t
                                    INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                    WHERE trt.assoc_table_row_id = activity.id AND trt.assoc_table = 'activity'
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
                        FROM activity
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

#[put("/activity")]
async fn update_activity(activity: Json<Activity>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Activity, Error> = sqlx::query_as!(
                Activity,
                "
                    INSERT INTO activity
                        (
                            id,
                            name,
                            slug,
                            description_short,
                            description_long,
                            video_link,
                            image_link,
                            image_link_2,
                            thumbnail_link,
                            gallery,
                            featured_activity,
                            partner_vendor,
                            continent,
                            country,
                            region,
                            city,
                            latitude,
                            longitude,
                            email,
                            phone,
                            address,
                            website_link
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug,
                        description_long = EXCLUDED.description_long,
                        description_short = EXCLUDED.description_short,
                        image_link = EXCLUDED.image_link,
                        image_link_2 = EXCLUDED.image_link_2,
                        video_link = EXCLUDED.video_link,
                        thumbnail_link = EXCLUDED.thumbnail_link,
                        gallery = EXCLUDED.gallery,
                        featured_activity = EXCLUDED.featured_activity,
                        partner_vendor = EXCLUDED.partner_vendor,
                        continent = EXCLUDED.continent,
                        country = EXCLUDED.country,
                        region = EXCLUDED.region,
                        city = EXCLUDED.city,
                        latitude = EXCLUDED.latitude,
                        longitude = EXCLUDED.longitude,
                        email = EXCLUDED.email,
                        phone = EXCLUDED.phone,
                        address = EXCLUDED.address,
                        website_link = EXCLUDED.website_link,
                        edited = NOW()
                    RETURNING
                        id,
                        name,
                        slug,
                        description_short,
                        description_long,
                        video_link,
                        image_link,
                        image_link_2,
                        thumbnail_link,
                        gallery,
                        featured_activity,
                        partner_vendor,
                        continent,
                        country,
                        region,
                        city,
                        latitude,
                        longitude,
                        email,
                        phone,
                        address,
                        website_link,
                        (
                            SELECT json_agg(tg)
                            FROM (
                                SELECT
                                    t.name,
                                    t.description
                                FROM tag t
                                INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                WHERE trt.assoc_table_row_id = activity.id AND trt.assoc_table = 'activity'
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
                activity.id,
                activity.name,
                activity.slug,
                activity.description_short,
                activity.description_long,
                activity.video_link,
                activity.image_link,
                activity.image_link_2,
                activity.thumbnail_link,
                activity.gallery.as_slice(),
                activity.featured_activity,
                activity.partner_vendor,
                activity.continent,
                activity.country,
                activity.region,
                activity.city,
                activity.latitude,
                activity.longitude,
                activity.email,
                activity.phone,
                activity.address,
                activity.website_link,
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

#[delete("/activity")]
async fn delete_activity(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                activity,
                "DELETE FROM activity WHERE id = $1;",
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
