use axum::{http::StatusCode, Extension, Json};
use sqlx::{Pool, Postgres};

use super::db::{insert_user, select_users, CreateUser, User};

pub async fn get_users(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = select_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = insert_user(&pool, new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
