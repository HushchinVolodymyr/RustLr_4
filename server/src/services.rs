extern crate dotenv;

use std::env;
use dotenv::dotenv;

use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng, Error as PasswordHashError};
use std::fmt;
use sqlx::{Error, PgPool};
use crate::models::{RMessage, User};

use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug)]
pub enum PasswordError {
    HashingError(String),
    VerificationError(String),
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::HashingError(err) => write!(f, "Hashing error: {}", err),
            PasswordError::VerificationError(err) => write!(f, "Verification error: {}", err),
        }
    }
}

impl std::error::Error for PasswordError {}

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    // Hash the password
    let argon2 = Argon2::default();
    let password_hash_result = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| PasswordError::HashingError(e.to_string()))?;

    // Return the hash as a string
    Ok(password_hash_result.to_string())
}

/// Verify a password against a hash
pub fn verify_password(hash: &str, password: &str) -> Result<bool, PasswordError> {
    // Parse the stored hash
    let parsed_hash = PasswordHash::new(password)
        .map_err(|err| PasswordError::VerificationError(err.to_string()))?;

    // Verify the password
    let argon2 = Argon2::default();
    let result = argon2.verify_password(hash.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Err(PasswordError::VerificationError("Password verification failed".to_string())),
    }
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("No secret set");

    // Define the claims
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,  // 1 hour expiration
    };

    // Define the header (optional, but can include the algorithm used)
    let header = Header::new(jsonwebtoken::Algorithm::HS256);

    // Encode the token
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}


// Get database URL from .env file
pub fn get_database_url() -> String {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    return database_url;
}

// Save user to database
pub async fn save_user_to_db(user: User, poll: &PgPool) -> Result<i32, Error> {
    let _result = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        "#,
        user.username,
        user.password_hash
    ).execute(poll).await?;

    let user_id = sqlx::query_scalar!(
    r#"
    SELECT id FROM users WHERE username = $1
    "#,
    user.username
    ).fetch_one(poll).await?;

    Ok(user_id)
}

// get user id by username
pub async fn get_user_id_by_username(username: &str, pool: &PgPool) -> Result<i32, Error> {
    let user_id = sqlx::query_scalar!(
        r#"
        SELECT id FROM users WHERE username = $1
        "#,
        username
    ).fetch_one(pool).await?;

    Ok(user_id)
}

// Get user from database by username
pub async fn get_user_by_username(username: &str, pool: &PgPool) -> Result<User, Error> {
    // Use query_as to directly map the result to the User struct
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, password_hash FROM users WHERE username = $1
        "#,
        username
    )
        .fetch_one(pool)  // This fetches a single row (assuming the username is unique)
        .await?;

    println!("{:?} : {:?}", user.username, user.password_hash);

    Ok(user)
}

pub async fn save_message_to_db(message: RMessage, pool: &PgPool) -> Result<(), Error> {
    let _result = sqlx::query!(
        r#"
        INSERT INTO messages (chat_id, user_id, message)
        VALUES ($1, $2, $3)
        "#,
        message.chat_id,
        message.user_id,
        message.message
    ).execute(pool).await?;

    Ok(())
}

// Get messages from database by chat_id
pub async fn get_messages_by_chat_id(chat_id: i32, pool: &PgPool) -> Result<Vec<RMessage>, Error> {
    let messages = sqlx::query_as!(
        RMessage,
        r#"
        SELECT chat_id, user_id, message FROM messages WHERE chat_id = $1
        "#,
        chat_id
    )
        .fetch_all(pool)
        .await?;

    Ok(messages)
}


