use axum::{http::StatusCode, Extension, Json};
use sqlx::{Pool, Postgres};
use db::{Post, select_all_posts};
use super::db;

pub async fn get_posts(Extension(pool): Extension<Pool<Postgres>>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = select_all_posts(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
} 