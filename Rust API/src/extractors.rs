use crate::auth::{verify_access_token, JwtClaims};
use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized},
    FromRequest, HttpRequest,
};
use std::future::Future;
use std::pin::Pin;

// --- GLOBAL PERMISSION CONSTANTS ---

pub mod user_type {
    pub const SUPER_ADMIN: i64 = 1;
    pub const ADMIN: i64 = 2;
    pub const STANDARD_USER: i64 = 3;
    pub const VIEWER: i64 = 4;
    pub const BLOCKED: i64 = 5;
}

pub mod group_permission {
    pub const MODERATOR: i64 = 1;
    pub const MEMBER: i64 = 2;
    pub const VIEWER: i64 = 3;
    pub const BLOCKED: i64 = 4;
}

// --- ERROR CODE CONSTANTS ---

pub mod errors {
    // User management
    pub const LIST_USERS: u8 = 0;
    pub const CREATE_USER: u8 = 1;
    pub const UPDATE_USER: u8 = 2;
    pub const DELETE_USER: u8 = 3;
    pub const LIST_USER_TYPES: u8 = 4;
    pub const GET_USER: u8 = 5;
    pub const UPDATE_PROFILE: u8 = 6;
    pub const UPDATE_STATUS: u8 = 7;
    pub const CHANGE_PASSWORD: u8 = 8;

    // Direct messages
    pub const SEND_DIRECT_MESSAGE: u8 = 10;
    pub const UPDATE_DIRECT_MESSAGE: u8 = 11;
    pub const DELETE_DIRECT_MESSAGE: u8 = 12;
    pub const READ_DIRECT_MESSAGES: u8 = 13;

    // Groups
    pub const CREATE_GROUP: u8 = 20;
    pub const UPDATE_GROUP: u8 = 21;
    pub const DELETE_GROUP: u8 = 22;
    pub const READ_GROUP: u8 = 23;
    pub const SEND_GROUP_MESSAGE: u8 = 24;
    pub const UPDATE_GROUP_MESSAGE: u8 = 25;
    pub const DELETE_GROUP_MESSAGE: u8 = 26;
    pub const ADD_GROUP_MEMBER: u8 = 27;
    pub const UPDATE_GROUP_MEMBER: u8 = 28;
    pub const REMOVE_GROUP_MEMBER: u8 = 29;

    // Relationships
    pub const SEND_FRIEND_REQUEST: u8 = 30;
    pub const UPDATE_RELATIONSHIP: u8 = 31;
    pub const DELETE_RELATIONSHIP: u8 = 32;
    pub const LIST_RELATIONSHIPS: u8 = 33;
    pub const LIST_RELATIONSHIPS_ADMIN: u8 = 34;

    // Message ownership
    pub const UPDATE_MESSAGE_NOT_OWNER: u8 = 40;
    pub const DELETE_MESSAGE_NOT_OWNER: u8 = 41;

    // Fallback
    pub const DEFAULT: u8 = 255;
}

// --- ERROR MESSAGES ---

pub fn permission_error_message(err_code: u8) -> &'static str {
    match err_code {
        // User management
        errors::LIST_USERS => "You lack permissions to list users",
        errors::CREATE_USER => "You lack permissions to create users",
        errors::UPDATE_USER => "You lack permissions to update users",
        errors::DELETE_USER => "You lack permissions to delete users",
        errors::LIST_USER_TYPES => "You lack permissions to list user types",
        errors::GET_USER => "You lack permissions to view this user",
        errors::UPDATE_PROFILE => "You lack permissions to update your profile",
        errors::UPDATE_STATUS => "You lack permissions to update your status",
        errors::CHANGE_PASSWORD => "You lack permissions to change your password",

        // Direct messages
        errors::SEND_DIRECT_MESSAGE => "You lack permissions to send direct messages",
        errors::UPDATE_DIRECT_MESSAGE => "You lack permissions to update this message",
        errors::DELETE_DIRECT_MESSAGE => "You lack permissions to delete this message",
        errors::READ_DIRECT_MESSAGES => "You lack permissions to read direct messages",

        // Groups
        errors::CREATE_GROUP => "You lack permissions to create groups",
        errors::UPDATE_GROUP => "You lack permissions to update this group",
        errors::DELETE_GROUP => "You lack permissions to delete this group",
        errors::READ_GROUP => "You lack permissions to view this group",
        errors::SEND_GROUP_MESSAGE => "You lack permissions to send messages in this group",
        errors::UPDATE_GROUP_MESSAGE => "You lack permissions to update messages in this group",
        errors::DELETE_GROUP_MESSAGE => "You lack permissions to delete messages in this group",
        errors::ADD_GROUP_MEMBER => "You lack permissions to add members to this group",
        errors::UPDATE_GROUP_MEMBER => "You lack permissions to update members in this group",
        errors::REMOVE_GROUP_MEMBER => "You lack permissions to remove members from this group",

        // Relationships
        errors::SEND_FRIEND_REQUEST => "You lack permissions to send friend requests",
        errors::UPDATE_RELATIONSHIP => "You lack permissions to update this relationship",
        errors::DELETE_RELATIONSHIP => "You lack permissions to delete this relationship",
        errors::LIST_RELATIONSHIPS => "You lack permissions to see a list of your relationships",
        errors::LIST_RELATIONSHIPS_ADMIN => {
            "You lack permissions to see a list of this user's relationships"
        }

        // Message ownership
        errors::UPDATE_MESSAGE_NOT_OWNER => "You cannot edit a message you did not send",
        errors::DELETE_MESSAGE_NOT_OWNER => "You cannot delete a message you did not send",

        _ => "Insufficient permissions",
    }
}

// --- HELPER ---
// Extracts and verifies JWT from Authorization header
// Returns fresh JwtClaims with DB values overlaid

async fn extract_and_verify(req: &HttpRequest) -> Result<JwtClaims, actix_web::Error> {
    // Get Authorization header
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h,
        None => {
            return Err(ErrorUnauthorized("No Authorization header"));
        }
    };

    // Parse header value
    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return Err(ErrorUnauthorized("Invalid Authorization header"));
        }
    };

    // Check Bearer prefix
    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized("Invalid token format, no Bearer"));
    }

    // Extract and verify token signature and expiry
    let token = &auth_str["Bearer ".len()..];

    let claims = match verify_access_token(token) {
        Ok(c) => c,
        Err(_) => {
            return Err(ErrorUnauthorized("Invalid or expired token"));
        }
    };

    // Get DB pool from app state
    let data = match req.app_data::<actix_web::web::Data<crate::state::AppState>>() {
        Some(d) => d,
        None => {
            return Err(ErrorInternalServerError("Missing app state"));
        }
    };

    // Query fresh user data — catches permission changes since token was issued
    let user = match sqlx::query!(
        "SELECT user_type_id, account_status_id FROM users WHERE id = $1",
        claims.user_id
    )
    .fetch_optional(&data.db)
    .await
    {
        Ok(u) => u,
        Err(_) => {
            return Err(ErrorInternalServerError("DB error"));
        }
    };

    let user = match user {
        Some(u) => u,
        None => {
            return Err(ErrorUnauthorized("User not found"));
        }
    };

    // Rebuild claims with fresh DB values overlaid on top of JWT values
    let fresh_claims = JwtClaims {
        sub: claims.sub,
        user_id: claims.user_id,
        user_type_id: user.user_type_id,
        account_status_id: user.account_status_id,
        exp: claims.exp,
    };

    Ok(fresh_claims)
}

// --- GLOBAL PERMISSION EXTRACTOR ---
// Used for all non-group routes
// RequireGlobal<{ global::ADMIN }, { errors::LIST_USERS }>

pub struct RequireGlobal<const LEVEL: i64, const ERR: u8>(pub JwtClaims);

impl<const LEVEL: i64, const ERR: u8> FromRequest for RequireGlobal<LEVEL, ERR> {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            // Verify token and get fresh claims from DB
            let claims = match extract_and_verify(&req).await {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            // Reject blocked users
            if claims.user_type_id == global::BLOCKED {
                return Err(ErrorForbidden("Account is blocked"));
            }

            // Reject suspended or closed accounts
            if claims.account_status_id != 1 {
                return Err(ErrorForbidden("Account is not active"));
            }

            // Check permission level
            if claims.user_type_id <= LEVEL {
                Ok(RequireGlobal(claims))
            } else {
                Err(ErrorForbidden(permission_error_message(ERR)))
            }
        })
    }
}

// --- GROUP PERMISSION EXTRACTOR ---
// Used for chat group routes only
// RequireGroup<{ global::STANDARD_USER }, { group::MEMBER }, { errors::SEND_GROUP_MESSAGE }>

pub struct RequireGroup<const GLOBAL_LEVEL: i64, const GROUP_LEVEL: i64, const ERR: u8> {
    pub claims: JwtClaims,
    pub group_permission: i64,
}

impl<const GLOBAL_LEVEL: i64, const GROUP_LEVEL: i64, const ERR: u8> FromRequest
    for RequireGroup<GLOBAL_LEVEL, GROUP_LEVEL, ERR>
{
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            // Verify token and get fresh claims from DB
            let claims = match extract_and_verify(&req).await {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            // Reject blocked users
            if claims.user_type_id == global::BLOCKED {
                return Err(ErrorForbidden("Account is blocked"));
            }

            // Reject suspended or closed accounts
            if claims.account_status_id != 1 {
                return Err(ErrorForbidden("Account is not active"));
            }

            // Check global level requirement for this route
            if claims.user_type_id > GLOBAL_LEVEL {
                return Err(ErrorForbidden(permission_error_message(ERR)));
            }

            // Admins and super admins bypass group check entirely
            if claims.user_type_id <= global::ADMIN {
                return Ok(RequireGroup {
                    claims,
                    group_permission: group::MODERATOR,
                });
            }

            // Get DB pool from app state
            let data = match req.app_data::<actix_web::web::Data<crate::state::AppState>>() {
                Some(d) => d,
                None => {
                    return Err(ErrorInternalServerError("Missing app state"));
                }
            };

            // Get group_id from request body extension
            // Set by the route handler before extractor runs
            let group_id = match req.match_info().get("group_id") {
                Some(id) => match id.parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => {
                        return Err(ErrorUnauthorized("Invalid group_id in path"));
                    }
                },
                None => {
                    return Err(ErrorUnauthorized("Missing group_id in path"));
                }
            };

            // Query fresh group permission from DB
            let group_perm = match sqlx::query_scalar!(
                "SELECT permission_type_id FROM chat_group_permissions
                 WHERE group_id = $1 AND user_id = $2",
                group_id,
                claims.user_id
            )
            .fetch_optional(&data.db)
            .await
            {
                Ok(p) => p,
                Err(_) => {
                    return Err(ErrorInternalServerError("DB error"));
                }
            };

            let group_permission = match group_perm {
                Some(p) => p,
                None => {
                    return Err(ErrorForbidden("Not a member of this group"));
                }
            };

            // Reject blocked group members
            if group_permission == group::BLOCKED {
                return Err(ErrorForbidden("You are blocked in this group"));
            }

            // Check group permission level
            if group_permission <= GROUP_LEVEL {
                Ok(RequireGroup {
                    claims,
                    group_permission,
                })
            } else {
                Err(ErrorForbidden(permission_error_message(ERR)))
            }
        })
    }
}

// --- MESSAGE OWNERSHIP HELPERS ---

pub fn check_can_update_message(sender_id: i64, user_id: i64) -> bool {
    sender_id == user_id
}

pub fn check_can_delete_message(
    sender_id: i64,
    user_id: i64,
    user_type_id: i64,
    group_permission: Option<i64>,
) -> bool {
    let is_sender = sender_id == user_id;

    let is_global_admin = user_type_id <= global::ADMIN;

    let is_group_moderator = match group_permission {
        Some(p) => p <= group::MODERATOR,
        None => false,
    };

    is_sender || is_global_admin || is_group_moderator
}
