use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    middleware::Next,
    Error, HttpMessage, HttpResponse,
};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    password_hash::{self, rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

use crate::state::User;

#[derive(Deserialize, Serialize)]
struct Claims {
    exp: usize,
    sub: String,
}

fn get_secret() -> String {
    let SECRET: String = std::env::var("SECRET").unwrap();
    SECRET
}

pub fn hash_password(password: &String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    hashed_password
}

pub fn verify_password(
    password: &String,
    hashed_password: &String,
) -> Result<(), password_hash::Error> {
    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    argon2.verify_password(password.as_bytes(), &parsed_hash)
}

pub fn generate_token(username: String) -> String {
    let claims = Claims {
        sub: username,
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 60 * 60 * 1,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .unwrap();
    token
}

fn verify_token(token: &str) -> Result<String, String> {
    match decode(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    ) {
        Err(e) => Err(e.to_string()),
        Ok(token_data) => {
            let claims: Claims = token_data.claims;
            Ok(claims.sub)
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AuthResponse {
    msg: String,
    success: bool,
}

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
                match verify_token(token) {
                    Ok(sub) => {
                        req.extensions_mut().insert(sub);
                        next.call(req).await
                    }
                    Err(e) => {
                        let response = AuthResponse {
                            msg: String::from("Invalid Token"),
                            success: false,
                        };

                        return Ok(req.into_response(HttpResponse::Unauthorized().json(response)));
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
                msg: String::from("Invalid Route"),
                success: false,
            };
            Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
        }
    }
}

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
            if (header_value == std::env::var("SECRET").unwrap()) {
                next.call(req).await
            } else {
                let response = AuthResponse {
                    msg: String::from("You are not the App."),
                    success: false,
                };

                return Ok(req.into_response(HttpResponse::Unauthorized().json(response)));
            }
        }
        None => {
            let response = AuthResponse {
                msg: String::from("You are not the App."),
                success: false,
            };
            Ok(req.into_response(HttpResponse::Unauthorized().json(response)))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    success: bool,
}

/*pub async fn try_auth(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let response = Response {
        msg: String::from("Success"),
        success: true,
    };

    return Ok(HttpResponse::Ok().json(response));
}*/
