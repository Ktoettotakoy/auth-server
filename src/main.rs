use std::convert::Infallible;

use auth_server::error::*;
use auth_server::models::*;
use auth_server::auth::{with_auth, Role};
use auth_server::db::db_ops::{DbPool};
use auth_server::*;
use warp::{reject,reply, Filter, Reply};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Database URL from environment or default
    let database_url = std::env::var("DATABASE_URL").unwrap();

    // Create database connection pool
    let db_pool = db::db_ops::create_pool(&database_url).await.unwrap();
    println!("Database connected successfully!");

    let login_route = warp::path("login")
        .and(warp::post())
        .and(with_db(db_pool.clone()))
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

    let register_route = warp::path("register")
        .and(warp::post())
        .and(with_auth(Role::Admin))
        .and(with_db(db_pool.clone()))
        .and(warp::body::json())
        .and_then(register_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .or(register_route)
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    println!("Server running on http://127.0.0.1:3030");
}

// Database filter
fn with_db(db_pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub async fn login_handler(db_pool: DbPool, body: LoginRequest) -> WebResult<impl Reply> {
    // Auth user
    let user = db::db_ops::authenticate_user(&db_pool, &body.email, &body.pw)
        .await
        .map_err(|e| reject::custom(e))?;

    let token = auth::generate_jwt_token(&user.uid, &Role::from_str(&user.role))
        .map_err(|e| reject::custom(e))?;

    Ok(reply::json(&LoginResponse { token }))
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello user: {}", uid))
    // reply::json(&UserResponse { uid })
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello admin: {}", uid))
    // reply::json(&UserResponse { uid })
}

pub async fn register_handler(admin_uid: String, db_pool: DbPool, body: RegisterRequest) -> WebResult<impl Reply> {
    println!("Admin {} is creating a new user", admin_uid);

    // Generate new UID
    let uid = uuid::Uuid::new_v4().to_string();

    db::db_ops::create_user(&db_pool, &uid, &body.email, &body.pw, &body.role)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(reply::json(&RegisterResponse { success: true }))
}




#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_connection() {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").unwrap();
        let result = db::db_ops::create_pool(&database_url).await;

        // This test will only pass if MySQL is running
        if result.is_ok() {
            println!("Database connection successful!");
        }
    }
}
