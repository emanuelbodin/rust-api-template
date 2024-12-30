use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
pub use utils::test_utils::*;

mod utils;

#[sqlx::test]
fn health_check(pool: Pool<Postgres>) {
    let mut context = TestContext::new(pool);
    let (status, body) = context.get("/health").await;
    assert_eq!(status, StatusCode::OK); // 5
    assert_eq!(body, "OK".to_string());
}
