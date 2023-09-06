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

// #[get("/employees")]
// async fn get_all_employees() -> HttpResponse {
//     let res = db2::get_all::<Employee>().await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// #[get("/employee")]
// async fn get_employee_by_id(id: Json<Id>) -> HttpResponse {
//     let res = db2::get_by_id::<Employee>(id.id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// #[put("/employee")]
// async fn update_employee(employee_update: Json<Employee>) -> HttpResponse {
//     let res = db2::update_by_id::<Employee>(employee_update.into_inner()).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json.rows_affected()).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// #[delete("/employee")]
// async fn delete_employee(id: Json<Id>) -> impl Responder {
//     let res = db2::delete_by_id::<Employee>(id.id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json.rows_affected()).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

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


// #[post("/blog")]
// async fn create_blog(blog: Json<Blog>) -> HttpResponse {
//     let res = db::create::<Blog>(blog).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Created()
//                 .status(http::StatusCode::CREATED)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/blogs")]
// async fn get_all_blogs() -> HttpResponse {
//     let res = db::get_all::<Blog>().await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/blog")]
// async fn get_blog_by_id(id: Json<Id>) -> impl Responder {
//     let res = db::get_by_id::<Blog>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[put("/blog")]
// async fn update_blog(blog_update: Json<UpdateColumn>) -> HttpResponse {
//     let res = db::update_by_id::<Blog>(blog_update).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[delete("/blog")]
// async fn delete_blog(id: Json<Id>) -> impl Responder {
//     let res = db::delete_by_id::<Blog>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[post("/job_listing")]
// async fn create_job_listing(job_listing: Json<JobListing>) -> HttpResponse {
//     let res = db::create::<JobListing>(job_listing).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Created()
//                 .status(http::StatusCode::CREATED)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/job_listings")]
// async fn get_all_job_listings() -> HttpResponse {
//     let res = db::get_all::<JobListing>().await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/job_listing")]
// async fn get_job_listing_by_id(id: Json<Id>) -> HttpResponse {
//     let res = db::get_by_id::<JobListing>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[put("/job_listing")]
// async fn update_job_listing(job_listing_update: Json<UpdateColumn>) -> HttpResponse {
//     let res = db::update_by_id::<JobListing>(job_listing_update).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[delete("/job_listing")]
// async fn delete_job_listing(id: Json<Id>) -> impl Responder {
//     let res = db::delete_by_id::<JobListing>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[post("/product_feature")]
// async fn create_product_feature(product_feature: Json<ProductFeature>) -> HttpResponse {
//     let res = db::create::<ProductFeature>(product_feature).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Created()
//                 .status(http::StatusCode::CREATED)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/product_features")]
// async fn get_all_product_features() -> HttpResponse {
//     let res = db::get_all::<ProductFeature>().await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/product_feature")]
// async fn get_product_feature_by_id(id: Json<Id>) -> HttpResponse {
//     let res = db::get_by_id::<ProductFeature>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[put("/product_feature")]
// async fn update_product_feature(product_feature_update: Json<UpdateColumn>) -> HttpResponse {
//     let res = db::update_by_id::<ProductFeature>(product_feature_update).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[delete("/product_feature")]
// async fn delete_product_feature(id: Json<Id>) -> HttpResponse {
//     let res = db2::delete_by_id::<ProductFeature>(id.id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json.rows_affected()).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// #[post("/blog_category")]
// async fn create_blog_category(blog_category: Json<BlogCategory>) -> HttpResponse {
//     let res = db::create::<BlogCategory>(blog_category).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Created()
//                 .status(http::StatusCode::CREATED)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/blog_categories")]
// async fn get_all_blog_categories() -> HttpResponse {
//     let res = db2::get_all::<BlogCategory>().await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// #[get("/blog_category")]
// async fn get_blog_category_by_id(id: Json<Id>) -> HttpResponse {
//     let res = db2::get_by_id::<BlogCategory>(id.id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// #[put("/blog_category")]
// async fn update_blog_category(blog_category_update: Json<BlogCategory>) -> HttpResponse {
//     let res = db2::update_by_id::<BlogCategory>(blog_category_update.into_inner()).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json.rows_affected()).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body("")
//         }
//     }
// }

// // #[delete("/blog_category")]
// // async fn delete_blog_category(id: Json<Id>) -> impl Responder {
// //     action_handler::blog_category::delete_blog_category::execute(id).await;
// //     HttpResponse::Ok()
// //         .status(http::StatusCode::OK)
// //         .finish()
// // }

// #[post("/continent")]
// async fn create_continent(continent: Json<Continent>) -> HttpResponse {
//     let res = db::create::<Continent>(continent).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Created()
//                 .status(http::StatusCode::CREATED)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/continent")]
// async fn get_all_continents() -> HttpResponse {
//     let res = db::get_all::<Continent>().await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[get("/continent")]
// async fn get_continent_by_id(id: Json<Id>) -> HttpResponse {
//     let res = db::get_by_id::<Continent>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[put("/continent")]
// async fn update_continent(continent_update: Json<UpdateColumn>) -> HttpResponse {
//     let res = db::update_by_id::<Continent>(continent_update).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

// #[delete("/continent")]
// async fn delete_continent(id: Json<Id>) -> impl Responder {
//     let res = db::delete_by_id::<Continent>(id).await;
//     match res {
//         Ok(json) => {
//             HttpResponse::Ok()
//                 .status(http::StatusCode::OK)
//                 .content_type("application/json") 
//                 .body(serde_json::to_string(&json).unwrap_or_else(|e| format!("JSON serialization error: {}", e)))
//         }
//         Err(e) => {
//             e.error_response()
//         }
//     }
// }

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
