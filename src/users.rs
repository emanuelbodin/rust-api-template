use serde::{Deserialize, Serialize};
use axum::{Extension, Json, http::StatusCode};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

pub async fn get_users(Extension(pool): Extension<Pool<Postgres>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn create_user(Extension(pool): Extension<Pool<Postgres>>, Json(new_user): Json<CreateUser>) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        new_user.username,
        new_user.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}