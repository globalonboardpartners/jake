use crate::data_types::structs::{Blog, Id};
use crate::db;
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, http, post, put, web::Query, HttpRequest, HttpResponse};
use sqlx::postgres::PgQueryResult;
use sqlx::Error;
use log::{info, error};
use actix_session::Session;
use actix_web::cookie::Cookie;

#[post("/blog")]
async fn create_blog(req: HttpRequest, blog: Json<Blog>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                    WITH new_row AS (
                        INSERT INTO blog
                            (
                                title,
                                slug,
                                category_id,
                                content,
                                image_link,
                                thumbnail_link,
                                featured
                            )
                        VALUES (
                            $1,
                            $2,
                            (SELECT id FROM blog_category WHERE category = $3),
                            $4,
                            $5,
                            $6,
                            $7
                        )
                        RETURNING
                            id,
                            title,
                            slug,
                            category_id,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
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
                    )
                    SELECT
                        new_row.id,
                        new_row.title,
                        new_row.slug,
                        new_row.content,
                        new_row.image_link,
                        new_row.thumbnail_link,
                        new_row.featured,
                        new_row.created,
                        new_row.edited,
                        (SELECT category FROM blog_category WHERE id = new_row.category_id) AS "category!"
                    FROM new_row
                "#,
                blog.title,
                blog.slug,
                blog.category,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured
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

#[get("/blog/featured")]
async fn get_featured_blogs(req: HttpRequest) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                Blog,
                r#"
                    SELECT
                        id,
                        title,
                        slug,
                        content,
                        image_link,
                        thumbnail_link,
                        featured,
                        (
	                        trim(to_char(created, 'DD')) || ' ' ||
	                        trim(to_char(created, 'Month')) || ' ' ||
	                        trim(to_char(created, 'YYYY HH12:MI AM'))
                        ) as created,
                        (
	                        trim(to_char(edited, 'DD')) || ' ' ||
	                        trim(to_char(edited, 'Month')) || ' ' ||
	                        trim(to_char(edited, 'YYYY HH12:MI AM'))
                        ) as edited,
                        (SELECT category FROM blog_category WHERE id = category_id) AS "category!"
                    FROM blog
                    WHERE featured = TRUE
                    LIMIT 2;
                "#
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

#[get("/blog")]
async fn get_blog_by_id_or_all(session: Session, req: HttpRequest, Query(id): Query<Id>) -> HttpResponse {

    let cookie = Cookie::new("cookie-test", "cookie-value");

    match session.get::<String>("my-kata-cookie") {
        Ok(value) => {
            dbg!(&value);
            info!("{}", value.unwrap())
        },
        Err(e) => {
            dbg!(&e);
            error!("No session data found for key_name: {}", e)
        },
    }

    if id.id.is_some() {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Blog, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                        SELECT
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited,
                            (SELECT category FROM blog_category WHERE id = category_id) AS "category!"
                        FROM blog
                        WHERE id = $1
                        LIMIT 1;
                    "#,
                    id.id
                )
                .fetch_one(&pg)
                .await;

                match returned {
                    Ok(record) => HttpResponse::Ok()
                        .cookie(Cookie::new("my_cookie", "cookie_value"))
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
                .cookie(cookie)
                .body(e.message),
        }
    } else {
        match db::connect(req).await {
            Ok(pg) => {
                let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                    Blog,
                    r#"
                        SELECT
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            (
	                            trim(to_char(created, 'DD')) || ' ' ||
	                            trim(to_char(created, 'Month')) || ' ' ||
	                            trim(to_char(created, 'YYYY HH12:MI AM'))
                            ) as created,
                            (
	                            trim(to_char(edited, 'DD')) || ' ' ||
	                            trim(to_char(edited, 'Month')) || ' ' ||
	                            trim(to_char(edited, 'YYYY HH12:MI AM'))
                            ) as edited,
                            (SELECT category FROM blog_category WHERE id = category_id) AS "category!"
                        FROM blog;
                    "#,
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

#[put("/blog")]
async fn update_blog(req: HttpRequest, blog: Json<Blog>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                r#"
                    WITH new_row AS (
                        INSERT INTO blog (id, title, slug, category_id, content, image_link, thumbnail_link, featured)
                        VALUES (
                            $1, $2, $3, (SELECT id FROM blog_category WHERE category = $4), $5, $6, $7, $8
                        )
                        ON CONFLICT (id)
                        DO UPDATE SET
                            id = EXCLUDED.id,
                            title = EXCLUDED.title,
                            slug = EXCLUDED.slug,
                            category_id = EXCLUDED.category_id,
                            content = EXCLUDED.content,
                            image_link = EXCLUDED.image_link,
                            thumbnail_link = EXCLUDED.thumbnail_link,
                            featured = EXCLUDED.featured,
                            edited = NOW()
                        RETURNING
                            id,
                            title,
                            slug,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            category_id,
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
                    )
                    SELECT
                        new_row.id,
                        new_row.title,
                        new_row.slug,
                        new_row.content,
                        new_row.image_link,
                        new_row.thumbnail_link,
                        new_row.featured,
                        new_row.created,
                        new_row.edited,
                        (SELECT category FROM blog_category WHERE id = new_row.category_id) AS "category!"
                    FROM new_row
                "#,
                blog.id,
                blog.title,
                blog.slug,
                blog.category,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured,
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

#[delete("/blog")]
async fn delete_blog(req: HttpRequest, id: Json<Id>) -> HttpResponse {
    match db::connect(req).await {
        Ok(pg) => {
            let returned: Result<PgQueryResult, Error> =
                sqlx::query_as!(Blog, "DELETE FROM blog WHERE id = $1;", id.id)
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
