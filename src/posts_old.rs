use axum::{http::StatusCode, Extension, Json, extract::Path};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    user_id: Option<i32>,
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    title: String,
    body: String,
    user_id: Option<i32>,
}


pub async fn get_posts(Extension(pool): Extension<Pool<Postgres>>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
} 

pub async fn get_post(Extension(pool): Extension<Pool<Postgres>>, Path(id): Path<i32>) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "SELECT id ,user_id, title, body FROM posts WHERE id = $1", 
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_|  StatusCode::NOT_FOUND)?;

    Ok(Json(post))

}

pub async fn create_post(Extension(pool): Extension<Pool<Postgres>>, Json(new_post): Json<CreatePost>) -> Result<Json<Post>, StatusCode> {
    let post  = sqlx::query_as!(
        Post, 
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, user_id, title, body", 
        new_post.user_id,
        new_post.title,
        new_post.body
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}

pub async fn update_post(Extension(pool): Extension<Pool<Postgres>>, Path(id): Path<i32>, Json(updated_post): Json<Post>) -> Result<Json<Post>, StatusCode> {
    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2 WHERE id = $3 RETURNING id, user_id, title, body",
        updated_post.title,
        updated_post.body, 
        id
    )
    .fetch_one(&pool)
    .await;   

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_post(Extension(pool): Extension<Pool<Postgres>>, Path(id): Path<i32>) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 RETURNING id",
        id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "message": "Post deleted successfully" }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}