
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::entities::user_to_check::UserToCheck;

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

