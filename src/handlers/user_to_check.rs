use axum::{extract::{State, Path}, http::StatusCode, routing::{get, post}, Json, Router};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use serde::Deserialize;

use crate::{entities};

use crate::db::user_to_check;

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    username: String,
    is_checked: bool,
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<entities::user_to_check::UserToCheck>, StatusCode> {
    let username = payload["username"].as_str().ok_or(StatusCode::BAD_REQUEST)?;
    let user = user_to_check::create_user(&pool, username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;    
    Ok(Json(user))
}

pub async fn get_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<entities::user_to_check::UserToCheck>>, StatusCode> {
    let users = user_to_check::get_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<entities::user_to_check::UserToCheck>, StatusCode> {
    let user = user_to_check::get_user_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

pub async fn update_user_to_checked(
    State(pool): State<Pool<Postgres>>,
    Path((id,is_good)): Path<(i32,bool)>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<entities::user_to_check::UserToCheck>, StatusCode> {
    let user = user_to_check::update_user_checked(&pool, id,is_good)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    user_to_check::delete_user(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(StatusCode::NO_CONTENT)
}