use crate::auth::{
    build_refresh_cookie, clear_refresh_cookie, generate_access_token, generate_refresh_token,
    hash_password, verify_password,
};
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
struct User {
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
struct Theme {
    id: i64,
    theme: String,
}

#[derive(Debug, Clone, Serialize)]
struct LoginResponse {
    msg: String,
    access_token: Option<String>,
    user: Option<User>,
    themes: Option<Vec<Theme>>,
    success: bool,
}

// --- HELPERS ---

fn empty_string_check(fields: Vec<&str>) -> bool {
    fields.iter().any(|f| f.trim().is_empty())
}

// --- CONTROLLERS ---

pub async fn register_user(
    data: web::Data<AppState>,
    username: String,
    email: String,
    name: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &email, &name, &password]) {
        let response = Response {
            msg: String::from("Username, Email, Name or Password must Not be empty"),
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
        Some(_existing_user) => {
            let response = Response {
                msg: String::from("Username or Email is already in use"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        None => {
            let hashed_password = hash_password(&password);

            let result = sqlx::query!(
                "INSERT INTO users (username, email, name, password, user_type_id, account_status_id)
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

            if (!result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("Used Registration failed"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = Response {
                    msg: String::from("Registration successful"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
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
            msg: String::from("Username or Password must Not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let user = sqlx::query_as!(
        User,
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
                msg: String::from("User Not found, check your login details."),
                access_token: None,
                user: None,
                themes: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(user) => {
            let auth_details = sqlx::query!(
                "SELECT password, account_status_id FROM users WHERE id = $1",
                user.id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match auth_details {
                Some(auth_details) => {
                    if auth_details.account_status_id != 1 {
                        let response: LoginResponse = LoginResponse {
                            msg: String::from("Account is suspended or closed"),
                            access_token: None,
                            user: None,
                            themes: None,
                            success: false,
                        };
                        return Ok(HttpResponse::Unauthorized().json(response));
                    }

                    match verify_password(&password, &auth_details.password) {
                        Ok(_) => {
                            let access_token = generate_access_token(
                                user.username.clone(),
                                user.id,
                                user.user_type_id,
                                user.account_status_id,
                            );
                            let refresh_token = generate_refresh_token();
                            let refresh_expires = Utc::now() + Duration::days(7);

                            let update_token = sqlx::query!(
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
                            .map_err(|e| {
                                actix_web::error::ErrorInternalServerError(e.to_string())
                            })?;

                            if (update_token.rows_affected() > 0) {
                                let themes = sqlx::query_as!(
                                    Theme,
                                    "SELECT id, theme FROM themes ORDER BY id"
                                )
                                .fetch_all(&pool)
                                .await
                                .map_err(|e| {
                                    actix_web::error::ErrorInternalServerError(e.to_string())
                                })?;

                                match themes.is_empty() {
                                    true => {
                                        let response = LoginResponse {
                                            msg: String::from("Login Failed"),
                                            access_token: None,
                                            user: None,
                                            themes: None,
                                            success: false,
                                        };

                                        Ok(HttpResponse::BadRequest().json(response))
                                    }
                                    false => {
                                        let response = LoginResponse {
                                            msg: String::from("Login Successful"),
                                            access_token: Some(access_token),
                                            user: Some(user),
                                            themes: Some(_themes),
                                            success: true,
                                        };

                                        Ok(HttpResponse::Ok()
                                            .cookie(build_refresh_cookie(refresh_token))
                                            .json(response))
                                    }
                                }
                            } else {
                                let response = LoginResponse {
                                    msg: String::from("User Authentication Not Found"),
                                    access_token: None,
                                    user: None,
                                    themes: None,
                                    success: false,
                                };

                                Ok(HttpResponse::Unauthorized().json(response))
                            }
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
                None => {
                    let response = LoginResponse {
                        msg: String::from("User Authentication Not Found"),
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

            let new_access_token = generate_access_token(
                user.username,
                user.id,
                user.user_type_id,
                user.account_status_id,
            );
            let new_refresh_token = generate_refresh_token();
            let new_refresh_expires = Utc::now() + Duration::days(7);

            let result = sqlx::query!(
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

            if (result.rows_affected() > 0) {
                let response = DataResponse {
                    msg: String::from("Token refreshed"),
                    data: new_access_token,
                    success: true,
                };

                Ok(HttpResponse::Ok()
                    .cookie(build_refresh_cookie(new_refresh_token))
                    .json(response))
            } else {
                let response = Response {
                    msg: String::from("User Token Update Failed"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn logout_user(
    data: web::Data<AppState>,
    user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let result = sqlx::query!(
        "UPDATE users SET
            refresh_token = NULL,
            refresh_token_expires_at = NULL,
            refresh_token_updated_at = NOW(),
            is_online = FALSE,
            status_id = 4
         WHERE id = $1",
        user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (result.rows_affected() > 0) {
        let response = Response {
            msg: String::from("Logged out successfully"),
            success: true,
        };

        Ok(HttpResponse::Ok()
            .cookie(clear_refresh_cookie())
            .json(response))
    } else {
        let response = Response {
            msg: String::from("User Token Update Failed"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn reset_user_password(
    data: web::Data<AppState>,
    username: String,
    email: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if empty_string_check(vec![&username, &email, &password]) {
        let response = Response {
            msg: String::from("Username, Email or Password must Not be empty"),
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
                "UPDATE users SET password = $1, updated_at = NOW() WHERE id = $2",
                hashed_password,
                user.id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("Password reset successful"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Password reset failed"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}
