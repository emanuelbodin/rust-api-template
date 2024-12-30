use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    title: String,
    body: String,
    user_id: Option<i32>,
}

pub async fn select_all_posts(pool: &Pool<Postgres>) -> Result<Vec<Post>, Error> {
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}

pub async fn select_post(pool: &Pool<Postgres>, id: i32) -> Result<Post, Error> {
    let post = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, body FROM posts WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(post)
}

pub async fn insert_post(pool: &Pool<Postgres>, new_post: CreatePost) -> Result<Post, Error> {
    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, user_id, title, body",
        new_post.user_id,
        new_post.title,
        new_post.body
    )
    .fetch_one(pool)
    .await?;
    Ok(post)
}

pub async fn update_post(
    pool: &Pool<Postgres>,
    id: i32,
    updated_post: Post,
) -> Result<Post, Error> {
    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2 WHERE id = $3 RETURNING id, user_id, title, body",
        updated_post.title,
        updated_post.body,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(post)
}

pub async fn delete_post(pool: &Pool<Postgres>, id: i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}
