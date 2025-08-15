use sqlx::{MySql, MySqlPool, Pool};
use crate::models::{User, Result};
use crate::error::Error;

use super::passwords::{hash_password, verify_password};

pub type DbPool = Pool<MySql>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    MySqlPool::connect(database_url)
        .await
        .map_err(|_| Error::DatabaseConnectionError)
}

pub async fn authenticate_user(pool: &DbPool, email: &str, password: &str) -> Result<User> {
    let row = sqlx::query!(
        "SELECT uid, email, password_hash, role, created_at, updated_at
         FROM users WHERE email = ?",
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| Error::DatabaseQueryError)?;

    match row {
        Some(user_row) => {
            if verify_password(password, &user_row.password_hash)? {
                Ok(User {
                    uid: user_row.uid,
                    email: user_row.email,
                    pw: user_row.password_hash,
                    role: user_row.role,
                })
            } else {
                Err(Error::InvalidCredentialsError)
            }
        }
        None => Err(Error::UserNotFoundError),
    }
}

pub async fn create_user(pool: &DbPool, uid: &str, email: &str, password: &str, role: &str) -> Result<()> {
    // Hash the password before storing
    let password_hash = hash_password(password)?;

    sqlx::query!(
        "INSERT INTO users (uid, email, password_hash, role) VALUES (?, ?, ?, ?)",
        uid, email, password_hash, role
    )
    .execute(pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("Duplicate entry") {
            Error::UserAlreadyExistsError
        } else {
            Error::DatabaseQueryError
        }
    })?;

    Ok(())
}

