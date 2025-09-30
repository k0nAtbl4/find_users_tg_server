use crate::entities::user_checked::UserChecked;
use sqlx::{Pool, Postgres};

pub async fn create_user(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<UserChecked, sqlx::Error> {
    let user = sqlx::query_as::<_, UserChecked>(
        "INSERT INTO users_checked (username) VALUES ($1) RETURNING *",
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_users(pool: &Pool<Postgres>) -> Result<Vec<UserChecked>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserChecked>("SELECT * FROM users_checked")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserChecked>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserChecked>("SELECT * FROM users_checked WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn update_user(
    pool: &Pool<Postgres>,
    id: i32,
    username: &str,
) -> Result<Option<UserChecked>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserChecked>(
        "UPDATE users_checked SET username = $1 WHERE id = $2 RETURNING *",
    )
    .bind(username)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserChecked>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserChecked>("DELETE FROM users_checked WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
