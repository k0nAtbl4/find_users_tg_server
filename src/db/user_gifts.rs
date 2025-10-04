use crate::entities::user_gifts::UserWithGifts;
use sqlx::{Pool, Postgres};

pub async fn create_user_gift(
    pool: &Pool<Postgres>,
    username: &str,
    gifts: &Vec<String>,
) -> Result<UserWithGifts, sqlx::Error> {
    sqlx::query_as::<_, UserWithGifts>(
        "INSERT INTO user_gifts (username, gifts) VALUES ($1, $2) RETURNING *",
    )
    .bind(username)
    .bind(gifts)
    .fetch_one(pool)
    .await
}

pub async fn get_user_gifts(pool: &Pool<Postgres>) -> Result<Vec<UserWithGifts>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserWithGifts>("SELECT * FROM user_gifts")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn get_user_gift_by_id(
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<Option<UserWithGifts>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserWithGifts>("SELECT * FROM user_gifts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn update_user_gift(
    pool: &Pool<Postgres>,
    id: i32,
    username: &str,
    gifts: &Vec<String>,
) -> Result<Option<UserWithGifts>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserWithGifts>(
        "UPDATE user_gifts SET username = $1, gifts = $2 WHERE id = $3 RETURNING *",
    )
    .bind(username)
    .bind(gifts)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user_gift(
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<Option<UserWithGifts>, sqlx::Error> {
    let user =
        sqlx::query_as::<_, UserWithGifts>("DELETE FROM user_gifts WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}
