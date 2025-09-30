#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserChecked {
    pub id: i32,
    pub username: String,
}
