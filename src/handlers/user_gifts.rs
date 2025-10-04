use axum::{extract::{State, Path}, http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use serde::Deserialize;

use crate::{entities};

use crate::db::user_gifts;

#[derive(Deserialize)]
pub struct CreateUserGiftPayload {
    pub username: String,
    pub gifts: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserGiftPayload {
    pub username: String,
    pub gifts: Vec<String>,
}

pub async fn create_user_gift(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateUserGiftPayload>,
) -> Result<Json<entities::user_gifts::UserWithGifts>, StatusCode> {
    let user = user_gifts::create_user_gift(&pool, &payload.username, &payload.gifts)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}

pub async fn get_user_gifts(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<entities::user_gifts::UserWithGifts>>, StatusCode> {
    let users = user_gifts::get_user_gifts(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn get_user_gift(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<entities::user_gifts::UserWithGifts>, StatusCode> {
    let user = user_gifts::get_user_gift_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

pub async fn update_user_gift(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserGiftPayload>,
) -> Result<Json<entities::user_gifts::UserWithGifts>, StatusCode> {
    let user = user_gifts::update_user_gift(&pool, id, &payload.username, &payload.gifts)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(user))
}

pub async fn delete_user_gift(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    user_gifts::delete_user_gift(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(StatusCode::NO_CONTENT)
}
