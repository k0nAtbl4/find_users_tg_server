use crate::entities::user_to_check::UserToCheck;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_user(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<UserToCheck, sqlx::Error> {
    match sqlx::query_as::<_, UserToCheck>(
        "INSERT INTO users_to_check (username, is_checked) VALUES ($1, false) RETURNING *",
    )
    .bind(username)
    .fetch_one(pool).await
    {
        Ok(user) => {Ok(user)},
        Err(e) => {Err(e)}
    }
    // Ok(user)
}

pub async fn get_users(pool: &Pool<Postgres>) -> Result<Vec<UserToCheck>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserToCheck>("SELECT * FROM users_to_check")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserToCheck>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserToCheck>("SELECT * FROM users_to_check WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    id: i32,
    username: &str,
    is_checked: bool,
) -> Result<Option<UserToCheck>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserToCheck>(
        "UPDATE users_to_check SET username = $1, is_checked = $2 WHERE id = $3 RETURNING *",
    )
    .bind(username)
    .bind(is_checked)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserToCheck>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserToCheck>("DELETE FROM users_to_check WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}