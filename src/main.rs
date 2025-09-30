use axum::{extract::State, http::StatusCode, routing::{get, post, put, delete}, Json, Router};
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use handlers::user_to_check::{create_user, get_users, get_user, update_user, delete_user};
use handlers::user_checked;
use db::db::{create_pool};

pub mod db;
pub mod entities;
pub mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = create_pool().await.expect("failed to create pool");

    let app = Router::new()
        .route("/", get(handlers::hello::hello))
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/checked_users", post(user_checked::create_user).get(user_checked::get_users))
        .route("/checked_users/:id", get(user_checked::get_user).put(user_checked::update_user).delete(user_checked::delete_user))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}