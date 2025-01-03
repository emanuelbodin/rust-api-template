use super::db;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use db::{delete_post, insert_post, select_all_posts, select_post, update_post, CreatePost, Post};
use sqlx::{Pool, Postgres};

#[utoipa::path(
    get,
    path = "/api/posts",
    responses(
        (status = 200, body = [Post])
    )
)]
pub async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = select_all_posts(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, description = "Post ID")
    ),
    responses(
        (status = 200, body = Post),
        (status = 404 )
    )
)]
pub async fn get_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    let post = select_post(&pool, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(post))
}

#[utoipa::path(
    post,
    path = "/api/posts",
    request_body = CreatePost,
    responses(
        (status = 200, body = Post),
        (status = 404 )
    )
)]
pub async fn create_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    let post = insert_post(&pool, new_post)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}

#[utoipa::path(
    put,
    path = "/api/posts",
    request_body = Post,
    responses(
        (status = 200, body = Post),
        (status = 404 )
    )
)]
pub async fn put_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(updated_post): Json<Post>,
) -> Result<Json<Post>, StatusCode> {
    let post = update_post(&pool, id, updated_post).await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, description = "Post ID to delete")
    ),
    responses(
        (status = 200, body = Post),
        (status = 404 )
    )
)]
pub async fn remove_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = delete_post(&pool, id).await;

    match result {
        Ok(_) => Ok(Json(
            serde_json::json!({ "message": "Post deleted successfully" }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
