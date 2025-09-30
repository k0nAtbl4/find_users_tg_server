#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserToCheck {
    pub id: i32,
    pub username: String,
    pub is_checked: bool,
}
