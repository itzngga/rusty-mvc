use crate::exception::app_error::AppError;
use crate::model::user_model as model;
use crate::service::user_service;
use actix_web::{get, post, put, web, HttpResponse, Responder, delete};

pub fn setup_routes(app: &mut web::ServiceConfig) {
    app.service((get_all_users, get_user_detail, add_new_user, update_user, delete_user));
}

#[get("/users")]
pub async fn get_all_users(
    request: web::Query<model::GetAllUserRequest>,
) -> Result<impl Responder, AppError> {
    let result = user_service::get_all_users(request).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}

#[post("/users")]
pub async fn add_new_user(
    request: web::Json<model::AddNewUserRequest>,
) -> Result<impl Responder, AppError> {
    let result = user_service::add_new_user(request).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}
#[get("/users/{user_id}")]
pub async fn get_user_detail(
    user_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let result = user_service::get_user_detail(*user_id).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}

#[put("/users/{user_id}")]
pub async fn update_user(
    user_id: web::Path<i32>,
    request: web::Json<model::UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    let result = user_service::update_user(*user_id, request).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    user_id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let result = user_service::delete_user(*user_id).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}
