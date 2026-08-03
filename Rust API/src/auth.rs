use actix_web::{
    body::{BoxBody, MessageBody},
    cookie::{time::Duration, Cookie, SameSite},
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    middleware::Next,
    Error, HttpMessage, HttpResponse,
};

use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    password_hash::{self, rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Claims embedded in the access token
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JwtClaims {
    pub sub: String, // username
    pub user_id: i64,
    pub user_type_id: i64,
    pub account_status_id: i64,
    pub exp: usize,
}

#[derive(Deserialize, Serialize)]
struct AuthResponse {
    msg: String,
    success: bool,
}

fn get_secret() -> String {
    std::env::var("SECRET").unwrap()
}

fn get_app_secret() -> String {
    std::env::var("APP_SECRET").unwrap()
}

// --- PASSWORD ---

pub fn hash_password(password: &String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(
    password: &String,
    hashed_password: &String,
) -> Result<(), password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    argon2.verify_password(password.as_bytes(), &parsed_hash)
}

// --- ACCESS TOKEN ---

pub fn generate_access_token(
    username: String,
    user_id: i64,
    user_type_id: i64,
    account_status_id: i64,
) -> String {
    let claims = JwtClaims {
        sub: username,
        user_id,
        user_type_id,
        account_status_id,
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 60 * 15, // 15 minutes
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .unwrap()
}

pub fn verify_access_token(token: &str) -> Result<JwtClaims, String> {
    match decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e.to_string()),
    }
}

// --- REFRESH TOKEN ---

pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn build_refresh_cookie(token: String) -> Cookie<'static> {
    Cookie::build("refresh_token", token)
        .http_only(true)
        .secure(false) // HTTP only for localhost, change later
        //.secure(true) // HTTPS only
        .same_site(SameSite::Strict)
        .max_age(Duration::days(7))
        .path("/login/refresh") // cookie only sent to refresh endpoint
        .finish()
}

pub fn clear_refresh_cookie() -> Cookie<'static> {
    Cookie::build("refresh_token", "")
        .http_only(true)
        .secure(false) // HTTP only for localhost, change later
        //.secure(true) // HTTPS only
        .same_site(SameSite::Strict)
        .max_age(Duration::seconds(0))
        .path("/login/refresh")
        .finish()
}

// --- MIDDLEWARE ---

// Protects routes requiring a valid JWT access token
// Inserts JwtClaims into request extensions for extractors to use
pub async fn auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.method() == Method::OPTIONS {
        return Ok(req.into_response(HttpResponse::Ok().finish()));
    }

    match req.headers().get("Authorization") {
        Some(header_value) => {
            let header_value = header_value.to_str().unwrap();
            if header_value.starts_with("Bearer ") {
                let token = header_value.split(" ").collect::<Vec<&str>>()[1];
                match verify_access_token(token) {
                    Ok(claims) => {
                        // Insert full claims so extractors can use them
                        req.extensions_mut().insert(claims);
                        next.call(req).await
                    }
                    Err(_) => {
                        let response = AuthResponse {
                            msg: String::from("Invalid or Expired Token"),
                            success: false,
                        };
                        Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
                    }
                }
            } else {
                let response = AuthResponse {
                    msg: String::from("Invalid Token, No Bearer"),
                    success: false,
                };
                Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
            }
        }
        None => {
            let response = AuthResponse {
                msg: String::from("No Authorization Header"),
                success: false,
            };
            Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
        }
    }
}

// Protects login/register routes — checks app secret rather than JWT
pub async fn app_auth(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.method() == Method::OPTIONS {
        return Ok(req.into_response(HttpResponse::Ok().finish()));
    }

    match req.headers().get("Authorization") {
        Some(header_value) => {
            let header_value = header_value.to_str().unwrap();
            if header_value == get_app_secret() {
                next.call(req).await
            } else {
                let response = AuthResponse {
                    msg: String::from("Invalid App Secret"),
                    success: false,
                };
                Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
            }
        }
        None => {
            let response = AuthResponse {
                msg: String::from("No Authorization Header"),
                success: false,
            };
            Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
        }
    }
}
