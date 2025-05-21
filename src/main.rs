use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use axum::{Json, Router, extract::Extension, extract::Path, http::StatusCode, routing::get};
use tracing::{Level, info};
use tracing_subscriber;

#[derive(Serialize, Deserialize)]
struct Article {
    id: i32,
    user_id: Option<i32>,
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct CreateArticle {
    title: String,
    body: String,
    user_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct UpdateArticle {
    title: String,
    body: String,
    user_id: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // init tracing sub for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database");

    // build app with the route
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/articles", get(get_articles).post(create_article))
        .route(
            "/articles/{id}",
            get(get_article).put(update_article).delete(delete_article),
        )
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    info!("Server is running on 0.0.0.0:8000");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    return "Hello, World!";
}

async fn get_articles(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Article>>, StatusCode> {
    let articles = sqlx::query_as!(Article, "SELECT id, user_id, title, body FROM articles")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok(Json(articles));
}

async fn get_article(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Article>, StatusCode> {
    let article = sqlx::query_as!(
        Article,
        "SELECT id, user_id, title, body FROM articles WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    return Ok(Json(article));
}

async fn create_article(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_article): Json<CreateArticle>,
) -> Result<Json<Article>, StatusCode> {
    let article = sqlx::query_as!(
        Article,
        "INSERT INTO articles (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, title, body, user_id",
        new_article.user_id,
        new_article.title,
        new_article.body
    ).fetch_one(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(article))
}

async fn update_article(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(updated_article): Json<UpdateArticle>,
) -> Result<Json<Article>, StatusCode> {
    let article = sqlx::query_as!(
        Article,
        "UPDATE articles SET title = $1, body = $2, user_id = $3 WHERE id = $4 RETURNING id, user_id, title, body",
        updated_article.title,
        updated_article.body,
        updated_article.user_id,
        id
    )
    .fetch_one(&pool)
    .await;

    match article {
        Ok(article) => Ok(Json(article)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_article(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query!("DELETE FROM articles WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json! ({
            "message": "Article deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
