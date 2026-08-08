use crate::auth::JwtClaims;
use crate::extractors::{
    check_can_delete_message, check_can_update_message, errors, permission_error_message,
};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct ConversationRow {
    id: i64,
    requester_id: i64,
    receiver_id: i64,
    status_id: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct MessageRow {
    id: i64,
    sender_id: i64,
    relationship_id: Option<i64>,
    message: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
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

pub async fn list_conversations(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let conversations = sqlx::query_as!(
        ConversationRow,
        "SELECT id, requester_id, receiver_id, status_id
         FROM user_relationships
         WHERE (requester_id = $1 OR receiver_id = $1)
         AND status_id = 2
         ORDER BY id",
        claims.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (conversations.is_empty()) {
        let response = Response {
            msg: String::from("No Conversations Found"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let response = DataResponse {
            msg: String::from("Success"),
            data: conversations,
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn list_messages(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    // Verify user is part of this relationship
    let relationship = sqlx::query!(
        "SELECT id FROM user_relationships
         WHERE id = $1 AND (requester_id = $2 OR receiver_id = $2)
         AND status_id = 2",
        relationship_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from("Conversation Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(_relationship) => {
            let messages = sqlx::query_as!(
                MessageRow,
                "SELECT id, sender_id, relationship_id, message, created_at, updated_at
                 FROM messages
                 WHERE relationship_id = $1
                 ORDER BY created_at ASC",
                relationship_id
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
    }
}

pub async fn send_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
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

    // Verify user is part of this relationship and it is accepted
    let relationship = sqlx::query!(
        "SELECT id FROM user_relationships
         WHERE id = $1 AND (requester_id = $2 OR receiver_id = $2)
         AND status_id = 2",
        relationship_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from("Conversation Not Found or Not Accepted"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(_) => {
            let result = sqlx::query!(
                "INSERT INTO messages (sender_id, relationship_id, message)
                 VALUES ($1, $2, $3)",
                claims.user_id,
                relationship_id,
                message
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected()) > 0 {
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
        "SELECT id, sender_id FROM messages WHERE id = $1 AND group_id IS NULL",
        message_id
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
            if (!check_can_update_message(msg.sender_id, claims.user_id)) {
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
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let existing = sqlx::query!(
        "SELECT id, sender_id FROM messages WHERE id = $1 AND group_id IS NULL",
        message_id
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
            // For DMs — sender or global admin can delete
            if (!check_can_delete_message(msg.sender_id, claims.user_id, claims.user_type_id, None))
            {
                return Ok(HttpResponse::Forbidden()
                    .body(permission_error_message(errors::DELETE_MESSAGE_NOT_OWNER)));
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
