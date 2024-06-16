use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PaginationResponse {
    pub total_page: i32,
    pub current_page: i32,
    pub limit_per_page: i32,
}
