use std::ptr::null;

use crate::state::AppState;
use crate::{auth::JwtClaims, extractors::user_type};
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct RelationshipRow {
    id: i64,
    requester_id: i64,
    receiver_id: i64,
    status_id: i64,
    blocked_by: Option<i64>,
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

pub async fn list_relationships(
    data: web::Data<AppState>,
    claims: JwtClaims,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationships = sqlx::query_as!(
        RelationshipRow,
        "SELECT id, requester_id, receiver_id, status_id, blocked_by
         FROM user_relationships
         WHERE requester_id = $1 OR receiver_id = $1
         ORDER BY id",
        claims.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (relationships.is_empty()) {
        let response = Response {
            msg: String::from("No Relationships Found"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let response = DataResponse {
            msg: String::from("Success"),
            data: relationships,
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn list_user_relationships(
    data: web::Data<AppState>,
    claims: JwtClaims,
    user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationships = sqlx::query_as!(
        RelationshipRow,
        "SELECT id, requester_id, receiver_id, status_id, blocked_by
         FROM user_relationships
         WHERE requester_id = $1 OR receiver_id = $1
         ORDER BY id",
        user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if (relationships.is_empty()) {
        let response = Response {
            msg: String::from("No Relationships Found"),
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let response = DataResponse {
            msg: String::from("Success"),
            data: relationships,
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn new_relationship(
    data: web::Data<AppState>,
    claims: JwtClaims,
    receiver_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    // Cannot send a friend request to yourself
    if (claims.user_id == receiver_id) {
        let response = Response {
            msg: String::from("You Cannot Send a Friend Request to Yourself"),
            success: false,
        };

        return Ok(HttpResponse::BadRequest().json(response));
    }

    let pool = data.db.to_owned();

    // Check relationship doesn't already exist
    let existing = sqlx::query!(
        "SELECT id FROM user_relationships
         WHERE (requester_id = $1 AND receiver_id = $2)
         OR (requester_id = $2 AND receiver_id = $1)",
        claims.user_id,
        receiver_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match existing {
        Some(_existing) => {
            let response = Response {
                msg: String::from("Already Have an Existing Relationship with this User"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        None => {
            let result = sqlx::query!(
                "INSERT INTO user_relationships (requester_id, receiver_id, status_id)
                 VALUES ($1, $2, $3)",
                claims.user_id,
                receiver_id,
                1
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("Friend Request Sent Successfully"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Failed to Send Friend Request"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn update_relationship(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
    accepted: bool,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationship = sqlx::query!(
        "SELECT id, requester_id, receiver_id, status_id, blocked_by, declined_by
         FROM user_relationships WHERE id = $1",
        relationship_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from("Relationship Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(relationship) => {
            // Only participants or admin can update
            if (relationship.requester_id != claims.user_id
                && relationship.receiver_id != claims.user_id
                && claims.user_type_id > global::ADMIN)
            {
                let response = Response {
                    msg: String::from("You are Not Part of this Relationship"),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            // Blocked relationships cannot be updated — use block_relationship to unblock
            if relationship.status_id == 3 {
                let response = Response {
                    msg: String::from(
                        "This Relationship is Blocked, Use the Unblock Feature to Restore It",
                    ),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            // If declined by admin — only admin can update it
            if (relationship.status_id == 4) {
                match relationship.declined_by {
                    Some(declined_by_id) => {
                        let decliner = sqlx::query!(
                            "SELECT user_type_id FROM users WHERE id = $1",
                            declined_by_id
                        )
                        .fetch_one(&pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                        if (decliner.user_type_id <= global::ADMIN
                            && claims.user_type_id > global::ADMIN)
                        {
                            let response = Response {
                                msg: String::from(
                                    "This Relationship was Declined by an Admin and Cannot be Updated",
                                ),
                                success: false,
                            };

                            return Ok(HttpResponse::Forbidden().json(response));
                        }
                    }
                    None => {
                        // Shouldn't happen — fall through
                    }
                }
            }

            // Already accepted and trying to accept again
            if (relationship.status_id == 2 && accepted) {
                let response = Response {
                    msg: String::from("This Freind Request is Already Accepted"),
                    success: false,
                };

                return Ok(HttpResponse::BadRequest().json(response));
            }

            // Already declined and trying to decline again — non admins only
            if (relationship.status_id == 4 && !accepted && claims.user_type_id > global::ADMIN) {
                let response = Response {
                    msg: String::from("This Friend Request is Already Declined"),
                    success: false,
                };

                return Ok(HttpResponse::BadRequest().json(response));
            }

            // Non admins — only receiver can accept or decline a pending request
            if (claims.user_type_id > global::ADMIN) {
                if (relationship.status_id == 1
                    && relationship.requester_id == claims.user_id
                    && elationship.receiver_id != claims.user_id)
                {
                    let response = Response {
                        msg: String::from(
                            "Only the Receiver of a Friend Request Can Accept or Decline It",
                        ),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }

                // Non admins cannot decline an already accepted relationship
                if (relationship.status_id == 2 && !accepted) {
                    let response = Response {
                        msg: String::from("You Cannot Decline an Already Accepted Friend Request"),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }
            }

            // accepted = true  → status 2, clear declined_by to NULL
            // accepted = false → status 4, set declined_by to claims.user_id
            let new_status: i64 = 0;
            let declined_by: Option<i64> = Some(0);
            if (accepted) {
                new_status = 2;
                declined_by = None;
            } else {
                new_status = 4;
                declined_by = Some(claims.user_id)
            }

            let result = sqlx::query!(
                "UPDATE user_relationships SET
                    status_id = $1,
                    declined_by = $2,
                    updated_at = NOW()
                 WHERE id = $3",
                new_status,
                declined_by,
                relationship_id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
                let msg = "";

                if (accepted) {
                    msg = "Relationship Accepted Successfully"
                } else {
                    msg = "Relationship Declined Successfully"
                };
                let response = Response {
                    msg: String::from(msg),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Relationship Not Updated"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn block_relationship(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
    block: bool,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationship = sqlx::query!(
        "SELECT id, requester_id, receiver_id, status_id, blocked_by
         FROM user_relationships WHERE id = $1",
        relationship_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from("Relationship Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(relationship) => {
            // Only participants can block/unblock
            if (rel.requester_id != claims.user_id && rel.receiver_id != claims.user_id) {
                let response = Response {
                    msg: String::from("You are Not Part of this Relationship"),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            if (block) {
                // Cannot block a pending relationship
                if relationship.status_id == 1 {
                    let response = Response {
                        msg: String::from(
                            "You Cannot Block a Pending Relationship, Accept it First or Decline it",
                        ),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }

                // Cannot block a declined relationship
                if relationship.status_id == 4 {
                    let response = Response {
                        msg: String::from("You Cannot Block a Declined Relationship"),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }

                // Cannot block an already blocked relationship
                if relationship.status_id == 3 {
                    if relationship.blocked_by == Some(claims.user_id) {
                        let response = Response {
                            msg: String::from("You Have Already Blocked this User"),
                            success: false,
                        };

                        return Ok(HttpResponse::Forbidden().json(response));
                    } else {
                        let response = Response {
                            msg: String::from(
                                "You are Blocked by this User, You Cannot make any Changes",
                            ),
                            success: false,
                        };

                        return Ok(HttpResponse::Forbidden().json(response));
                    }
                }

                // Must be accepted to block
                if (relationship.status_id == 2) {
                    let result = sqlx::query!(
                        "UPDATE user_relationships SET
                            status_id = 3,
                            blocked_by = $1,
                            updated_at = NOW()
                         WHERE id = $2",
                        claims.user_id,
                        relationship_id
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    if (result.rows_affected() > 0) {
                        let response = Response {
                            msg: String::from("User Blocked Successfully"),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    } else {
                        let response = Response {
                            msg: String::from("User Could Not be Blocked"),
                            success: false,
                        };

                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("You Can Only Block an Accepted Relationship"),
                        success: false,
                    };

                    Ok(HttpResponse::Forbidden().json(response))
                }
            } else {
                // Unblocking

                // Cannot unblock a pending relationship
                if (relationship.status_id == 1 || relationship.status_id == 2) {
                    let response = Response {
                        msg: String::from("This Relationship is Not Blocked"),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }

                // Cannot unblock a accepted or declined relationship
                if (relationship.status_id == 2 || relationship.status_id == 4) {
                    let response = Response {
                        msg: String::from("This Relationship is Not Blocked"),
                        success: false,
                    };

                    return Ok(HttpResponse::Forbidden().json(response));
                }

                // Must be blocked to unblock
                if (relationship.status_id == 3) {
                    match relationship.blocked_by {
                        None => {
                            // Shouldn't happen — fall through
                        }
                        Some(blocked_by_id) => {
                            // Only the blocker can unblock
                            if (blocked_by_id != claims.user_id) {
                                if (relationship.requester_id == claims.user_id) {
                                    let response = Response {
                                        msg: String::from("You Cannot Unblock Yourself"),
                                        success: false,
                                    };

                                    return Ok(HttpResponse::Forbidden().json(response));
                                } else if (relationship.requester_id != claims.user_id) {
                                    let response = Response {
                                        msg: String::from(
                                            "You Cannot Unblock a Relationship you are Not Part of",
                                        ),
                                        success: false,
                                    };

                                    return Ok(HttpResponse::Forbidden().json(response));
                                }
                            }
                        }
                    }

                    // Unblock — restore to accepted (2)
                    let result = sqlx::query!(
                        "UPDATE user_relationships SET
                            status_id = 2,
                            blocked_by = NULL,
                            updated_at = NOW()
                         WHERE id = $1",
                        relationship_id
                    )
                    .execute(&pool)
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                    if (result.rows_affected() > 0) {
                        let response = Response {
                            msg: String::from("User Unblocked Successfully"),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    } else {
                        let response = Response {
                            msg: String::from("User Could Not be Unblocked"),
                            success: false,
                        };

                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("This Relationship is Not Blocked"),
                        success: false,
                    };

                    Ok(HttpResponse::Forbidden().json(response))
                }
            }
        }
    }
}

pub async fn delete_relationship(
    data: web::Data<AppState>,
    claims: JwtClaims,
    relationship_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = data.db.to_owned();

    let relationship = sqlx::query!(
        "SELECT id, requester_id, receiver_id, status_id, blocked_by, declined_by
         FROM user_relationships WHERE id = $1",
        relationship_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    match relationship {
        None => {
            let response = Response {
                msg: String::from("Relationship Not Found"),
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
        Some(relationship) => {
            // If declined — check who declined it
            if (relationship.status_id) == 4 {
                match relationship.declined_by {
                    None => {}
                    Some(declined_by_id) => {
                        // Check if declined by an admin
                        let decliner = sqlx::query!(
                            "SELECT user_type_id FROM users WHERE id = $1",
                            declined_by_id
                        )
                        .fetch_one(&pool)
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

                        // If declined by admin — only admin can delete
                        if (decliner.user_type_id <= global::ADMIN
                            && claims.user_type_id > global::ADMIN)
                        {
                            let response = Response {
                                msg: String::from(
                                    "This Relationship was Declined by an Admin and Cannot be Deleted",
                                ),
                                success: false,
                            };

                            return Ok(HttpResponse::Forbidden().json(response));
                        }

                        // If declined by receiver — only receiver or admin can delete
                        if (declined_by_id == relationship.receiver_id
                            && claims.user_id != relationship.receiver_id
                            && claims.user_type_id > global::ADMIN)
                        {
                            let response = Response {
                                msg: String::from(
                                    "This Relationship was Declined, Only the Receiver or an Admin Can Delete It",
                                ),
                                success: false,
                            };

                            return Ok(HttpResponse::Forbidden().json(response));
                        }
                    }
                }
            }

            // If blocked — only the blocker or admin can delete
            if (relationship.status_id == 3) {
                match rel.blocked_by {
                    None => {}
                    Some(blocked_by_id) => {
                        if (blocked_by_id != claims.user_id && claims.user_type_id > global::ADMIN)
                        {
                            let response = Response {
                                msg: String::from(
                                    "Only the User Who Blocked it or an Admin Can Delete this Relationship",
                                ),
                                success: false,
                            };

                            return Ok(HttpResponse::Forbidden().json(response));
                        }
                    }
                }
            }

            // For pending or accepted — only participants or admin can delete
            if (rel.requester_id != claims.user_id
                && rel.receiver_id != claims.user_id
                && claims.user_type_id > global::ADMIN)
            {
                let response = Response {
                    msg: String::from("You do Not have a Relationship with this User"),
                    success: false,
                };

                return Ok(HttpResponse::Forbidden().json(response));
            }

            let result = sqlx::query!(
                "DELETE FROM user_relationships WHERE id = $1",
                relationship_id
            )
            .execute(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if (result.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("Relationship Deleted Successfully"),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Relationship Not Deleted"),
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}
