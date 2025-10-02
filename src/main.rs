use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use db::db::create_pool;
use dotenvy::dotenv;
use handlers::user_checked;
use handlers::user_to_check::{
    create_user, delete_user, get_user, get_users, update_user_to_checked,
};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;

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
        // .route("/get_food", get(get_unchecked_users))
        .route(
            "/user_to_checked/:id",
            get(get_user)
                .put(update_user_to_checked)
                .delete(delete_user),
        )
        .route("/user_to_checked/:data",put(update_user_to_checked))
        .route(
            "/checked_users",
            post(user_checked::create_user).get(user_checked::get_users),
        )
        .route(
            "/checked_users/:id",
            get(user_checked::get_user)
                .put(user_checked::update_user)
                .delete(user_checked::delete_user),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
