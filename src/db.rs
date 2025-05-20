use sqlx::{Pool, Postgres};
use std::env;

pub async fn init_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::connect(&database_url).await
}
