use serde::{Deserialize, Serialize};
use warp::{Rejection};

use std::sync::Arc;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Users = Arc<HashMap<String, User>>;


#[derive(Clone)]
pub struct User {
    pub uid: i32,
    pub email: String,
    pub pw: String,
    pub role: String // maybe add more roles later
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
