pub mod articles;
pub mod auth;
pub mod categories;

pub async fn root() -> &'static str {
    "Welcome to Audio Blog API"
}
