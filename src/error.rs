use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {

    #[error("Invalid credentials")]
    InvalidCredentialsError,

    #[error("Missing authorization header")]
    MissingAuthorizationHeader,
    #[error("Invalid authorization header")]
    InvalidAuthorizationHeader,

    #[error("JWT token not valid")]
    JWTTokenError,
    #[error("JWT token creation error")]
    JWTTokenCreationError,

    #[error("No permission error")]
    NoPermissionError,

    // New database errors
    #[error("Database connection error")]
    DatabaseConnectionError,

    #[error("Database query error")]
    DatabaseQueryError,

    #[error("User not found")]
    UserNotFoundError,

    #[error("User already exists")]
    UserAlreadyExistsError,

    #[error("Password hashing error")]
    PasswordHashError,

    #[error("Password verification error")]
    PasswordVerificationError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::InvalidCredentialsError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::MissingAuthorizationHeader => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::InvalidAuthorizationHeader => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
            Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::DatabaseConnectionError => (StatusCode::INTERNAL_SERVER_ERROR, "Database connection failed".to_string()),
            Error::DatabaseQueryError => (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed".to_string()),
            Error::UserNotFoundError => (StatusCode::NOT_FOUND, e.to_string()),
            Error::UserAlreadyExistsError => (StatusCode::CONFLICT, e.to_string()),
            Error::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, "Password processing error".to_string()),
            Error::PasswordVerificationError => (StatusCode::INTERNAL_SERVER_ERROR, "Password verification error".to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed".to_string())
    } else {
        eprintln!("Unhandled error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
