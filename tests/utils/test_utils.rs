use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use axum_rest_api_template::create_app;
use sqlx::{Pool, Postgres};
use tower::Service;

#[cfg(test)]
pub struct TestContext {
    pub app: Router,
}

impl TestContext {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            app: create_app(pool),
        }
    }

    pub async fn get(&mut self, url: &str) -> (StatusCode, String) {
        let request = Request::builder()
            .uri(url)
            .method("GET")
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();
        let response = self.app.call(request).await.unwrap();
        let code = response.status();
        let body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), 10000)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        (code, body)
    }

    pub async fn post(&mut self, url: &str, body: &str) -> (StatusCode, String) {
        let request = Request::builder()
            .uri(url)
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let response = self.app.call(request).await.unwrap();
        let code = response.status();
        let body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), 10000)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        (code, body)
    }
}
