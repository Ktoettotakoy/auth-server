use crate::{error::Error, Result, WebResult};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Algorithm, Validation};
use std::fmt;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION,},
    reject, Filter, Rejection
};

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"mySecretChangeLater";

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}


impl Role {
    pub fn from_str(role: &str) -> Role {
        match role.to_ascii_lowercase().as_str() {
            "admin" => Role::Admin,
            _ => Role::User, // default role
        }
    }
}

// Formatting option for the role enum
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

// JWT claim
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}


pub fn generate_jwt_token(uid: &str, role: &Role) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}


pub fn with_auth(role: Role) -> impl Filter<Extract = (String, ), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(token) => {
            let decoded = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(Error::NoPermissionError))
            }
            // TODO: handle more roles
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(header) => header,
        None => return Err(Error::MissingAuthorizationHeader),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(h) => h,
        Err(_) => return Err(Error::InvalidAuthorizationHeader),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthorizationHeader);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
