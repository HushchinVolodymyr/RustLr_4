use rocket::serde::json::json;
use rocket::serde::json::serde_json;
use rocket::{get, post, response::status::Created, serde::json::Json};
use serde::{Serialize, Deserialize};
use crate::models::User;
use crate::services::{get_database_url, hash_password, save_user_to_db, create_jwt, verify_password, get_user_by_username, get_user_id_by_username, get_messages_by_chat_id};
use sqlx::{PgPool, postgres::PgRow};


#[derive(Serialize, Deserialize)]
pub struct ReqUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewChat {
    first_user: i32,
    second_user: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub chat_id: i32,
    pub user_id: i32,
    pub message: String,
}

type Result<T, E = String> = std::result::Result<T, E>;

#[post("/register", format = "json", data = "<user>")]
pub async fn register_user(user: Json<ReqUser>) -> Result<Created<Json<serde_json::Value>>> {
    let database_url = get_database_url();
    let poll = PgPool::connect(&database_url).await.unwrap();

    let password_hs: String = hash_password(&user.password).unwrap();

    let new_user = User {
        username: user.username.clone(),
        password_hash: password_hs
    };

    let user_id = save_user_to_db(new_user, &poll).await.unwrap();

    let token = create_jwt(&user.username).unwrap();

    let response = json!({
        "message": "Created!",
        "token": token,
        "user_id": user_id
    });
    Ok(Created::new("/").body(Json(response)))
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login_user(user: Json<ReqUser>) -> Result<Json<serde_json::Value>> {
    let database_url = get_database_url();
    let pool = PgPool::connect(&database_url).await.map_err(|e| format!("Failed to connect to database: {}", e))?;

    let user_db = get_user_by_username(&user.username, &pool).await.map_err(|e| format!("User not found: {}", e))?;


    if !verify_password(&user.password, &user_db.password_hash).map_err(|e| format!("Password verification failed: {}", e))? {
        return Err("Password incorrect".to_string());
    }

    let user_id = get_user_id_by_username(&user.username, &pool).await.map_err(|e| format!("Failed to get user id: {}", e))?;

    if user_id == 0 {
        return Err("User not found".to_string());
    }

    let token = create_jwt(&user.username).map_err(|e| format!("Failed to create JWT: {}", e))?;

    let response = json!({
        "message": "Login successful!",
        "token": token,
        "user_id": user_id
    });

    Ok(Json(response))
}

#[get("/messages")]
pub async fn get_messages() -> Result<Json<serde_json::Value>> {
    let database_url = get_database_url();
    let pool = PgPool::connect(&database_url).await.map_err(|e| format!("Failed to connect to database: {}", e))?;

    let messages = get_messages_by_chat_id(1, &pool).await.map_err(|e| format!("Failed to get messages: {}", e))?;

    let response = json!({
        "messages": messages
    });

    Ok(Json(response))
}