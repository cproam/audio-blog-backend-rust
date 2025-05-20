use axum::{http::StatusCode, response::IntoResponse};

pub async fn login() -> impl IntoResponse {
    (StatusCode::OK, "Login endpoint")
}

pub async fn register() -> impl IntoResponse {
    (StatusCode::OK, "Register endpoint")
}