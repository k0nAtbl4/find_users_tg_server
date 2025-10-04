use axum::{
    Router,
    routing::{get, post},
};
use db::db::create_pool;
use dotenvy::dotenv;
use handlers::user_checked;
use handlers::user_to_check::{
    create_user, delete_user, get_user, get_users, update_user_to_checked,
};
use handlers::user_gifts::{
    create_user_gift, delete_user_gift, get_user_gift, get_user_gifts, update_user_gift,
};
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
        .route("/user_to_checked",post(update_user_to_checked))
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
        .route("/user_gifts", post(create_user_gift).get(get_user_gifts))
        .route(
            "/user_gifts/:id",
            get(get_user_gift)
                .put(update_user_gift)
                .delete(delete_user_gift),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
