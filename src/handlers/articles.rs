use axum::{http::StatusCode, response::IntoResponse};

pub async fn list_articles() -> impl IntoResponse {
    (StatusCode::OK, "Articles endpoint")
}