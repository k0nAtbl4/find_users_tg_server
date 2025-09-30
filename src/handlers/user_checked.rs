use axum::{extract::{State, Path}, http::StatusCode, routing::{get, post}, Json, Router};
use sqlx::{Pool, Postgres};
use serde::Deserialize;

use crate::{entities};
use crate::db::user_checked;

#[derive(Deserialize)]
pub struct UserPayload {
    username: String,
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<entities::user_checked::UserChecked>, StatusCode> {
    let user = user_checked::create_user(&pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;    
    Ok(Json(user))
}

pub async fn get_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<entities::user_checked::UserChecked>>, StatusCode> {
    let users = user_checked::get_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<entities::user_checked::UserChecked>, StatusCode> {
    let user = user_checked::get_user_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<entities::user_checked::UserChecked>, StatusCode> {
    let user = user_checked::update_user(&pool, id, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    user_checked::delete_user(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(StatusCode::NO_CONTENT)
}
