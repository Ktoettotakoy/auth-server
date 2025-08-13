use std::convert::Infallible;

use auth::{with_auth, Role};
use error::Error::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::sync::Arc;
use warp::{reject,reply, Filter, Rejection, Reply};

mod auth;
mod error;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[derive(Clone)]
pub struct User {
    pub uid: String,
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

#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_route = warp::path("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path("user")
        .and(warp::get())
        .and(with_auth(Role::User))
        .and_then(user_handler);

        let admin_route = warp::path("admin")
        .and(warp::get())
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users.iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token = auth::generate_jwt_token(uid, &Role::from_str(&user.role))
                .map_err(|e| reject::custom(e))?;

            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(InvalidCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello user: {}", uid))
    // reply::json(&UserResponse { uid })
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello admin: {}", uid))
    // reply::json(&UserResponse { uid })
}

fn init_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "1".to_string(),
        User {
            uid: "uid1".to_string(),
            email: "u1@user.com".to_string(),
            pw: "pw1".to_string(),
            role: "user".to_string(),
        },
    );
    users.insert(
        "2".to_string(),
        User {
            uid: "uid2".to_string(),
            email: "u2@admin.com".to_string(),
            pw: "pw2".to_string(),
            role: "admin".to_string(),
        },
    );
    users
}
