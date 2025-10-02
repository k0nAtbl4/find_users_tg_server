use crate::entities::user_to_check::UserToCheck;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn create_user(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<UserToCheck, sqlx::Error> {
    sqlx::query_as::<_, UserToCheck>(
        "INSERT INTO users_to_check (username, is_checked) VALUES ($1, false) RETURNING *",
    )
    .bind(username)
    .fetch_one(pool)
    .await
}

pub async fn get_users(pool: &Pool<Postgres>) -> Result<Vec<UserToCheck>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserToCheck>("SELECT * FROM users_to_check")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn get_unchecked_users(pool: &Pool<Postgres>) -> Result<Vec<UserToCheck>, sqlx::Error> {
    let users = sqlx::query_as::<_, UserToCheck>(
        "SELECT * FROM users_to_check WHERE is_checked = false LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn get_user_by_id(
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<Option<UserToCheck>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserToCheck>("SELECT * FROM users_to_check WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn update_user_checked(
    pool: &Pool<Postgres>,
    id: i32,
    is_good: bool,
) -> Result<Option<UserToCheck>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let user_to_check =
        sqlx::query_as::<_, UserToCheck>("DELETE FROM users_to_check WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?;

    if let Some(user) = user_to_check {
        if is_good {
            sqlx::query("INSERT INTO users_checked (username) VALUES ($1)")
                .bind(&user.username)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(Some(user))
    } else {
        tx.rollback().await?;
        Ok(None)
    }
}

pub async fn delete_user(
    pool: &Pool<Postgres>,
    id: i32,
) -> Result<Option<UserToCheck>, sqlx::Error> {
    let user =
        sqlx::query_as::<_, UserToCheck>("DELETE FROM users_to_check WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}
