use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use test_utils::TestContext;

mod test_utils;

#[sqlx::test]
fn health_check(pool: Pool<Postgres>) {
    let mut context = TestContext::new(pool);
    let (status, body) = context.get("/health").await;
    assert_eq!(status, StatusCode::OK); // 5
    assert_eq!(body, "OK".to_string());
}
