use actix_web::{get, post, put, delete, http, web, Responder, dev::HttpServiceFactory, HttpResponse};
use actix_web::web::Json;
use crate::utils::handle_request_error as handle_sql_error;
use crate::db2;
use crate::data_types::structs::{Id, UpdateColumn, BlogCategory, Employee, Blog, JobListing, ProductFeature, Continent};
use sqlx::Error;
use actix_web::http::StatusCode;
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono::Utc;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[post("/employee")]
async fn create_employee(employee: Json<Employee>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Employee, Error> = sqlx::query_as!(
                Employee,
                "
                    INSERT INTO employee
                        (
                            name,
                            position,
                            bio,
                            image_url
                        )
                    VALUES ($1, $2, $3, $4)
                    RETURNING
                        id,
                        name,
                        position,
                        bio,
                        image_url;
                ",
                employee.name,
                employee.position,
                employee.bio,
                employee.image_url,
            )
            .fetch_one(&db)
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

#[get("/employees")]
async fn get_all_employees() -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Vec<Employee>, Error> = sqlx::query_as!(
                Employee,
                "SELECT * from employee;"
            )
            .fetch_all(&db)
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

#[get("/employee")]
async fn get_employee_by_id(id: Json<Id>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Employee, Error> = sqlx::query_as!(
                Employee,
                "SELECT * FROM employee WHERE id = $1;",
                id.id
            )
            .fetch_one(&db)
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

#[put("/employee")]
async fn update_employee(employee: Json<Employee>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Employee, Error> = sqlx::query_as!(
                Employee,
                "
                    INSERT INTO employee (id, name, position, bio, image_url)
                    VALUES ($5, $1, $2, $3, $4)
                    ON CONFLICT (id)
                    DO UPDATE SET 
                    name = EXCLUDED.name, position = EXCLUDED.position, bio = EXCLUDED.bio, image_url = EXCLUDED.image_url
                    RETURNING *;
                ",
                employee.name,
                employee.position,
                employee.bio,
                employee.image_url,
                employee.id,
            )
            .fetch_one(&db)
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

#[delete("/employee")]
async fn delete_employee(id: Json<Id>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                Employee,
                "DELETE FROM employee WHERE id = $1;",
                id.id
            )
            .execute(&db)
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

#[post("/blog")]
async fn create_blog(blog: Json<Blog>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                "
                    INSERT INTO blog
                        (
                            title,
                            slug,
                            category_id,
                            content,
                            image_link,
                            thumbnail_link,
                            featured,
                            publish_date
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING *;
                ",
                blog.title,
                blog.slug,
                blog.category_id,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured,
                Utc::now().naive_utc(),
            )
            .fetch_one(&db)
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

#[get("/blogs")]
async fn get_all_blogs() -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Vec<Blog>, Error> = sqlx::query_as!(
                Blog,
                "SELECT * from blog;"
            )
            .fetch_all(&db)
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

#[get("/blog")]
async fn get_blog_by_id(id: Json<Id>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                "SELECT * FROM blog WHERE id = $1;",
                id.id
            )
            .fetch_one(&db)
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

#[put("/blog")]
async fn update_blog(blog: Json<Blog>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<Blog, Error> = sqlx::query_as!(
                Blog,
                "
                    INSERT INTO blog (id, title, slug, category_id, content, image_link, thumbnail_link, featured, publish_date)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
                        publish_date = EXCLUDED.publish_date
                    RETURNING *;
                ",
                blog.id,
                blog.title,
                blog.slug,
                blog.category_id,
                blog.content,
                blog.image_link,
                blog.thumbnail_link,
                blog.featured,
                blog.publish_date,
            )
            .fetch_one(&db)
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

#[delete("/blog")]
async fn delete_blog(id: Json<Id>) -> HttpResponse {
    let pg = db2::connect().await;

    match pg {
        Ok(db) => {
            let returned: Result<PgQueryResult, Error> = sqlx::query_as!(
                Blog,
                "DELETE FROM blog WHERE id = $1;",
                id.id
            )
            .execute(&db)
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

pub fn employee() -> impl HttpServiceFactory {
    (
        greet,
        get_all_employees,
        get_employee_by_id,
        delete_employee,
        create_employee,
        update_employee,

    )
}

pub fn blog() -> impl HttpServiceFactory {
    (
        create_blog,
        // get_all_blogs,
        // get_blog_by_id,
        // update_blog,
        // delete_blog,
    )
}

// pub fn job_listing() -> impl HttpServiceFactory {
//     (
//         create_job_listing,
//         get_all_job_listings,
//         get_job_listing_by_id,
//         update_job_listing,
//         delete_job_listing,
//     )
// }

// pub fn product_feature() -> impl HttpServiceFactory {
//     (
//         create_product_feature,
//         get_all_product_features,
//         get_product_feature_by_id,
//         update_product_feature,
//         delete_product_feature,
//     )
// }

// pub fn blog_category() -> impl HttpServiceFactory {
//     (
//         create_blog_category,
//         get_all_blog_categories,
//         get_blog_category_by_id,
//         update_blog_category,
//         // delete_blog_category,
//     )
// }

// pub fn continent() -> impl HttpServiceFactory {
//     (
//         create_continent,
//         get_all_continents,
//         get_continent_by_id,
//         update_continent,
//         delete_continent,
//     )
// }
