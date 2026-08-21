use crate::auth::JwtClaims;
use crate::encryption::{decrypt_message, encrypt_message};
use crate::extractors::{errors, group_permission, permission_error_message, user_type};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct GroupRow {
    id: i64,
    name: String,
    updated_by: Option<i64>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct GroupMemberRow {
    id: i64,
    username: String,
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

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct PermissionTypeRow {
    id: i64,
    permission_type: String,
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseEmptyList {
    msg: String,
    empty: bool,
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

    if (claims.user_type_id <= user_type::ADMIN) {
        let groups = sqlx::query_as!(
            GroupRow,
            "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups"
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

        if (groups.is_empty()) {
            let response = ResponseEmptyList {
                msg: String::from("No Groups Found"),
                empty: true,
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
    } else {
        let groups = sqlx::query_as!(
            GroupRow,
            "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups WHERE id IN (SELECT group_id FROM chat_group_permissions WHERE user_id = $1)",
            claims.user_id
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

        if (groups.is_empty()) {
            let response = ResponseEmptyList {
                msg: String::from("No Groups Found"),
                empty: true,
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
}

pub async fn search_groups(
    data: web::Data<AppState>,
    claims: JwtClaims,
    search_name: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (search_name.is_empty()) {
        let response = Response {
            msg: String::from("Must Provide Group Name to Search for Groups"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    } else if (search_name.len() < 3) {
        let response = Response {
            msg: String::from("Provided Search must be 3 or More Characters"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let search_like = format!("%{}%", search_name);

    let pool = data.db.to_owned();

    if (claims.user_type_id <= user_type::ADMIN) {
        let groups = sqlx::query_as!(
            GroupRow,
            "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups WHERE name LIKE $1",
            search_like
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

        if (groups.is_empty()) {
            let response = ResponseEmptyList {
                msg: String::from("No Groups Found"),
                empty: true,
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
    } else {
        let groups = sqlx::query_as!(
            GroupRow,
            "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups WHERE name LIKE $2 AND id IN (SELECT group_id FROM chat_group_permissions WHERE user_id = $1)",
            claims.user_id,
            search_like
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

        if (groups.is_empty()) {
            let response = ResponseEmptyList {
                msg: String::from("No Groups Found"),
                empty: true,
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
}

pub async fn list_user_groups(
    data: web::Data<AppState>,
    claims: JwtClaims,
    user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let groups = sqlx::query_as!(
            GroupRow,
            "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups WHERE id IN (SELECT group_id FROM chat_group_permissions WHERE user_id = $1)",
            user_id
        ) .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (groups.is_empty()) {
        let response = ResponseEmptyList {
            msg: String::from("No Groups Found for the User Selected"),
            empty: true,
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

pub async fn search_user_groups(
    data: web::Data<AppState>,
    claims: JwtClaims,
    user_id: i64,
    search_name: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (search_name.is_empty()) {
        let response = Response {
            msg: String::from("Must Provide Group Name to Search for Groups"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    } else if (search_name.len() < 3) {
        let response = Response {
            msg: String::from("Provided Search must be 3 or More Characters"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let search_like = format!("%{}%", search_name);

    let pool = data.db.to_owned();

    let groups = sqlx::query_as!(
            GroupRow,
            "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups WHERE name LIKE $2 AND id IN (SELECT group_id FROM chat_group_permissions WHERE user_id = $1)",
            user_id,
            search_like
        ) .fetch_all(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (groups.is_empty()) {
        let response = ResponseEmptyList {
            msg: String::from("No Groups Found for the User Selected"),
            empty: true,
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

pub async fn list_group_members(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let members = sqlx::query_as!(
      GroupMemberRow,
        "SELECT id, username FROM users WHERE id IN (SELECT user_id FROM chat_group_permissions WHERE group_id = $1)",
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match members {
        None => {
            let response = Response {
                msg: String::from("No Group Members Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(members) => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: members,
                success: true,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn list_non_group_members(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let members = sqlx::query_as!(
      GroupMemberRow,
        "SELECT id, username FROM users WHERE id NOT IN (SELECT user_id FROM chat_group_permissions WHERE group_id = $1)",
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match members {
        None => {
            let response = ResponseEmptyList {
                msg: String::from("No Group Non-Members Found"),
                empty: true,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(members) => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: members,
                success: true,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn list_users_who_sent_group_messages(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let members = sqlx::query_as!(
      GroupMemberRow,
        "SELECT id, username FROM users WHERE id IN (SELECT sender_id FROM messages WHERE group_id = $1)",
        group_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match members {
        None => {
            let response = ResponseEmptyList {
                msg: String::from("No Users Who Sent Messages in this Group Found"),
                empty: true,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(members) => {
            let response = DataResponse {
                msg: String::from("Success"),
                data: members,
                success: true,
            };

            Ok(HttpResponse::Ok().json(response))
        }
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
        "SELECT id, name, updated_by, created_at, updated_at FROM chat_groups WHERE id = $1",
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
    if (name.trim().is_empty()) {
        let response = Response {
            msg: String::from("Group Name must Not be empty"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let result = sqlx::query!(
        "INSERT INTO chat_groups (name) VALUES ($1) RETURNING id",
        name,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match result {
        None => {
            let response = Response {
                msg: String::from("Failed to Create Group"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(new_group) => {
            let new_permission = sqlx::query!(
                "INSERT INTO chat_group_permissions (group_id, user_id, permission_type_id)
         VALUES ($1, $2, $3)",
                new_group.id,
                claims.user_id,
                1
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (new_permission.rows_affected() > 0) {
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
    }
}

pub async fn update_group(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    name: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (name.trim().is_empty()) {
        let response = Response {
            msg: String::from("Group Name must Not be empty"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let group = sqlx::query!("SELECT id FROM chat_groups WHERE id = $1", group_id)
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
        Some(existing_group) => {
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

            if (result.rows_affected() > 0) {
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

    let group = sqlx::query!("SELECT id FROM chat_groups WHERE id = $1", group_id)
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
            let mut tx = pool
                .begin()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            sqlx::query!("DELETE FROM messages WHERE group_id = $1", group_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            sqlx::query!(
                "DELETE FROM chat_group_permissions WHERE group_id = $1",
                group_id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            let result = sqlx::query!("DELETE FROM chat_groups WHERE id = $1", group_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            tx.commit()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
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

    let chat_messages = sqlx::query_as!(
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

    match chat_messages.is_empty() {
        true => {
            let response = ResponseEmptyList {
                msg: String::from("No Messages Found"),
                empty: true,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        false => {
            let mut decrypted_messages: Vec<MessageRow> = Vec::new();

            for mut chat_message in chat_messages {
                let mut decrypted_message = match decrypt_message(&chat_message.message) {
                    Ok(decypted_text) => decypted_text,
                    Err(_err) => {
                        let response = Response {
                            msg: String::from("Messages Could Not be Decrypted"),
                            success: true,
                        };

                        return Ok(HttpResponse::BadRequest().json(response));
                    }
                };

                chat_message.message = decrypted_message;

                decrypted_messages.push(chat_message);
            }

            let response = DataResponse {
                msg: String::from("Success"),
                data: decrypted_messages,
                success: true,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn search_messages(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    message_content: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (message_content.is_empty()) {
        let response = Response {
            msg: String::from("Must Provide Text to Search for Messages"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    } else if (message_content.len() < 3) {
        let response = Response {
            msg: String::from("Provided Search must be 3 or More Characters"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let chat_messages = sqlx::query_as!(
        MessageRow,
        "SELECT id, sender_id, group_id, message, created_at, updated_at
         FROM messages
         WHERE group_id = $1 AND message like $2
         ORDER BY created_at ASC",
        group_id,
        message_content
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match chat_messages.is_empty() {
        true => {
            let response = ResponseEmptyList {
                msg: String::from("No Messages Found"),
                empty: true,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        false => {
            let mut decrypted_messages: Vec<MessageRow> = Vec::new();

            for mut chat_message in chat_messages {
                let mut decrypted_message = match decrypt_message(&chat_message.message) {
                    Ok(decypted_text) => decypted_text,
                    Err(_err) => {
                        let response = Response {
                            msg: String::from("Messages Could Not be Decrypted"),
                            success: true,
                        };

                        return Ok(HttpResponse::BadRequest().json(response));
                    }
                };

                chat_message.message = decrypted_message;

                decrypted_messages.push(chat_message);
            }

            let response = DataResponse {
                msg: String::from("Success"),
                data: decrypted_messages,
                success: true,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    }
}

pub async fn send_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    message: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (message.trim().is_empty()) {
        let response = Response {
            msg: String::from("Message must Not be empty"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    let encrypted_message = match encrypt_message(message.as_str()) {
        Ok(ecrypted_text) => ecrypted_text,
        Err(_err) => {
            let response = Response {
                msg: String::from("Message Could Not be Encrypted"),
                success: true,
            };

            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    let result = sqlx::query!(
        "INSERT INTO messages (sender_id, group_id, message) VALUES ($1, $2, $3)",
        claims.user_id,
        group_id,
        encrypted_message
    )
    .execute(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (result.rows_affected() > 0) {
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

pub async fn update_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    message_id: i64,
    group_id: i64,
    message: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (message.trim().is_empty()) {
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
        Some(existing_message) => {
            if (existing_message.sender_id != claims.user_id) {
                let response = Response {
                    msg: String::from("You Cannot Edit a Message that was Not Sent by You"),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            let result = sqlx::query!(
                "UPDATE messages SET message = $1, updated_at = NOW() WHERE id = $2",
                message,
                message_id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
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
        Some(existing_message) => {
            if (existing_message.sender_id != claims.user_id
                && claims.user_type_id > user_type::ADMIN)
            {
                let response = Response {
                    msg: String::from("You Cannot Delete a Message that was Not Sent by You"),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            let result = sqlx::query!("DELETE FROM messages WHERE id = $1", message_id)
                .execute(&pool)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
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

    if (permissions.is_empty()) {
        let response = Response {
            msg: String::from("No Group Permissions Found"),
            success: false,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = DataResponse {
            msg: String::from("Success"),
            data: permissions,
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn list_group_permission_types(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let permission_types = sqlx::query_as!(
        PermissionTypeRow,
        "SELECT id, permission_type
         FROM chat_group_permission_types
         ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (permission_types.is_empty()) {
        let response = Response {
            msg: String::from("No Group Permission Types Found"),
            success: false,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = DataResponse {
            msg: String::from("Success"),
            data: permission_types,
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn add_group_permission(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    user_id: i64,
    permission_type_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let existing = sqlx::query!("SELECT id FROM chat_groups WHERE id = $1", group_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match existing {
        None => {
            let response = Response {
                msg: String::from("Group Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(existing_group) => {
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

                    if (result.rows_affected() > 0) {
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

    let editors_group_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM chat_group_permissions WHERE group_id = $1 AND user_id = $2",
        group_id,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match editors_group_permission {
        None => {
            let response = Response {
                msg: String::from("You Do Not have any Permissions in this Group"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(existing_editors_group_permission) => {
            let group_permission = sqlx::query!(
                "SELECT id, permission_type_id FROM chat_group_permissions WHERE id = $1 AND group_id = $2 AND user_id = $3",
                permission_type_id,
                group_id,
                user_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match group_permission {
                None => {
                    let response = Response {
                        msg: String::from(
                            "The Group Permission you are trying to Update does not Exist",
                        ),
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
                Some(existing_group_permission) => {
                    if (existing_group_permission.permission_type_id == group_permission::OWNER
                        && existing_editors_group_permission.permission_type_id
                            != group_permission::OWNER)
                    {
                        let response = Response {
                msg: String::from("You Cannot Edit an Owner's Group Permission if you are not an Owner of the Group"),
                success: false,
            };

                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let owners = sqlx::query!(
                            "SELECT id
         FROM chat_group_permissions WHERE permission_type_id = 4 AND group_id = $1
         ORDER BY id",
                            group_id
                        )
                        .fetch_all(&pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                        if (owners.is_empty()) {
                            let response = Response {
                                msg: String::from("No Owner Group Permissions List Found"),
                                success: false,
                            };

                            return Ok(HttpResponse::Ok().json(response));
                        } else if (owners.len() == 1) {
                            let response = Response {
            msg: String::from("You cannot Edit an Group Owner's Permission if there is only one Current Group Owner"),
            success: false,
        };

                            return Ok(HttpResponse::Ok().json(response));
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

                        if (result.rows_affected() > 0) {
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
                }
            }
        }
    }
}

pub async fn delete_group_permission(
    data: web::Data<AppState>,
    claims: JwtClaims,
    group_id: i64,
    user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let deletors_group_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM chat_group_permissions WHERE group_id = $1 AND user_id = $2",
        group_id,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match deletors_group_permission {
        None => {
            let response = Response {
                msg: String::from("You Do Not have any Permissions in this Group"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(existing_deletors_group_permission) => {
            let group_permission = sqlx::query!(
                "SELECT id, permission_type_id FROM chat_group_permissions WHERE group_id = $1 AND user_id = $2",
                group_id,
                user_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            match group_permission {
                None => {
                    let response = Response {
                        msg: String::from(
                            "The Group Permission you are trying to Delete does not Exist",
                        ),
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
                Some(existing_group_permission) => {
                    if (existing_group_permission.permission_type_id == group_permission::OWNER
                        && existing_deletors_group_permission.permission_type_id
                            != group_permission::OWNER)
                    {
                        let response = Response {
                msg: String::from("You Cannot Delete an Owner's Group Permission if you are not an Owner of the Group"),
                success: false,
            };

                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let owners = sqlx::query!(
                            "SELECT id
         FROM chat_group_permissions WHERE permission_type_id = 4 AND group_id = $1
         ORDER BY id",
                            group_id
                        )
                        .fetch_all(&pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                        if (owners.is_empty()) {
                            let response = Response {
                                msg: String::from("No Owner Group Permissions List Found"),
                                success: false,
                            };

                            return Ok(HttpResponse::Ok().json(response));
                        } else if (owners.len() == 1) {
                            let response = Response {
            msg: String::from("You cannot Delete your Group Owner Permission if You are the Only Group Owner"),
            success: false,
        };

                            return Ok(HttpResponse::Ok().json(response));
                        }

                        let result = sqlx::query!(
                            "DELETE FROM chat_group_permissions WHERE group_id = $1 AND user_id = $2",
                            group_id,
                            user_id,
                        )
                        .execute(&pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                        if (result.rows_affected() > 0) {
                            let response = Response {
                                msg: String::from("Member Permission Deleted Successfully"),
                                success: true,
                            };

                            Ok(HttpResponse::Ok().json(response))
                        } else {
                            let response = Response {
                                msg: String::from("Member Permission Not Deleted"),
                                success: false,
                            };

                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    }
                }
            }
        }
    }
}
