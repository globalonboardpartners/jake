use crate::data_types::structs::{Id, ProductFeature};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/product_feature")]
async fn create_product_feature(product_feature: Json<ProductFeature>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<ProductFeature, Error> = sqlx::query_as!(
                ProductFeature,
                "
                    INSERT INTO product_feature
                        (
                            title,
                            slug,
                            description_short,
                            description_long,
                            image_link,
                            video_link,
                            icon,
                            quote,
                            quote_author,
                            quote_author_position,
                            order_number
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    RETURNING *;
                ",
                product_feature.title,
                product_feature.slug,
                product_feature.description_short,
                product_feature.description_long,
                product_feature.image_link,
                product_feature.video_link,
                product_feature.icon,
                product_feature.quote,
                product_feature.quote_author,
                product_feature.quote_author_position,
                product_feature.order_number,
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
            .body(e),
    }
}

#[get("/product_feature")]
async fn get_all_product_features() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<ProductFeature>, Error> =
                sqlx::query_as!(ProductFeature, "SELECT * from product_feature;")
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
            .body(e),
    }
}

#[get("/product_feature")]
async fn get_product_feature_by_id(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<ProductFeature, Error> = sqlx::query_as!(
                ProductFeature,
                "SELECT * FROM product_feature WHERE id = $1;",
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
            .body(e),
    }
}

#[put("/product_feature")]
async fn update_product_feature(product_feature: Json<ProductFeature>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<ProductFeature, Error> = sqlx::query_as!(
                ProductFeature,
                "
                    INSERT INTO product_feature
                        (
                            id,
                            title,
                            slug,
                            description_short,
                            description_long,
                            image_link,
                            video_link,
                            icon,
                            quote,
                            quote_author,
                            quote_author_position,
                            order_number
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        title = EXCLUDED.title,
                        slug = EXCLUDED.slug,
                        description_short = EXCLUDED.description_short,
                        description_long = EXCLUDED.description_long,
                        image_link = EXCLUDED.image_link,
                        video_link = EXCLUDED.video_link,
                        icon = EXCLUDED.icon,
                        quote = EXCLUDED.quote,
                        quote_author = EXCLUDED.quote_author,
                        quote_author_position = EXCLUDED.quote_author_position,
                        order_number = EXCLUDED.order_number
                    RETURNING *;
                ",
                product_feature.id,
                product_feature.title,
                product_feature.slug,
                product_feature.description_short,
                product_feature.description_long,
                product_feature.image_link,
                product_feature.video_link,
                product_feature.icon,
                product_feature.quote,
                product_feature.quote_author,
                product_feature.quote_author_position,
                product_feature.order_number,
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
            .body(e),
    }
}

#[delete("/product_feature")]
async fn delete_product_feature(id: Json<Id>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                ProductFeature,
                "DELETE FROM product_feature WHERE id = $1;",
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
            .body(e),
    }
}
