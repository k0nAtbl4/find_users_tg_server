use axum::response::Json;
use serde_json::{json, Value};

pub async fn hello() -> &'static str {
    "Hello, World!"
}
