use actix_web::{get, post, put, delete, http, web, Responder, dev::HttpServiceFactory, HttpResponse};
use actix_web::web::Json;
use crate::action_handler;
use crate::data_types::structs::{Id, NewEmployee, UpdateColumn, NewBlog};

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

#[delete("/employee")]
async fn delete_employee(id: Json<Id>) -> impl Responder {
    action_handler::employee::delete_employee::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[put("/employee")]
async fn update_employee(employee_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::employee::update_employee::execute(employee_update).await;
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

pub fn routes() -> impl HttpServiceFactory {
    (
        greet,
        get_all_employees,
        get_employee_by_id,
        delete_employee,
        create_employee,
        update_employee,
        create_blog,
        get_all_blogs,
        get_blog_by_id,
        update_blog,
        delete_blog,
    )
}
