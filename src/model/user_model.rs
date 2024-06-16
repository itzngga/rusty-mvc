use crate::model::common_model::PaginationResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetAllUserRequest {
    pub page: Option<i32>,
    pub limit_per_page: Option<i32>,
    pub q: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetAllUserData {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetAllUserResponse {
    pub total_data: i32,
    pub pagination: PaginationResponse,
    pub users: Vec<GetAllUserData>,
}

#[derive(Deserialize, Serialize)]
pub struct GetUserDetailResponse {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct AddNewUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub password: String,
}
