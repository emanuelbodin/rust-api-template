use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::{routing::get, Extension, Router};
use tracing::{info,Level};
use tracing_subscriber;
use posts_old::{create_post, get_post, update_post, delete_post};
use posts::handlers::get_posts;

pub mod posts_old;
pub mod posts;
pub mod users;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to databse");

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts).post(create_post))
        .route("/users", get(users::get_users).post(users::create_user))
        .route("/posts/:id", get(get_post).put(update_post).delete(delete_post))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello world"
}