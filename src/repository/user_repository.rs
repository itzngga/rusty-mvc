use crate::config::sqlite::new_sqlite_connection;
use crate::entity::user_entity::{UserEntity, UserEntityInsert};
use crate::exception::app_error::AppError;
use crate::model::user_model::GetAllUserRequest;
use crate::schema::users;
use actix_web::web::Query;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
pub async fn get_all_users(
    request: &Query<GetAllUserRequest>,
) -> Result<(Vec<UserEntity>, i32), AppError> {
    let mut conn = new_sqlite_connection();

    let mut query = users::table.into_boxed();
    if let Some(i) = request.q.clone() {
        query = query.filter(users::username.like(format!("%{}%", i)))
    }

    let offset = (request.page.unwrap() - 1) * request.limit_per_page.unwrap();
    let user_result = query
        .limit(request.limit_per_page.unwrap() as i64)
        .offset(offset as i64)
        .load::<UserEntity>(&mut conn)
        .map_err(|_| AppError::new(500).message("find users table error".to_owned()))?;

    let mut query = users::table.into_boxed();
    if let Some(i) = request.q.clone() {
        query = query.filter(users::username.like(format!("%{}%", i)))
    }

    let count: i64 = query
        .count()
        .get_result(&mut conn)
        .map_err(|_| AppError::new(500).message("find users table error".to_owned()))?;

    Ok((user_result, count as i32))
}

pub async  fn get_user_detail(user_id: i32) -> Result<UserEntity, AppError> {
    let mut conn = new_sqlite_connection();

    let user = users::table
        .filter(users::id.eq(user_id))
        .first(&mut conn)
        .map_err(|_| AppError::new(500).message("get user detail error".to_owned()))?;
    Ok(user)
}

pub async fn add_new_user(user: UserEntityInsert) -> Result<(), AppError> {
    let mut conn = new_sqlite_connection();

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&mut conn)
        .map_err(|_| AppError::new(500).message("insert users error".to_owned()))?;
    Ok(())
}

pub async fn update_user(user: UserEntity) -> Result<(), AppError> {
    let mut conn = new_sqlite_connection();

    diesel::update(users::table.find(user.id))
        .set(&user)
        .execute(&mut conn)
        .map_err(|_| AppError::new(500).message("update users error".to_owned()))?;
    Ok(())
}

pub async fn delete_user(user_id: i32) -> Result<(), AppError> {
    let mut conn = new_sqlite_connection();

    diesel::delete(users::table.find(user_id))
        .execute(&mut conn)
        .map_err(|_| AppError::new(500).message("delete users error".to_owned()))?;
    Ok(())
}

pub async fn is_valid_user_id(user_id: i32) -> Result<bool, AppError> {
    let mut conn = new_sqlite_connection();

    let count: i64 = users::table
        .filter(users::id.eq(user_id))
        .count()
        .get_result(&mut conn)
        .map_err(|_| AppError::new(500).message("delete users error".to_owned()))?;

    if count == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}
