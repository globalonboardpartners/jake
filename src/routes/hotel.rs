use crate::data_types::structs::{Hotel, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/hotel")]
async fn create_hotel(hotel: Json<Hotel>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Hotel, Error> = sqlx::query_as!(
                Hotel,
                "
                    INSERT INTO hotel
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
                            featured_hotel,
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
                            tags
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
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
                        featured_hotel,
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
                hotel.name,
                hotel.slug,
                hotel.description_short,
                hotel.description_long,
                hotel.video_link,
                hotel.image_link,
                hotel.image_link_2,
                hotel.thumbnail_link,
                hotel.gallery.as_slice(),
                hotel.featured_hotel,
                hotel.partner_vendor,
                hotel.continent,
                hotel.country,
                hotel.region,
                hotel.city,
                hotel.latitude,
                hotel.longitude,
                hotel.email,
                hotel.phone,
                hotel.address,
                hotel.website_link,
                hotel.tags
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

#[get("/hotel")]
async fn get_hotel_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<Hotel, Error> = sqlx::query_as!(
                    Hotel,
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
                            featured_hotel,
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
                        FROM hotel
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
                let returned: Result<Vec<Hotel>, Error> = sqlx::query_as!(
                    Hotel,
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
                            featured_hotel,
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
                        FROM hotel
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

#[put("/hotel")]
async fn update_hotel(hotel: Json<Hotel>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Hotel, Error> = sqlx::query_as!(
                Hotel,
                "
                    INSERT INTO hotel
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
                            featured_hotel,
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
                            tags
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
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
                        featured_hotel = EXCLUDED.featured_hotel,
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
                        tags = EXCLUDED.tags,
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
                        featured_hotel,
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
                hotel.id,
                hotel.name,
                hotel.slug,
                hotel.description_short,
                hotel.description_long,
                hotel.video_link,
                hotel.image_link,
                hotel.image_link_2,
                hotel.thumbnail_link,
                hotel.gallery.as_slice(),
                hotel.featured_hotel,
                hotel.partner_vendor,
                hotel.continent,
                hotel.country,
                hotel.region,
                hotel.city,
                hotel.latitude,
                hotel.longitude,
                hotel.email,
                hotel.phone,
                hotel.address,
                hotel.website_link,
                hotel.tags
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

#[delete("/hotel")]
async fn delete_hotel(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                Hotel,
                "DELETE FROM hotel WHERE id = $1;",
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
