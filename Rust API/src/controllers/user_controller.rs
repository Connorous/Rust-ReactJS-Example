use crate::auth::{
    build_refresh_cookie, clear_refresh_cookie, generate_access_token, generate_refresh_token,
    hash_password, verify_password, JwtClaims,
};
use crate::routes::user_routes::UpdateProfileRequestBody;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use serde::Serialize;

// --- RESPONSE STRUCTS ---

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DataResponse<T: Serialize> {
    msg: String,
    data: T,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct LoggedInUser {
    id: i64,
    username: String,
    name: String,
    email: String,
    bio_info: Option<String>,
    user_type_id: i64,
    account_status_id: i64,
    status_id: Option<i64>,
    is_online: bool,
    theme_id: Option<i64>,
    theme_dark_mode: bool,
    light_theme_primary_colour: String,
    light_theme_secondary_colour: String,
    light_theme_accent_colour: String,
    light_theme_sent_colour: String,
    light_theme_received_colour: String,
    light_theme_dark_text_colour: String,
    light_theme_light_text_colour: String,
    dark_theme_primary_colour: String,
    dark_theme_secondary_colour: String,
    dark_theme_accent_colour: String,
    dark_theme_sent_colour: String,
    dark_theme_received_colour: String,
    dark_theme_dark_text_colour: String,
    dark_theme_light_text_colour: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct UserRow {
    id: i64,
    username: String,
    name: String,
    email: String,
    bio_info: Option<String>,
    user_type_id: i64,
    account_status_id: i64,
    status_id: Option<i64>,
    is_online: bool,
    theme_id: Option<i64>,
    theme_dark_mode: bool,
    light_theme_primary_colour: String,
    light_theme_secondary_colour: String,
    light_theme_accent_colour: String,
    light_theme_sent_colour: String,
    light_theme_received_colour: String,
    light_theme_dark_text_colour: String,
    light_theme_light_text_colour: String,
    dark_theme_primary_colour: String,
    dark_theme_secondary_colour: String,
    dark_theme_accent_colour: String,
    dark_theme_sent_colour: String,
    dark_theme_received_colour: String,
    dark_theme_dark_text_colour: String,
    dark_theme_light_text_colour: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct UserType {
    id: i64,
    r#type: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct Theme {
    id: i64,
    theme: String,
}

#[derive(Debug, Clone, Serialize)]
struct LoginResponse {
    msg: String,
    access_token: Option<String>,
    user: Option<LoggedInUser>,
    themes: Option<Vec<Theme>>,
    success: bool,
}

// --- HELPERS ---

fn empty_string_check(fields: Vec<&str>) -> bool {
    fields.iter().any(|f| f.trim().is_empty())
}

fn user_row_to_logged_in(user: UserRow) -> LoggedInUser {
    LoggedInUser {
        id: user.id,
        username: user.username,
        name: user.name,
        email: user.email,
        bio_info: user.bio_info,
        user_type_id: user.user_type_id,
        account_status_id: user.account_status_id,
        status_id: user.status_id,
        is_online: user.is_online,
        theme_id: user.theme_id,
        theme_dark_mode: user.theme_dark_mode,
        light_theme_primary_colour: user.light_theme_primary_colour,
        light_theme_secondary_colour: user.light_theme_secondary_colour,
        light_theme_accent_colour: user.light_theme_accent_colour,
        light_theme_sent_colour: user.light_theme_sent_colour,
        light_theme_received_colour: user.light_theme_received_colour,
        light_theme_dark_text_colour: user.light_theme_dark_text_colour,
        light_theme_light_text_colour: user.light_theme_light_text_colour,
        dark_theme_primary_colour: user.dark_theme_primary_colour,
        dark_theme_secondary_colour: user.dark_theme_secondary_colour,
        dark_theme_accent_colour: user.dark_theme_accent_colour,
        dark_theme_sent_colour: user.dark_theme_sent_colour,
        dark_theme_received_colour: user.dark_theme_received_colour,
        dark_theme_dark_text_colour: user.dark_theme_dark_text_colour,
        dark_theme_light_text_colour: user.dark_theme_light_text_colour,
    }
}

// SELECT columns used in multiple queries — avoids repeating the long list
const USER_SELECT: &str = "id, username, name, email, bio_info, user_type_id, account_status_id,
    status_id, is_online, theme_id, theme_dark_mode,
    light_theme_primary_colour, light_theme_secondary_colour,
    light_theme_accent_colour, light_theme_sent_colour,
    light_theme_received_colour, light_theme_dark_text_colour,
    light_theme_light_text_colour, dark_theme_primary_colour,
    dark_theme_secondary_colour, dark_theme_accent_colour,
    dark_theme_sent_colour, dark_theme_received_colour,
    dark_theme_dark_text_colour, dark_theme_light_text_colour";

// --- LOGIN CONTROLLERS ---

pub async fn register_user(
    data: web::Data<AppState>,
    username: String,
    email: String,
    name: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &email, &name, &password]) {
        let response = Response {
            msg: String::from("Username, Email, Name or Password must not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let existing = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 OR email = $2",
        username,
        email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match existing {
        Some(_) => {
            let response = Response {
                msg: String::from("Username or Email is already in use"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        None => {
            let hashed_password = hash_password(&password);

            let result = sqlx::query!(
                "INSERT INTO users (username, email, name, password_hash, user_type_id, account_status_id)
                 VALUES ($1, $2, $3, $4, $5, $6)",
                username,
                email,
                name,
                hashed_password,
                4i64,
                1i64
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match result.rows_affected() {
                0 => {
                    let response = Response {
                        msg: String::from("Registration failed"),
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
                _ => {
                    let response = Response {
                        msg: String::from("Registration successful"),
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
            }
        }
    }
}

pub async fn login_user(
    data: web::Data<AppState>,
    username: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &password]) {
        let response = Response {
            msg: String::from("Username or Password must not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let user = sqlx::query_as!(
        UserRow,
        "SELECT id, username, name, email, bio_info, user_type_id, account_status_id,
                status_id, is_online, theme_id, theme_dark_mode,
                light_theme_primary_colour, light_theme_secondary_colour,
                light_theme_accent_colour, light_theme_sent_colour,
                light_theme_received_colour, light_theme_dark_text_colour,
                light_theme_light_text_colour, dark_theme_primary_colour,
                dark_theme_secondary_colour, dark_theme_accent_colour,
                dark_theme_sent_colour, dark_theme_received_colour,
                dark_theme_dark_text_colour, dark_theme_light_text_colour
         FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match user {
        None => {
            let response = LoginResponse {
                msg: String::from("Username not found"),
                access_token: None,
                user: None,
                themes: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(user) => {
            // Fetch password hash and account status separately
            let auth_row = sqlx::query!(
                "SELECT password_hash, account_status_id FROM users WHERE id = $1",
                user.id
            )
            .fetch_one(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Check account is active
            if auth_row.account_status_id != 1 {
                let response = LoginResponse {
                    msg: String::from("Account is suspended or closed"),
                    access_token: None,
                    user: None,
                    themes: None,
                    success: false,
                };
                return Ok(HttpResponse::Unauthorized().json(response));
            }

            match verify_password(&password, &auth_row.password_hash) {
                Ok(_) => {
                    let access_token = generate_access_token(
                        user.username.clone(),
                        user.id,
                        user.user_type_id,
                        user.account_status_id,
                    );
                    let refresh_token = generate_refresh_token();
                    let refresh_expires = Utc::now() + Duration::days(7);

                    // Store refresh token and set online
                    sqlx::query!(
                        "UPDATE users SET
                            refresh_token = $1,
                            refresh_token_expires_at = $2,
                            refresh_token_created_at = COALESCE(refresh_token_created_at, NOW()),
                            refresh_token_updated_at = NOW(),
                            is_online = TRUE,
                            status_id = 1
                         WHERE id = $3",
                        refresh_token,
                        refresh_expires,
                        user.id
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    // Fetch themes to return with login response
                    let themes = sqlx::query_as!(Theme, "SELECT id, theme FROM themes ORDER BY id")
                        .fetch_all(&pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    let response = LoginResponse {
                        msg: String::from("Login successful"),
                        access_token: Some(access_token),
                        user: Some(user_row_to_logged_in(user)),
                        themes: Some(themes),
                        success: true,
                    };

                    Ok(HttpResponse::Ok()
                        .cookie(build_refresh_cookie(refresh_token))
                        .json(response))
                }
                Err(_) => {
                    let response = LoginResponse {
                        msg: String::from("Incorrect password"),
                        access_token: None,
                        user: None,
                        themes: None,
                        success: false,
                    };
                    Ok(HttpResponse::Unauthorized().json(response))
                }
            }
        }
    }
}

pub async fn refresh_token(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let cookie = match req.cookie("refresh_token") {
        Some(c) => c,
        None => {
            let response = Response {
                msg: String::from("No refresh token"),
                success: false,
            };
            return Ok(HttpResponse::Unauthorized().json(response));
        }
    };

    let refresh_token = cookie.value().to_string();
    let pool = data.db.to_owned();

    let user = sqlx::query!(
        "SELECT id, username, user_type_id, account_status_id, refresh_token_expires_at
         FROM users WHERE refresh_token = $1",
        refresh_token
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match user {
        None => {
            let response = Response {
                msg: String::from("Invalid refresh token"),
                success: false,
            };
            Ok(HttpResponse::Unauthorized().json(response))
        }
        Some(user) => {
            let expires_at = match user.refresh_token_expires_at {
                Some(exp) => exp,
                None => {
                    let response = Response {
                        msg: String::from("Refresh token has no expiry"),
                        success: false,
                    };
                    return Ok(HttpResponse::Unauthorized().json(response));
                }
            };

            if Utc::now() > expires_at {
                sqlx::query!(
                    "UPDATE users SET
                        refresh_token = NULL,
                        refresh_token_expires_at = NULL,
                        refresh_token_updated_at = NOW()
                     WHERE id = $1",
                    user.id
                )
                .execute(&pool)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                let response = Response {
                    msg: String::from("Refresh token expired, please log in again"),
                    success: false,
                };
                return Ok(HttpResponse::Unauthorized()
                    .cookie(clear_refresh_cookie())
                    .json(response));
            }

            if user.account_status_id != 1 {
                let response = Response {
                    msg: String::from("Account is suspended or closed"),
                    success: false,
                };
                return Ok(HttpResponse::Unauthorized()
                    .cookie(clear_refresh_cookie())
                    .json(response));
            }

            // Generate new access token and rotate refresh token
            let new_access_token = generate_access_token(
                user.username,
                user.id,
                user.user_type_id,
                user.account_status_id,
            );
            let new_refresh_token = generate_refresh_token();
            let new_refresh_expires = Utc::now() + Duration::days(7);

            sqlx::query!(
                "UPDATE users SET
                    refresh_token = $1,
                    refresh_token_expires_at = $2,
                    refresh_token_updated_at = NOW()
                 WHERE id = $3",
                new_refresh_token,
                new_refresh_expires,
                user.id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            let response = DataResponse {
                msg: String::from("Token refreshed"),
                data: new_access_token,
                success: true,
            };

            Ok(HttpResponse::Ok()
                .cookie(build_refresh_cookie(new_refresh_token))
                .json(response))
        }
    }
}

pub async fn logout_user(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    sqlx::query!(
        "UPDATE users SET
            refresh_token = NULL,
            refresh_token_expires_at = NULL,
            refresh_token_updated_at = NOW(),
            is_online = FALSE,
            status_id = 4
         WHERE id = $1",
        claims.user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let response = Response {
        msg: String::from("Logged out successfully"),
        success: true,
    };

    Ok(HttpResponse::Ok()
        .cookie(clear_refresh_cookie())
        .json(response))
}

pub async fn reset_user_password(
    data: web::Data<AppState>,
    username: String,
    email: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &email, &password]) {
        let response = Response {
            msg: String::from("Username, Email or Password must not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let user = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 AND email = $2",
        username,
        email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match user {
        None => {
            let response = Response {
                msg: String::from("No user found with that username and email"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(user) => {
            let hashed_password = hash_password(&password);

            let result = sqlx::query!(
                "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                hashed_password,
                user.id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match result.rows_affected() {
                0 => {
                    let response = Response {
                        msg: String::from("Password reset failed"),
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
                _ => {
                    let response = Response {
                        msg: String::from("Password reset successful"),
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
            }
        }
    }
}

// --- USER CONTROLLERS ---

pub async fn list_users(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let users = sqlx::query_as!(
        UserRow,
        "SELECT id, username, name, email, bio_info, user_type_id, account_status_id,
                status_id, is_online, theme_id, theme_dark_mode,
                light_theme_primary_colour, light_theme_secondary_colour,
                light_theme_accent_colour, light_theme_sent_colour,
                light_theme_received_colour, light_theme_dark_text_colour,
                light_theme_light_text_colour, dark_theme_primary_colour,
                dark_theme_secondary_colour, dark_theme_accent_colour,
                dark_theme_sent_colour, dark_theme_received_colour,
                dark_theme_dark_text_colour, dark_theme_light_text_colour
         FROM users ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match users.is_empty() {
        true => {
            let response = Response {
                msg: String::from("No users found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        false => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: users,
                success: true,
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn list_user_types(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let user_types = sqlx::query_as!(UserType, "SELECT id, type FROM user_types ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match user_types.is_empty() {
        true => {
            let response = Response {
                msg: String::from("No user types found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        false => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: user_types,
                success: true,
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn get_user(
    data: web::Data<AppState>,
    claims: JwtClaims,
    id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let user = sqlx::query_as!(
        UserRow,
        "SELECT id, username, name, email, bio_info, user_type_id, account_status_id,
                status_id, is_online, theme_id, theme_dark_mode,
                light_theme_primary_colour, light_theme_secondary_colour,
                light_theme_accent_colour, light_theme_sent_colour,
                light_theme_received_colour, light_theme_dark_text_colour,
                light_theme_light_text_colour, dark_theme_primary_colour,
                dark_theme_secondary_colour, dark_theme_accent_colour,
                dark_theme_sent_colour, dark_theme_received_colour,
                dark_theme_dark_text_colour, dark_theme_light_text_colour
         FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match user {
        None => {
            let response = Response {
                msg: String::from("User not found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(user) => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: user,
                success: true,
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn new_user(
    data: web::Data<AppState>,
    claims: JwtClaims,
    username: String,
    email: String,
    name: String,
    password: String,
    user_type_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &email, &name, &password]) {
        let response = Response {
            msg: String::from("Username, Email, Name or Password must not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    // Cannot create a user with higher authority than yourself
    if claims.user_type_id > user_type_id {
        let response = Response {
            msg: String::from("You cannot create a user with higher authority than yourself"),
            success: false,
        };
        return Ok(HttpResponse::Forbidden().json(response));
    }

    let pool = data.db.to_owned();

    let existing = sqlx::query!(
        "SELECT id FROM users WHERE username = $1 OR email = $2",
        username,
        email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match existing {
        Some(_) => {
            let response = Response {
                msg: String::from("Username or Email is already in use"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        None => {
            let hashed_password = hash_password(&password);

            let result = sqlx::query!(
                "INSERT INTO users (username, email, name, password_hash, user_type_id,
                  account_status_id, created_by, updated_by)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                username,
                email,
                name,
                hashed_password,
                user_type_id,
                1i64,
                claims.user_id,
                claims.user_id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match result.rows_affected() {
                0 => {
                    let response = Response {
                        msg: String::from("Failed to create user"),
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
                _ => {
                    let response = Response {
                        msg: String::from("User created successfully"),
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
            }
        }
    }
}

pub async fn update_user(
    data: web::Data<AppState>,
    claims: JwtClaims,
    id: i64,
    username: String,
    email: String,
    name: String,
    user_type_id: i64,
    account_status_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &email, &name]) {
        let response = Response {
            msg: String::from("Username, Email or Name must not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    // Fetch target user to check their current type
    let target_user = sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match target_user {
        None => {
            let response = Response {
                msg: String::from("User not found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(target) => {
            // Cannot update a user with higher authority than yourself
            if claims.user_type_id > target.user_type_id {
                let response = Response {
                    msg: String::from(
                        "You cannot update a user with higher authority than yourself",
                    ),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Cannot set a user to a type with higher authority than yourself
            if claims.user_type_id > user_type_id {
                let response = Response {
                    msg: String::from(
                        "You cannot set a user to a type with higher authority than yourself",
                    ),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            let result = sqlx::query!(
                "UPDATE users SET
                    username = $1,
                    email = $2,
                    name = $3,
                    user_type_id = $4,
                    account_status_id = $5,
                    updated_by = $6,
                    updated_at = NOW()
                 WHERE id = $7",
                username,
                email,
                name,
                user_type_id,
                account_status_id,
                claims.user_id,
                id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match result.rows_affected() {
                0 => {
                    let response = Response {
                        msg: String::from("User not updated"),
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
                _ => {
                    let response = Response {
                        msg: String::from("User updated successfully"),
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
            }
        }
    }
}

pub async fn update_profile(
    data: web::Data<AppState>,
    claims: JwtClaims,
    body: UpdateProfileRequestBody,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let result = sqlx::query!(
        "UPDATE users SET
            name = $1,
            bio_info = $2,
            theme_id = $3,
            theme_dark_mode = $4,
            light_theme_primary_colour = $5,
            light_theme_secondary_colour = $6,
            light_theme_accent_colour = $7,
            light_theme_sent_colour = $8,
            light_theme_received_colour = $9,
            light_theme_dark_text_colour = $10,
            light_theme_light_text_colour = $11,
            dark_theme_primary_colour = $12,
            dark_theme_secondary_colour = $13,
            dark_theme_accent_colour = $14,
            dark_theme_sent_colour = $15,
            dark_theme_received_colour = $16,
            dark_theme_dark_text_colour = $17,
            dark_theme_light_text_colour = $18,
            updated_at = NOW()
         WHERE id = $19",
        body.name,
        body.bio_info,
        body.theme_id,
        body.theme_dark_mode,
        body.light_theme_primary_colour,
        body.light_theme_secondary_colour,
        body.light_theme_accent_colour,
        body.light_theme_sent_colour,
        body.light_theme_received_colour,
        body.light_theme_dark_text_colour,
        body.light_theme_light_text_colour,
        body.dark_theme_primary_colour,
        body.dark_theme_secondary_colour,
        body.dark_theme_accent_colour,
        body.dark_theme_sent_colour,
        body.dark_theme_received_colour,
        body.dark_theme_dark_text_colour,
        body.dark_theme_light_text_colour,
        claims.user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match result.rows_affected() {
        0 => {
            let response = Response {
                msg: String::from("Profile not updated"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        _ => {
            let response = Response {
                msg: String::from("Profile updated successfully"),
                success: true,
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn update_status(
    data: web::Data<AppState>,
    claims: JwtClaims,
    status_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let result = sqlx::query!(
        "UPDATE users SET status_id = $1, updated_at = NOW() WHERE id = $2",
        status_id,
        claims.user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match result.rows_affected() {
        0 => {
            let response = Response {
                msg: String::from("Status not updated"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        _ => {
            let response = Response {
                msg: String::from("Status updated successfully"),
                success: true,
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn delete_user(
    data: web::Data<AppState>,
    claims: JwtClaims,
    id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let target_user = sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match target_user {
        None => {
            let response = Response {
                msg: String::from("User not found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(target) => {
            // Cannot delete a user with higher authority than yourself
            if claims.user_type_id > target.user_type_id {
                let response = Response {
                    msg: String::from(
                        "You cannot delete a user with higher authority than yourself",
                    ),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Only super admin can delete themselves
            if claims.user_id == id && claims.user_type_id != 1 {
                let response = Response {
                    msg: String::from("You cannot delete your own account"),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            let mut tx = pool
                .begin()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete messages sent by user
            sqlx::query!("DELETE FROM messages WHERE sender_id = $1", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete group permissions
            sqlx::query!("DELETE FROM chat_group_permissions WHERE user_id = $1", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete groups created by user
            sqlx::query!("DELETE FROM chat_groups WHERE created_by = $1", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete relationships
            sqlx::query!(
                "DELETE FROM user_relationships WHERE requester_id = $1 OR receiver_id = $1",
                id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete user
            let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            tx.commit()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match result.rows_affected() {
                0 => {
                    let response = Response {
                        msg: String::from("User not deleted"),
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
                _ => {
                    let response = Response {
                        msg: String::from("User deleted successfully"),
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
            }
        }
    }
}
