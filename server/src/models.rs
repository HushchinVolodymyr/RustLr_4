use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub first_user: i32,
    pub second_user: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RMessage {
    pub chat_id: i32,
    pub user_id: i32,
    pub message: String,
}

