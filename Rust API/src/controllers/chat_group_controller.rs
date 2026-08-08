use crate::auth::JwtClaims;
use crate::extractors::{
    check_can_delete_message, check_can_update_message, errors, global, permission_error_message,
};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct GroupRow {
    id: i64,
    name: String,
    owner_id: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct MessageRow {
    id: i64,
    sender_id: i64,
    group_id: Option<i64>,
    message: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct GroupPermissionRow {
    id: i64,
    group_id: i64,
    user_id: i64,
    permission_type_id: i64,
}

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

pub async fn list_groups(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let groups = sqlx::query_as!(
        GroupRow,
        "SELECT cg.id, cg.name, cg.owner_id
         FROM chat_groups cg
         INNER JOIN chat_group_permissions cgp ON cgp.group_id = cg.id
         WHERE cgp.user_id = $1 AND cgp.permission_type_id != 4
         ORDER BY cg.id",
        claims.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if groups.is_empty() {
        let response = Response {
            msg: String::from("No Groups Found"),
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let response = DataResponse {
            msg: String::from("Success"),
            data: groups,
            success: true,
        };
        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn get_group(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let group = sqlx::query_as!(
        GroupRow,
        "SELECT id, name, owner_id FROM chat_groups WHERE id = $1",
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match group {
        None => {
            let response = Response {
                msg: String::from("Group Not Found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(group) => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: group,
                success: true,
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn new_group(
    data: web::Data<AppState>,
    claims: JwtClaims,
    name: String,
) -> Result<HttpResponse, actix_web::Error> {
    if name.trim().is_empty() {
        let response = Response {
            msg: String::from("Group Name must Not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    // Insert group and get id back
    let group = sqlx::query!(
        "INSERT INTO chat_groups (name, owner_id) VALUES ($1, $2) RETURNING id",
        name,
        claims.user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // Add creator as moderator
    let result = sqlx::query!(
        "INSERT INTO chat_group_permissions (group_id, user_id, permission_type_id)
         VALUES ($1, $2, $3)",
        group.id,
        claims.user_id,
        1i64 // moderator
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if result.rows_affected() > 0 {
        let response = Response {
            msg: String::from("Group Created Successfully"),
            success: true,
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Failed to Create Group"),
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn update_group(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    name: String,
) -> Result<HttpResponse, actix_web::Error> {
    if name.trim().is_empty() {
        let response = Response {
            msg: String::from("Group Name must Not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    // Only owner or admin can update group
    let group = sqlx::query!(
        "SELECT id, owner_id FROM chat_groups WHERE id = $1",
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match group {
        None => {
            let response = Response {
                msg: String::from("Group Not Found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(group) => {
            if group.owner_id != claims.user_id && claims.user_type_id > global::ADMIN {
                let response = Response {
                    msg: String::from("Only the Group Owner or an Admin Can Update This Group"),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            let result = sqlx::query!(
                "UPDATE chat_groups SET name = $1, updated_by = $2, updated_at = NOW()
                 WHERE id = $3",
                name,
                claims.user_id,
                group_id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if result.rows_affected() > 0 {
                let response = Response {
                    msg: String::from("Group Updated Successfully"),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Group Not Updated"),
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn delete_group(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let group = sqlx::query!(
        "SELECT id, owner_id FROM chat_groups WHERE id = $1",
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match group {
        None => {
            let response = Response {
                msg: String::from("Group Not Found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(group) => {
            if group.owner_id != claims.user_id && claims.user_type_id > global::ADMIN {
                let response = Response {
                    msg: String::from("Only the Group Owner or an Admin Can Delete This Group"),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            let mut tx = pool
                .begin()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete messages
            sqlx::query!("DELETE FROM messages WHERE group_id = $1", group_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete permissions
            sqlx::query!(
                "DELETE FROM chat_group_permissions WHERE group_id = $1",
                group_id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            // Delete group
            let result = sqlx::query!("DELETE FROM chat_groups WHERE id = $1", group_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            tx.commit()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if result.rows_affected() > 0 {
                let response = Response {
                    msg: String::from("Group Deleted Successfully"),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Group Not Deleted"),
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn list_messages(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let messages = sqlx::query_as!(
        MessageRow,
        "SELECT id, sender_id, group_id, message, created_at, updated_at
         FROM messages
         WHERE group_id = $1
         ORDER BY created_at ASC",
        group_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let response = DataResponse {
        msg: String::from("Success"),
        data: messages,
        success: true,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn send_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    message: String,
) -> Result<HttpResponse, actix_web::Error> {
    if message.trim().is_empty() {
        let response = Response {
            msg: String::from("Message must Not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    // Check user has permission to send in this group
    let permission = sqlx::query!(
        "SELECT permission_type_id FROM chat_group_permissions
         WHERE group_id = $1 AND user_id = $2",
        group_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match permission {
        None => {
            let response = Response {
                msg: String::from("You are Not a Member of this Group"),
                success: false,
            };
            Ok(HttpResponse::Forbidden().json(response))
        }
        Some(perm) => {
            // Viewers and blocked cannot send messages
            if perm.permission_type_id >= 3 {
                let response = Response {
                    msg: String::from("You do Not have Permission to Send Messages in this Group"),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Viewers globally cannot send messages
            if claims.user_type_id >= global::VIEWER {
                let response = Response {
                    msg: String::from("You do Not have Permission to Send Messages"),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            let result = sqlx::query!(
                "INSERT INTO messages (sender_id, group_id, message) VALUES ($1, $2, $3)",
                claims.user_id,
                group_id,
                message
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if result.rows_affected() > 0 {
                let response = Response {
                    msg: String::from("Message Sent Successfully"),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Failed to Send Message"),
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn update_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    message_id: i64,
    group_id: i64,
    message: String,
) -> Result<HttpResponse, actix_web::Error> {
    if message.trim().is_empty() {
        let response = Response {
            msg: String::from("Message must Not be empty"),
            success: false,
        };
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let existing = sqlx::query!(
        "SELECT id, sender_id FROM messages WHERE id = $1 AND group_id = $2",
        message_id,
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match existing {
        None => {
            let response = Response {
                msg: String::from("Message Not Found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(msg) => {
            if !check_can_update_message(msg.sender_id, claims.user_id) {
                return Ok(HttpResponse::Forbidden()
                    .body(permission_error_message(errors::UPDATE_MESSAGE_NOT_OWNER)));
            }

            let result = sqlx::query!(
                "UPDATE messages SET message = $1, updated_at = NOW() WHERE id = $2",
                message,
                message_id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if result.rows_affected() > 0 {
                let response = Response {
                    msg: String::from("Message Updated Successfully"),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Message Not Updated"),
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn delete_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    message_id: i64,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let existing = sqlx::query!(
        "SELECT id, sender_id FROM messages WHERE id = $1 AND group_id = $2",
        message_id,
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match existing {
        None => {
            let response = Response {
                msg: String::from("Message Not Found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(msg) => {
            // Get group permission for delete check
            let group_perm = sqlx::query!(
                "SELECT permission_type_id FROM chat_group_permissions
                 WHERE group_id = $1 AND user_id = $2",
                group_id,
                claims.user_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            let group_permission = group_perm.map(|p| p.permission_type_id);

            if !check_can_delete_message(
                msg.sender_id,
                claims.user_id,
                claims.user_type_id,
                group_permission,
            ) {
                return Ok(HttpResponse::Forbidden()
                    .body(permission_error_message(errors::DELETE_MESSAGE_NOT_OWNER)));
            }

            let result = sqlx::query!("DELETE FROM messages WHERE id = $1", message_id)
                .execute(&pool)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if result.rows_affected() > 0 {
                let response = Response {
                    msg: String::from("Message Deleted Successfully"),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Message Not Deleted"),
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn list_group_permissions(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let permissions = sqlx::query_as!(
        GroupPermissionRow,
        "SELECT id, group_id, user_id, permission_type_id
         FROM chat_group_permissions
         WHERE group_id = $1
         ORDER BY id",
        group_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let response = DataResponse {
        msg: String::from("Success"),
        data: permissions,
        success: true,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn add_group_permission(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    user_id: i64,
    permission_type_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    // Check caller is owner or admin
    let group = sqlx::query!("SELECT owner_id FROM chat_groups WHERE id = $1", group_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match group {
        None => {
            let response = Response {
                msg: String::from("Group Not Found"),
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(group) => {
            // Check caller permission in group
            let caller_perm = sqlx::query!(
                "SELECT permission_type_id FROM chat_group_permissions
                 WHERE group_id = $1 AND user_id = $2",
                group_id,
                claims.user_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            let is_moderator = caller_perm
                .map(|p| p.permission_type_id == 1)
                .unwrap_or(false);
            let is_admin = claims.user_type_id <= global::ADMIN;

            if !is_moderator && !is_admin {
                let response = Response {
                    msg: String::from("You do Not have Permission to Add Members to this Group"),
                    success: false,
                };
                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Check user not already in group
            let existing = sqlx::query!(
                "SELECT id FROM chat_group_permissions WHERE group_id = $1 AND user_id = $2",
                group_id,
                user_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match existing {
                Some(_) => {
                    let response = Response {
                        msg: String::from("User is Already a Member of this Group"),
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
                None => {
                    let result = sqlx::query!(
                        "INSERT INTO chat_group_permissions
                         (group_id, user_id, permission_type_id, updated_by)
                         VALUES ($1, $2, $3, $4)",
                        group_id,
                        user_id,
                        permission_type_id,
                        claims.user_id
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    if result.rows_affected() > 0 {
                        let response = Response {
                            msg: String::from("Member Added Successfully"),
                            success: true,
                        };
                        Ok(HttpResponse::Ok().json(response))
                    } else {
                        let response = Response {
                            msg: String::from("Failed to Add Member"),
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                }
            }
        }
    }
}

pub async fn update_group_permission(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    user_id: i64,
    permission_type_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    // Check caller is moderator or admin
    let caller_perm = sqlx::query!(
        "SELECT permission_type_id FROM chat_group_permissions
         WHERE group_id = $1 AND user_id = $2",
        group_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let is_moderator = caller_perm
        .map(|p| p.permission_type_id == 1)
        .unwrap_or(false);
    let is_admin = claims.user_type_id <= global::ADMIN;

    if !is_moderator && !is_admin {
        let response = Response {
            msg: String::from("You do Not have Permission to Update Members in this Group"),
            success: false,
        };
        return Ok(HttpResponse::Forbidden().json(response));
    }

    let result = sqlx::query!(
        "UPDATE chat_group_permissions SET
            permission_type_id = $1,
            updated_by = $2,
            updated_at = NOW()
         WHERE group_id = $3 AND user_id = $4",
        permission_type_id,
        claims.user_id,
        group_id,
        user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if result.rows_affected() > 0 {
        let response = Response {
            msg: String::from("Member Permission Updated Successfully"),
            success: true,
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Member Permission Not Updated"),
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn delete_group_permission(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    // Check caller is moderator or admin
    let caller_perm = sqlx::query!(
        "SELECT permission_type_id FROM chat_group_permissions
         WHERE group_id = $1 AND user_id = $2",
        group_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let is_moderator = caller_perm
        .map(|p| p.permission_type_id == 1)
        .unwrap_or(false);
    let is_admin = claims.user_type_id <= global::ADMIN;

    // User can also remove themselves
    let is_self = claims.user_id == user_id;

    if !is_moderator && !is_admin && !is_self {
        let response = Response {
            msg: String::from("You do Not have Permission to Remove Members from this Group"),
            success: false,
        };
        return Ok(HttpResponse::Forbidden().json(response));
    }

    let result = sqlx::query!(
        "DELETE FROM chat_group_permissions WHERE group_id = $1 AND user_id = $2",
        group_id,
        user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if result.rows_affected() > 0 {
        let response = Response {
            msg: String::from("Member Removed Successfully"),
            success: true,
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Member Not Removed"),
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}
