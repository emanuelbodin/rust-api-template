use axum::{routing::get, Extension, Router};
use posts::handlers::{create_post, get_post, get_posts, put_post, remove_post};
use sqlx::{Pool, Postgres};
use users::handlers::{create_user, get_users};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod open_api;
pub mod posts;
pub mod users;

pub fn create_app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/posts", get(get_posts).post(create_post))
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/posts/:id",
            get(get_post).put(put_post).delete(remove_post),
        )
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", open_api::ApiDoc::openapi()))
        .layer(Extension(pool))
}

async fn health_check() -> &'static str {
    "OK"
}
