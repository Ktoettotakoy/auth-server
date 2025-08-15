use std::convert::Infallible;

use auth_server::error::*;
use auth_server::model::*;
use auth_server::auth::{with_auth, Role};
use auth_server::*;
use warp::{reject,reply, Filter, Reply};

#[tokio::main]
async fn main() {
    // let users = Arc::new(init_users());


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
        .recover(handle_rejection);

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
        None => Err(reject::custom(Error::InvalidCredentialsError)),
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

// TODO: replace with a proper mysql db
