use sqlx::{Pool, Postgres, Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: i32,
    user_id: Option<i32>,
    title: String,
    body: String,
}

pub async fn select_all_posts(pool: &Pool<Postgres>) -> Result<Vec<Post>, Error> {
    let posts = sqlx::query_as!(Post, "SELECT id, user_id, title, body FROM posts")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}