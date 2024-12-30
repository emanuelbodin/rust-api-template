use axum::http::StatusCode;
use axum_rest_api_template::posts::db::Post;
use axum_rest_api_template::users::db::User;
use sqlx::{Pool, Postgres};
use test_utils::TestContext;

mod test_utils;

#[sqlx::test]
fn create_post(pool: Pool<Postgres>) {
    let mut context = TestContext::new(pool);
    let (status, body) = context
        .post(
            "/api/users",
            r#"{
            "username": "test",
            "email": "test@mail.com"
        }"#,
        )
        .await;

    assert_eq!(status, StatusCode::OK);
    let user_created = serde_json::from_str::<User>(&body).unwrap();
    let (status, body) = context
        .post(
            "/api/posts",
            format!(
                r#"{{
            "title": "Hello, world!",
            "body": "This is my first post",
            "user_id": {}
            }}"#,
                user_created.id
            )
            .as_str(),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    let post_created = serde_json::from_str::<Post>(&body).unwrap();
    assert_eq!(post_created.user_id, user_created.id);
    assert_eq!(post_created.title, "Hello, world!");
    assert_eq!(post_created.body, "This is my first post");
}

#[sqlx::test]
fn get_posts(pool: Pool<Postgres>) {
    let mut context = TestContext::new(pool);
    let (status, _) = context.get("/api/posts/1").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
