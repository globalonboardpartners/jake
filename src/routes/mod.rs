use actix_web::{get, post, put, delete, http, web, Responder, dev::HttpServiceFactory, HttpResponse};
use actix_web::web::Json;
use crate::action_handler;
use crate::data_types::structs::{Id, NewEmployee, UpdateColumn, NewBlog, NewJobListing, NewProductFeature, NewBlogCategory, NewContinent};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[post("/employee")]
async fn create_employee(employee: Json<NewEmployee>) -> HttpResponse {
    action_handler::employee::create_employee::execute(employee).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[get("/employees")]
async fn get_all_employees() -> impl Responder {
    let res = action_handler::employee::get_all_employees::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/employee")]
async fn get_employee_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::employee::get_employee_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[put("/employee")]
async fn update_employee(employee_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::employee::update_employee::execute(employee_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[delete("/employee")]
async fn delete_employee(id: Json<Id>) -> impl Responder {
    action_handler::employee::delete_employee::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[post("/blog")]
async fn create_blog(blog: Json<NewBlog>) -> HttpResponse {
    action_handler::blog::create_blog::execute(blog).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[get("/blogs")]
async fn get_all_blogs() -> HttpResponse {
    let res = action_handler::blog::get_all_blogs::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/blog")]
async fn get_blog_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::blog::get_blog_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[put("/blog")]
async fn update_blog(blog_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::blog::update_blog::execute(blog_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[delete("/blog")]
async fn delete_blog(id: Json<Id>) -> impl Responder {
    action_handler::blog::delete_blog::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[post("/job_listing")]
async fn create_job_listing(job_listing: Json<NewJobListing>) -> HttpResponse {
    action_handler::job_listing::create_job_listing::execute(job_listing).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[get("/job_listings")]
async fn get_all_job_listings() -> HttpResponse {
    let res = action_handler::job_listing::get_all_job_listings::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/job_listing")]
async fn get_job_listing_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::job_listing::get_job_listing_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[put("/job_listing")]
async fn update_job_listing(job_listing_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::job_listing::update_job_listing::execute(job_listing_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[delete("/job_listing")]
async fn delete_job_listing(id: Json<Id>) -> impl Responder {
    action_handler::job_listing::delete_job_listing::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[post("/product_feature")]
async fn create_product_feature(product_feature: Json<NewProductFeature>) -> HttpResponse {
    action_handler::product_feature::create_product_feature::execute(product_feature).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[get("/product_features")]
async fn get_all_product_features() -> HttpResponse {
    let res = action_handler::product_feature::get_all_product_features::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/product_feature")]
async fn get_product_feature_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::product_feature::get_product_feature_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[put("/product_feature")]
async fn update_product_feature(product_feature_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::product_feature::update_product_feature::execute(product_feature_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[delete("/product_feature")]
async fn delete_product_feature(id: Json<Id>) -> impl Responder {
    action_handler::product_feature::delete_product_feature::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[post("/blog_category")]
async fn create_blog_category(blog_category: Json<NewBlogCategory>) -> HttpResponse {
    action_handler::blog_category::create_blog_category::execute(blog_category).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[get("/blog_categories")]
async fn get_all_blog_categories() -> HttpResponse {
    let res = action_handler::blog_category::get_all_blog_categories::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/blog_category")]
async fn get_blog_category_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::blog_category::get_blog_category_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[put("/blog_category")]
async fn update_blog_category(blog_category_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::blog_category::update_blog_category::execute(blog_category_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

// #[delete("/blog_category")]
// async fn delete_blog_category(id: Json<Id>) -> impl Responder {
//     action_handler::blog_category::delete_blog_category::execute(id).await;
//     HttpResponse::Ok()
//         .status(http::StatusCode::OK)
//         .finish()
// }

#[post("/continent")]
async fn create_continent(continent: Json<NewContinent>) -> HttpResponse {
    action_handler::continent::create_continent::execute(continent).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[get("/continent")]
async fn get_all_continents() -> HttpResponse {
    let res = action_handler::continent::get_all_continents::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/continent")]
async fn get_continent_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::continent::get_continent_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[put("/continent")]
async fn update_continent(continent_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::product_feature::continent::execute(continent_update).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[delete("/continent")]
async fn delete_continent(id: Json<Id>) -> impl Responder {
    action_handler::continent::delete_continent::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
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
        get_all_blogs,
        get_blog_by_id,
        update_blog,
        delete_blog,
    )
}

pub fn job_listing() -> impl HttpServiceFactory {
    (
        create_job_listing,
        get_all_job_listings,
        get_job_listing_by_id,
        update_job_listing,
        delete_job_listing,
    )
}

pub fn product_feature() -> impl HttpServiceFactory {
    (
        create_product_feature,
        get_all_product_features,
        get_product_feature_by_id,
        update_product_feature,
        delete_product_feature,
    )
}

pub fn blog_category() -> impl HttpServiceFactory {
    (
        create_blog_category,
        get_all_blog_categories,
        get_blog_category_by_id,
        update_blog_category,
        // delete_blog_category,
    )
}

pub fn continent() -> impl HttpServiceFactory {
    (
        create_continent,
        get_all_continents,
        get_continent_by_id,
        update_continent,
        delete_continent,
    )
}
