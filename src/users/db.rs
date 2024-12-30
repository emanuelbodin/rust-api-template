use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn select_users(pool: &Pool<Postgres>) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as!(User, "SELECT id, username, email FROM users",)
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn insert_user(pool: &Pool<Postgres>, new_user: CreateUser) -> Result<User, Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        new_user.username,
        new_user.email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
