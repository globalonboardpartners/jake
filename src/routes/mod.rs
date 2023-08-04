use actix_web::{get, post, put, delete, http, web, Responder, dev::HttpServiceFactory, HttpResponse};
use actix_web::web::Json;
use crate::action_handler;
use crate::data_types::structs::{Id, NewEmployee, UpdateColumn};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello boi {name}!")
}

#[get("/employees")]
async fn get_all_employees() -> impl Responder {
    let res = action_handler::get_all_employees::execute().await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[get("/employee")]
async fn get_employee_by_id(id: Json<Id>) -> impl Responder {
    let res = action_handler::get_employee_by_id::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .content_type("application/json")
        .body(res)
}

#[delete("/employee")]
async fn delete_employee(id: Json<Id>) -> impl Responder {
    action_handler::delete_employee::execute(id).await;
    HttpResponse::Ok()
        .status(http::StatusCode::OK)
        .finish()
}

#[post("/employee")]
async fn create_employee(employee: Json<NewEmployee>) -> HttpResponse {
    action_handler::create_employee::execute(employee).await;
    HttpResponse::Created()
        .status(http::StatusCode::CREATED)
        .finish()
}

#[put("/employee")]
async fn update_employee(employee_update: Json<UpdateColumn>) -> HttpResponse {
    action_handler::update_employee::execute(employee_update).await;
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
        update_employee
    )
}
