use crate::entity::user_entity::{UserEntity, UserEntityInsert};
use crate::exception::app_error::AppError;
use crate::model::common_model::PaginationResponse;
use crate::model::user_model::{AddNewUserRequest, GetAllUserData, GetAllUserRequest, GetAllUserResponse, GetUserDetailResponse, UpdateUserRequest};
use crate::repository::user_repository;
use crate::utils;
use actix_web::web::{Json, Query};
use chrono::Utc;

pub async fn get_all_users(
    mut request: Query<GetAllUserRequest>,
) -> Result<GetAllUserResponse, AppError> {
    if request.page == None || request.page.unwrap() <= 0 {
        request.page = Some(1);
    }

    if request.limit_per_page == None || request.limit_per_page.unwrap() <= 0 {
        request.limit_per_page = Some(25);
    }

    let users =  user_repository::get_all_users(&request).await?;

    let total_page = utils::common::calculate_total_pages(users.1, request.limit_per_page.unwrap());

    if request.page.unwrap() > total_page {
        return Err(AppError::new(404).message("page not found".to_owned()));
    }

    let mut response = GetAllUserResponse {
        total_data: users.1,
        pagination: PaginationResponse {
            total_page,
            current_page: request.page.unwrap(),
            limit_per_page: request.limit_per_page.unwrap(),
        },
        users: vec![],
    };

    for user in users.0 {
        response.users.push(GetAllUserData {
            id: user.id,
            username: user.username,
            password: user.password,
            created_at: Some(
                user.created_at
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
            updated_at: Some(
                user.updated_at
                    .unwrap()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
        });
    }

    return Ok(response);
}

pub async fn get_user_detail(user_id: i32) -> Result<GetUserDetailResponse, AppError> {
    if !user_repository::is_valid_user_id(user_id.clone()).await? {
        return Err(AppError::new(404).message("user id not found".to_owned()))
    }

    let user =  user_repository::get_user_detail(user_id).await?;
    
    Ok (GetUserDetailResponse{
        id: user.id,
        username: user.username,
        password: user.password,
        created_at: Some(
            user.created_at
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        ),
        updated_at: Some(
            user.updated_at
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        ),
    })
}

pub async fn add_new_user(request: Json<AddNewUserRequest>) -> Result<(), AppError> {
    let user_entity = UserEntityInsert {
        username: Some(request.username.clone()),
        password: Some(request.password.clone()),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    match user_repository::add_new_user(user_entity).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn update_user(user_id: i32, request: Json<UpdateUserRequest>) -> Result<(), AppError> {
    if !user_repository::is_valid_user_id(user_id.clone()).await? {
        return Err(AppError::new(404).message("user id not found".to_owned()))
    }

    let user_entity = UserEntity {
        id: user_id,
        username: Some(request.username.clone()),
        password: Some(request.password.clone()),
        created_at: None,
        updated_at: Some(Utc::now().naive_utc()),
    };

    match user_repository::update_user(user_entity).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub async fn delete_user(user_id: i32) -> Result<(), AppError> {
    if !user_repository::is_valid_user_id(user_id.clone()).await? {
        return Err(AppError::new(404).message("user id not found".to_owned()))
    }

    match user_repository::delete_user(user_id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
