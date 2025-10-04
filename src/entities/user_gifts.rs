#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserWithGifts {
    pub id: i32,
    pub username: String,
    pub gifts: Vec<String>,
}

