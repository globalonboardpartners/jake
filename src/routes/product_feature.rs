use actix_web::{get, post, put, delete, http, HttpResponse};
use actix_web::web::Json;
use crate::utils::handle_sql_error;
use crate::db;
use crate::data_types::structs::{Id, ProductFeature};
use sqlx::Error;
use actix_web::http::StatusCode;
use sqlx::postgres::PgQueryResult;

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
                            description
                        )
                    VALUES ($1, $2)
                    RETURNING *;
                ",
                product_feature.title,
                product_feature.description,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Created()
                        .status(StatusCode::CREATED)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

#[get("/product_feature")]
async fn get_all_product_features() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<ProductFeature>, Error> = sqlx::query_as!(
                ProductFeature,
                "SELECT * from product_feature;"
            )
            .fetch_all(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
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
                Ok(record) => {
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
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
                            description
                        )
                    VALUES ($1, $2, $3)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                    id = EXCLUDED.id, title = EXCLUDED.title, description = EXCLUDED.description
                    RETURNING *;
                ",
                product_feature.id,
                product_feature.title,
                product_feature.description,
            )
            .fetch_one(&pg)
            .await;

            match returned {
                Ok(record) => {
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json") 
                        .body(serde_json::to_string(&Json(record)).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
                    
                },

                Err(e) => handle_sql_error(e)
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
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
                Ok(_) => {
                    HttpResponse::NoContent()
                        .status(StatusCode::NO_CONTENT)
                        .content_type("application/json") 
                        .finish()
                },

                Err(e) => handle_sql_error(e)
            }
        },

        Err(e) => {
            HttpResponse::InternalServerError()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json") 
                .body(e)
        }
    }
}

