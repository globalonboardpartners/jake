use crate::data_types::structs::{PartnerVendor, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/partner_vendor")]
async fn create_partner_vendor(partner_vendor: Json<PartnerVendor>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PartnerVendor, Error> = sqlx::query_as!(
                PartnerVendor,
                r#"
                    INSERT INTO partner_vendor
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
                            featured_partner_vendor,
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
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
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
                        featured_partner_vendor,
                        continent,
                        country,
                        region,
                        city,
                        latitude,
                        longitude,
                        email,
                        phone,
                        address,
                        tags,
                        website_link,
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
                "#,
                partner_vendor.name,
                partner_vendor.slug,
                partner_vendor.description_short,
                partner_vendor.description_long,
                partner_vendor.video_link,
                partner_vendor.image_link,
                partner_vendor.image_link_2,
                partner_vendor.thumbnail_link,
                partner_vendor.gallery.as_slice(),
                partner_vendor.featured_partner_vendor,
                partner_vendor.continent,
                partner_vendor.country,
                partner_vendor.region,
                partner_vendor.city,
                partner_vendor.latitude,
                partner_vendor.longitude,
                partner_vendor.email,
                partner_vendor.phone,
                partner_vendor.address,
                partner_vendor.website_link,
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

#[get("/partner_vendor")]
async fn get_partner_vendor_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<PartnerVendor, Error> = sqlx::query_as!(
                    PartnerVendor,
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
                            featured_partner_vendor,
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
                        FROM partner_vendor
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
                let returned: Result<Vec<PartnerVendor>, Error> = sqlx::query_as!(
                    PartnerVendor,
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
                            featured_partner_vendor,
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
                        FROM partner_vendor
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

#[put("/partner_vendor")]
async fn update_partner_vendor(partner_vendor: Json<PartnerVendor>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PartnerVendor, Error> = sqlx::query_as!(
                PartnerVendor,
                "
                    INSERT INTO partner_vendor
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
                            featured_partner_vendor,
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
                        featured_partner_vendor = EXCLUDED.featured_partner_vendor,
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
                        featured_partner_vendor,
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
                partner_vendor.id,
                partner_vendor.name,
                partner_vendor.slug,
                partner_vendor.description_short,
                partner_vendor.description_long,
                partner_vendor.video_link,
                partner_vendor.image_link,
                partner_vendor.image_link_2,
                partner_vendor.thumbnail_link,
                partner_vendor.gallery.as_slice(),
                partner_vendor.featured_partner_vendor,
                partner_vendor.continent,
                partner_vendor.country,
                partner_vendor.region,
                partner_vendor.city,
                partner_vendor.latitude,
                partner_vendor.longitude,
                partner_vendor.email,
                partner_vendor.phone,
                partner_vendor.address,
                partner_vendor.website_link,
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

#[delete("/partner_vendor")]
async fn delete_partner_vendor(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                PartnerVendor,
                "DELETE FROM partner_vendor WHERE id = $1;",
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
