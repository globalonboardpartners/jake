use crate::data_types::structs::{Client, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;

#[post("/client")]
async fn create_client(req: HttpRequest, client: Json<Client>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Client, Error> = sqlx::query_as!(
                Client,
                "
                    INSERT INTO client
                        (
                            name,
                            slug,
                            title,
                            description_short,
                            description_long,
                            logo,
                            image_link,
                            quote,
                            quote_author,
                            quote_author_position,
                            number_of_employees,
                            industry,
                            website_link,
                            features_used,
                            featured
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                    RETURNING
                        id,
                        name,
                        slug,
                        title,
                        description_short,
                        description_long,
                        logo,
                        image_link,
                        quote,
                        quote_author,
                        quote_author_position,
                        number_of_employees,
                        industry,
                        website_link,
                        features_used,
                        featured,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY'))
                        ) as edited
                ",
                client.name,
                client.slug,
                client.title,
                client.description_short,
                client.description_long,
                client.logo,
                client.image_link,
                client.quote,
                client.quote_author,
                client.quote_author_position,
                client.number_of_employees,
                client.industry,
                client.website_link,
                client.features_used,
                client.featured,
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

#[get("/client")]
async fn get_client_by_id_or_all(req: HttpRequest, Query(id): Query<Id>) -> HttpResponse {
    if id.id.is_some() {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Client, Error> = sqlx::query_as!(
                    Client,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            title,
                            description_short,
                            description_long,
                            logo,
                            image_link,
                            quote,
                            quote_author,
                            quote_author_position,
                            number_of_employees,
                            industry,
                            website_link,
                            features_used,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY'))
                            ) as edited
                        FROM client
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
                let returned: Result<Vec<Client>, Error> = sqlx::query_as!(
                    Client,
                    "
                        SELECT
                            id,
                            name,
                            slug,
                            title,
                            description_short,
                            description_long,
                            logo,
                            image_link,
                            quote,
                            quote_author,
                            quote_author_position,
                            number_of_employees,
                            industry,
                            website_link,
                            features_used,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY'))
                            ) as edited
                        FROM client
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

#[put("/client")]
async fn update_client(req: HttpRequest, client: Json<Client>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Client, Error> = sqlx::query_as!(
                Client,
                "
                    INSERT INTO client
                        (
                            id,
                            name,
                            slug,
                            title,
                            description_short,
                            description_long,
                            logo,
                            image_link,
                            quote,
                            quote_author,
                            quote_author_position,
                            number_of_employees,
                            industry,
                            website_link,
                            features_used,
                            featured
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                        id = EXCLUDED.id,
                        name = EXCLUDED.name,
                        slug = EXCLUDED.slug,
                        title = EXCLUDED.title,
                        description_short = EXCLUDED.description_short,
                        description_long = EXCLUDED.description_long,
                        logo = EXCLUDED.logo,
                        image_link = EXCLUDED.image_link,
                        quote = EXCLUDED.quote,
                        quote_author = EXCLUDED.quote_author,
                        quote_author_position = EXCLUDED.quote_author_position,
                        number_of_employees = EXCLUDED.number_of_employees,
                        industry = EXCLUDED.industry,
                        website_link = EXCLUDED.website_link,
                        features_used = EXCLUDED.features_used,
                        featured = EXCLUDED.featured
                    RETURNING
                        id,
                        name,
                        slug,
                        title,
                        description_short,
                        description_long,
                        logo,
                        image_link,
                        quote,
                        quote_author,
                        quote_author_position,
                        number_of_employees,
                        industry,
                        website_link,
                        features_used,
                        featured,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY'))
                        ) as edited
                ",
                client.id,
                client.name,
                client.slug,
                client.title,
                client.description_short,
                client.description_long,
                client.logo,
                client.image_link,
                client.quote,
                client.quote_author,
                client.quote_author_position,
                client.number_of_employees,
                client.industry,
                client.website_link,
                client.features_used,
                client.featured,
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

#[delete("/client")]
async fn delete_client(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =
                sqlx::query_as!(Client, "DELETE FROM client WHERE id = $1;", id.id)
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
