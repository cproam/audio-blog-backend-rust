use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber;

mod db;
mod handlers;

#[tokio::main]
async fn main() {
    // Инициализация логгирования
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // Подключение к базе данных
    let pool = db::init_db_pool()
        .await
        .expect("Failed to connect to database");

    // Создание роутов
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/login", get(handlers::auth::login))
        .route("/register", get(handlers::auth::register))
        .route("/articles", get(handlers::articles::list_articles))
        .route("/categories", get(handlers::categories::list_categories))
        .with_state(pool);

    // Запуск сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running at http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .expect("Server failed to start");
}
