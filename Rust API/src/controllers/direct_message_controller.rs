use crate::auth::JwtClaims;
use crate::encryption::{decrypt_message, encrypt_message};
use crate::extractors::{errors, permission_error_message, user_type};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Serialize;

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

pub async fn list_messages(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationship = sqlx::query!(
        "SELECT id, status_id, blocked_by, declined_by FROM user_relationships
         WHERE id = $1 AND (requester_id = $2 OR receiver_id = $2)",
        relationship_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from(
                    "Relationship to Messaged User Not Found, It May No Longer Exist",
                ),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(_relationship) => {
            if (!(_relationship.blocked_by.is_none()) && _relationship.status_id == 3) {
                if (_relationship.blocked_by != Some(claims.user_id)) {
                    let response = Response {
                        msg: String::from("You have been Blocked by this User, so you may not view Messages between You and this User"),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }
            }

            if (!(_relationship.declined_by.is_none()) && _relationship.status_id == 4) {
                if (_relationship.declined_by != Some(claims.user_id)) {
                    let declined_by_user = sqlx::query!(
                        "SELECT id, user_type_id FROM users WHERE id = $1",
                        _relationship.declined_by
                    )
                    .fetch_optional(&pool)
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    match declined_by_user {
                        None => {}
                        Some(_declined_by_user) => {
                            if (_declined_by_user.user_type_id <= user_type::ADMIN) {
                                let response = Response {
                        msg: String::from("Your Relationship has been declined by an Admin, so you may not view Messages between You and this User"),
                        success: false,
                    };

                                return Ok(HttpResponse::Forbidden().json(response));
                            } else {
                                let response = Response {
                        msg: String::from("Your Relationship has been declined, so you may not view Messages between You and this User"),
                        success: false,
                    };

                                return Ok(HttpResponse::Forbidden().json(response));
                            }
                        }
                    }
                }
            }

            let dm_messages = sqlx::query_as!(
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

            match dm_messages.is_empty() {
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

                    for mut dm_message in dm_messages {
                        let mut decrypted_message = match decrypt_message(&dm_message.message) {
                            Ok(decypted_text) => decypted_text,
                            Err(_err) => {
                                let response = Response {
                                    msg: String::from("Messages Could Not be Decrypted"),
                                    success: true,
                                };

                                return Ok(HttpResponse::BadRequest().json(response));
                            }
                        };

                        dm_message.message = decrypted_message;

                        decrypted_messages.push(dm_message);
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
    }
}

pub async fn search_messages(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
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

    let relationship = sqlx::query!(
        "SELECT id, status_id, blocked_by, declined_by FROM user_relationships
         WHERE id = $1 AND (requester_id = $2 OR receiver_id = $2)",
        relationship_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from(
                    "Relationship to Messaged User Not Found, It May No Longer Exist",
                ),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(_relationship) => {
            if (!(_relationship.blocked_by.is_none()) && _relationship.status_id == 3) {
                if (_relationship.blocked_by != Some(claims.user_id)) {
                    let response = Response {
                        msg: String::from("You have been Blocked by this User, so you may not view Messages between You and this User"),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }
            }

            if (!(_relationship.declined_by.is_none()) && _relationship.status_id == 4) {
                if (_relationship.declined_by != Some(claims.user_id)) {
                    let declined_by_user = sqlx::query!(
                        "SELECT id, user_type_id FROM users WHERE id = $1",
                        _relationship.declined_by
                    )
                    .fetch_optional(&pool)
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    match declined_by_user {
                        None => {}
                        Some(_declined_by_user) => {
                            if (_declined_by_user.user_type_id <= user_type::ADMIN) {
                                let response = Response {
                        msg: String::from("Your Relationship has been declined by an Admin, so you may not view Messages between You and this User"),
                        success: false,
                    };

                                return Ok(HttpResponse::Forbidden().json(response));
                            } else {
                                let response = Response {
                        msg: String::from("Your Relationship has been declined, so you may not view Messages between You and this User"),
                        success: false,
                    };

                                return Ok(HttpResponse::Forbidden().json(response));
                            }
                        }
                    }
                }
            }

            let dm_messages = sqlx::query_as!(
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

            match dm_messages.is_empty() {
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

                    for mut dm_message in dm_messages {
                        let mut decrypted_message = match decrypt_message(&dm_message.message) {
                            Ok(decypted_text) => decypted_text,
                            Err(_err) => {
                                let response = Response {
                                    msg: String::from("Messages Could Not be Decrypted"),
                                    success: true,
                                };

                                return Ok(HttpResponse::BadRequest().json(response));
                            }
                        };

                        if (decrypted_message.contains(&message_content)) {
                            dm_message.message = decrypted_message;

                            decrypted_messages.push(dm_message);
                        }
                    }

                    if (decrypted_messages.is_empty()) {
                        let response = ResponseEmptyList {
                            msg: String::from("No Messages Found for the Filter Content Provided"),
                            empty: true,
                            success: false,
                        };

                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let response = DataResponse {
                            msg: String::from("Success"),
                            data: decrypted_messages,
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    }
                }
            }
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

    let relationship = sqlx::query!(
        "SELECT id, status_id FROM user_relationships
         WHERE id = $1 AND (requester_id = $2 OR receiver_id = $2)",
        relationship_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from(
                    "Relationship with the User You are trying to Send a Message to Not Found",
                ),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(_relationship) => {
            if (_relationship.status_id != 2) {
                let response = Response {
                    msg: String::from(
                        "You Cannot Send a Message to an Unaccepted/Blocked/Inactive Relationship",
                    ),
                    success: false,
                };

                Ok(HttpResponse::Forbidden().json(response))
            } else {
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
                    "INSERT INTO messages (sender_id, relationship_id, message)
                 VALUES ($1, $2, $3)",
                    claims.user_id,
                    relationship_id,
                    encrypted_message
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

    let relationship = sqlx::query!(
        "SELECT id, status_id, blocked_by, declined_by FROM user_relationships
         WHERE id = $1 AND (requester_id = $1 OR receiver_id = $1)",
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from(
                    "Relationship with the User You are trying to Send a Message to Not Found",
                ),
                success: false,
            };

            return Ok(HttpResponse::BadRequest().json(response));
        }
        Some(_relationship) => {
            if ((_relationship.status_id != 2 && _relationship.blocked_by != Some(claims.user_id))
                || (_relationship.status_id != 2
                    && _relationship.declined_by != Some(claims.user_id)))
            {
                let response = Response {
                    msg: String::from(
                        "You Cannot Edit a Message from an Blocked/Inactive Relationship if you were not the one who Blocked/Declined it",
                    ),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

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
                Some(existing_message) => {
                    if (existing_message.sender_id != claims.user_id) {
                        let response = Response {
                            msg: String::from("You Cannot Edit a Message that was Not Sent by You"),
                            success: false,
                        };

                        return Ok(HttpResponse::Forbidden().json(response));
                    }

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
                        "UPDATE messages SET message = $1, updated_at = NOW() WHERE id = $2",
                        encrypted_message,
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
    }
}

pub async fn delete_message(
    data: web::Data<AppState>,
    claims: JwtClaims,
    message_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationship = sqlx::query!(
        "SELECT id, status_id, blocked_by, declined_by FROM user_relationships
         WHERE id = $1 AND (requester_id = $2 OR receiver_id = $2)",
        message_id,
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from(
                    "Relationship with the User You are trying to Send a Message to Not Found",
                ),
                success: false,
            };

            return Ok(HttpResponse::BadRequest().json(response));
        }
        Some(_relationship) => {
            if ((_relationship.status_id != 2 && _relationship.blocked_by != Some(claims.user_id))
                || (_relationship.status_id != 2
                    && _relationship.declined_by != Some(claims.user_id)))
            {
                let response = Response {
                    msg: String::from(
                        "You Cannot Delete a Message from an Blocked/Inactive Relationship if you were not the one who Blocked/Declined it",
                    ),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

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
                Some(existing_message) => {
                    if (existing_message.sender_id != claims.user_id) {
                        let response = Response {
                            msg: String::from(
                                "You Cannot Delete a Message that was Not Sent by You",
                            ),
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
    }
}
