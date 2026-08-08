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
struct UserManageRow {
    id: i64,
    username: String,
    name: String,
    email: String,
    bio_info: Option<String>,
    user_type_id: i64,
    account_status_id: i64,
    status_id: Option<i64>,
    is_online: bool,
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

// --- HELPERS ---

fn empty_string_check(fields: Vec<&str>) -> bool {
    fields.iter().any(|f| f.trim().is_empty())
}

// --- USER CONTROLLERS ---

pub async fn list_users(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let users = sqlx::query_as!(
        UserManageRow,
        "SELECT id, username, name, email, bio_info, user_type_id, account_status_id, status_id, is_online FROM users ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match users.is_empty() {
        true => {
            let response = Response {
                msg: String::from("No Users Found"),
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
                msg: String::from("No User Types found"),
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
         FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match user {
        None => {
            let response = Response {
                msg: String::from("User Not found"),
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
    if (empty_string_check(vec![&username, &email, &name, &password])) {
        let response = Response {
            msg: String::from("Username, Email, Name or Password must Not be empty"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    // Cannot create a user with higher authority than yourself
    if (claims.user_type_id > user_type_id) {
        let response = Response {
            msg: String::from("You Cannot Create a User with Greater Permissions than Yourself"),
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
        Some(_existing) => {
            let response = Response {
                msg: String::from("Username or Email is already in use"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        None => {
            let hashed_password = hash_password(&password);

            let result = sqlx::query!(
                "INSERT INTO users (username, email, name, password, user_type_id,
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

            if (result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("User Created Successfully"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Failed to Create User"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
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
    if (empty_string_check(vec![&username, &email, &name])) {
        let response = Response {
            msg: String::from("Username, Email or Name must Not be empty"),
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
                msg: String::from("User Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(target) => {
            // Cannot update a user with higher authority than yourself
            if (claims.user_type_id > target.user_type_id) {
                let response = Response {
                    msg: String::from(
                        "You Cannot Update a User with Greater Permissions than Yourself",
                    ),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Cannot set a user to a type with higher authority than yourself
            if (claims.user_type_id > user_type_id) {
                let response = Response {
                    msg: String::from("You Cannot Update a User to a Type Greater than Yourself"),
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

            if (result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("User updated Successfully"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("User Not Updated"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn logout_user(
    data: web::Data<AppState>,
    claims: JwtClaims,
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
        claims.user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (result.rows_affected() > 0) {
        let response = Response {
            msg: String::from("Logged out Successfully"),
            success: true,
        };

        Ok(HttpResponse::Ok()
            .cookie(clear_refresh_cookie())
            .json(response))
    } else {
        let response = Response {
            msg: String::from("Logout Failed"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
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
            bio_info = $1,
            theme_id = $2,
            theme_dark_mode = $3,
            light_theme_primary_colour = $4,
            light_theme_secondary_colour = $5,
            light_theme_accent_colour = $6,
            light_theme_sent_colour = $7,
            light_theme_received_colour = $8,
            light_theme_dark_text_colour = $9,
            light_theme_light_text_colour = $10,
            dark_theme_primary_colour = $11,
            dark_theme_secondary_colour = $12,
            dark_theme_accent_colour = $13,
            dark_theme_sent_colour = $14,
            dark_theme_received_colour = $15,
            dark_theme_dark_text_colour = $16,
            dark_theme_light_text_colour = $17,
            updated_at = NOW()
         WHERE id = $18",
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

    if (result.rows_affected() > 0) {
        let response = Response {
            msg: String::from("Profile Updated Successfully"),
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Profile Not Updated"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
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

    if (result.rows_affected() > 0) {
        let response = Response {
            msg: String::from("Status Updated Successfully"),
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Status Not Updated"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
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
                msg: String::from("User Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(target) => {
            // Cannot delete a user with higher authority than yourself
            if (claims.user_type_id > target.user_type_id) {
                let response = Response {
                    msg: String::from(
                        "You Cannot Delete a User with Greater Permissions than Yourself",
                    ),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Only super admin can delete themselves
            if (claims.user_id == id && claims.user_type_id != 1) {
                let response = Response {
                    msg: String::from("You Cannot Delete Your Own Account"),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            let owned_groups = sqlx::query!("SELECT id FROM chat_groups WHERE owner_id = $1", id)
                .fetch_all(&pool)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (!(owned_groups.is_empty())) {
                let response = Response {
                    msg: String::from(
                        "User Cannot be Deleted if they are the Owner of One or More Groups",
                    ),
                    success: false,
                };

                return Ok(HttpResponse::BadRequest().json(response));
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

            if (result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("User Deleted Successfully"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("User Not Deleted"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}
