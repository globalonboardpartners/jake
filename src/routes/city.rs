use crate::data_types::structs::{City, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/city")]
async fn create_city(city: Json<City>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<City, Error> = sqlx::query_as!(
                City,
                "
                    INSERT INTO city
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
                            featured_city,
                            getting_around,
                            food_and_drink,
                            facts,
                            hotel,
                            restaurant,
                            attraction,
                            shopping,
                            continent,
                            country,
                            region,
                            latitude,
                            longitude
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
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
                        featured_city,
                        getting_around,
                        food_and_drink,
                        facts,
                        hotel,
                        restaurant,
                        attraction,
                        shopping,
                        continent,
                        country,
                        region,
                        latitude,
                        longitude,
                        (
                            SELECT json_agg(tg)
                            FROM (
                                SELECT
                                    t.name,
                                    t.description
                                FROM tag t
                                INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                WHERE trt.assoc_table_row_id = city.id AND trt.assoc_table = 'city'
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
                city.name,
                city.slug,
                city.description_long,
                city.description_short,
                city.image_link,
                city.thumbnail_link,
                city.special_offer_image_link,
                city.video_link,
                city.gallery.as_slice(),
                city.featured_city,
                city.getting_around.as_slice(),
                city.food_and_drink.as_slice(),
                city.facts.as_slice(),
                city.hotel.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.restaurant.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.attraction.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.shopping.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.continent,
                city.country,
                city.region,
                city.latitude,
                city.longitude,
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

#[get("/city")]
async fn get_city_by_id_or_all(Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect().await {
            Ok(pg) => {
                let returned: Result<City, Error> = sqlx::query_as!(
                    City,
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
                            featured_city,
                            getting_around,
                            food_and_drink,
                            facts,
                            hotel,
                            restaurant,
                            attraction,
                            shopping,
                            continent,
                            country,
                            region,
                            latitude,
                            longitude,
                            (
                                SELECT json_agg(tg)
                                FROM (
                                    SELECT
                                        t.name,
                                        t.description
                                    FROM tag t
                                    INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                    WHERE trt.assoc_table_row_id = city.id AND trt.assoc_table = 'city'
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
                        FROM city
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
                let returned: Result<Vec<City>, Error> = sqlx::query_as!(
                    City,
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
                            featured_city,
                            getting_around,
                            food_and_drink,
                            facts,
                            hotel,
                            restaurant,
                            attraction,
                            shopping,
                            continent,
                            country,
                            region,
                            latitude,
                            longitude,
                            (
                                SELECT json_agg(tg)
                                FROM (
                                    SELECT
                                        t.name,
                                        t.description
                                    FROM tag t
                                    INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                    WHERE trt.assoc_table_row_id = city.id AND trt.assoc_table = 'city'
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
                        FROM city
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

#[put("/city")]
async fn update_city(city: Json<City>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<City, Error> = sqlx::query_as!(
                City,
                "
                    INSERT INTO city
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
                            featured_city,
                            getting_around,
                            food_and_drink,
                            facts,
                            hotel,
                            restaurant,
                            attraction,
                            shopping,
                            continent,
                            country,
                            region,
                            latitude,
                            longitude
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
                        thumbnail_link = EXCLUDED.thumbnail_link,
                        special_offer_image_link = EXCLUDED.special_offer_image_link,
                        video_link = EXCLUDED.video_link,
                        gallery = EXCLUDED.gallery,
                        featured_city = EXCLUDED.featured_city,
                        getting_around = EXCLUDED.getting_around,
                        food_and_drink = EXCLUDED.food_and_drink,
                        facts = EXCLUDED.facts,
                        hotel = EXCLUDED.hotel,
                        restaurant = EXCLUDED.restaurant,
                        attraction = EXCLUDED.attraction,
                        shopping = EXCLUDED.shopping,
                        continent = EXCLUDED.continent,
                        country = EXCLUDED.country,
                        region = EXCLUDED.region,
                        latitude = EXCLUDED.latitude,
                        longitude = EXCLUDED.longitude,
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
                        featured_city,
                        getting_around,
                        food_and_drink,
                        facts,
                        hotel,
                        restaurant,
                        attraction,
                        shopping,
                        continent,
                        country,
                        region,
                        latitude,
                        longitude,
                        (
                            SELECT json_agg(tg)
                            FROM (
                                SELECT
                                    t.name,
                                    t.description
                                FROM tag t
                                INNER JOIN table_row_tags trt ON t.id = trt.tag_id
                                WHERE trt.assoc_table_row_id = city.id AND trt.assoc_table = 'city'
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
                city.id,
                city.name,
                city.slug,
                city.description_long,
                city.description_short,
                city.image_link,
                city.thumbnail_link,
                city.special_offer_image_link,
                city.video_link,
                city.gallery.as_slice(),
                city.featured_city,
                city.getting_around.as_slice(),
                city.food_and_drink.as_slice(),
                city.facts.as_slice(),
                city.hotel.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.restaurant.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.attraction.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.shopping.as_ref().map_or(&[][..], |v| v.as_slice()),
                city.continent,
                city.country,
                city.region,
                city.latitude,
                city.longitude
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

#[delete("/city")]
async fn delete_city(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                City,
                "DELETE FROM city WHERE id = $1;",
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
