use crate::exception::app_error::AppError;
use crate::model::user_model as model;
use crate::service::user_service;
use actix_web::{web, HttpResponse, Responder};

pub fn setup_routes(app: &mut web::ServiceConfig) {
    app.route("/users", web::get().to(get_all_users));
    app.route("/users/{user_id}", web::get().to(get_user_detail));
    app.route("/users", web::post().to(add_new_user));
    app.route("/users/{user_id}", web::put().to(update_user));
    app.route("/users/{user_id}", web::delete().to(delete_user));
}

pub async fn get_all_users(
    request: web::Query<model::GetAllUserRequest>,
) -> Result<impl Responder, AppError> {
    let result = user_service::get_all_users(request).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}

pub async fn add_new_user(
    request: web::Json<model::AddNewUserRequest>,
) -> Result<impl Responder, AppError> {
    let result = user_service::add_new_user(request).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}
pub async fn get_user_detail(user_id: web::Path<i32>) -> Result<impl Responder, AppError> {
    let result = user_service::get_user_detail(*user_id).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}

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

pub async fn delete_user(user_id: web::Path<i32>) -> Result<impl Responder, AppError> {
    let result = user_service::delete_user(*user_id).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}
