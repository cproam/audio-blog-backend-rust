use axum::{http::StatusCode, response::IntoResponse};

pub async fn list_categories() -> impl IntoResponse {
    (StatusCode::OK, "Categories endpoint")
}