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
    pub const BLOCK_RELATIONSHIP: u8 = 32;
    pub const DELETE_RELATIONSHIP: u8 = 33;
    pub const LIST_RELATIONSHIPS: u8 = 34;
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
        errors::LIST_USERS => "You Do Not Have Permissions to View List of Users",
        errors::CREATE_USER => "You Do Not Have Permissions to Create Users",
        errors::UPDATE_USER => "You Do Not Have Permissions to Update Users",
        errors::DELETE_USER => "You Do Not Have Permissions to Delete Users",
        errors::LIST_USER_TYPES => "You Do Not Have Permissions to Get User Types",
        errors::GET_USER => "You Do Not Have Permissions to View this user",
        errors::UPDATE_PROFILE => "You Do Not Have Permissions to Update Your Profile",
        errors::UPDATE_STATUS => "You Do Not Have Permissions to Update Your Status",
        errors::CHANGE_PASSWORD => "You Do Not Have Permissions to Change Your Password",

        // Direct messages
        errors::SEND_DIRECT_MESSAGE => "You Do Not Have Permissions to Send direct Messages",
        errors::UPDATE_DIRECT_MESSAGE => "You Do Not Have Permissions to Update this Message",
        errors::DELETE_DIRECT_MESSAGE => "You Do Not Have Permissions to Delete this Message",
        errors::READ_DIRECT_MESSAGES => "You Do Not Have Permissions to Read Direct Messages",

        // Groups
        errors::CREATE_GROUP => "You Do Not Have Permissions to Create Chat Groups",
        errors::UPDATE_GROUP => "You Do Not Have Permissions to Update this Chat Group",
        errors::DELETE_GROUP => "You Do Not Have Permissions to Delete this Chat Group",
        errors::READ_GROUP => "You Do Not Have Permissions to View this Chat Group",
        errors::SEND_GROUP_MESSAGE => {
            "You Do Not Have Permissions to Send Messages in this Chat Group"
        }
        errors::UPDATE_GROUP_MESSAGE => {
            "You Do Not Have Permissions to Update Your Messages in this Chat Group"
        }
        errors::DELETE_GROUP_MESSAGE => {
            "You Do Not Have Permissions to Delete Messages in this Chat Group"
        }
        errors::ADD_GROUP_MEMBER => {
            "You Do Not Have Permissions to Add New Members to this Chat Group"
        }
        errors::UPDATE_GROUP_MEMBER => {
            "You Do Not Have Permissions to Update Members in this Chat Group"
        }
        errors::REMOVE_GROUP_MEMBER => {
            "You Do Not Have Permissions to Remove Members From this Chat Group"
        }

        // Relationships
        errors::SEND_FRIEND_REQUEST => "You Do Not Have Permissions to Send Friend Requests",
        errors::UPDATE_RELATIONSHIP => {
            "You Do Not Have Permissions to Accept or Decline Friend Requests"
        }
        errors::BLOCK_RELATIONSHIP => "You Do Not Have Permissions to Block Friend Requests",
        errors::DELETE_RELATIONSHIP => "You Do Not Have Permissions to Delete this Relationship",
        errors::LIST_RELATIONSHIPS => {
            "You Do Not Have Permissions to see a List of Your Relationships with other Users"
        }
        errors::LIST_RELATIONSHIPS_ADMIN => {
            "You Do Not Have Permissions to see a List of this User's Relationships with other Users"
        }

        _ => "Insufficient Permissions",
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
// RequireGlobal<{ user_type::ADMIN }, { errors::LIST_USERS }>

pub struct RequireUserType<const LEVEL: i64, const ERR: u8>(pub JwtClaims);

impl<const USER_TYPE_LEVEL: i64, const ERR: u8> FromRequest
    for RequireUserType<USER_TYPE_LEVEL, ERR>
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
            if claims.user_type_id == user_type::BLOCKED {
                return Err(ErrorForbidden("Account is Blocked"));
            }

            // Reject suspended or closed accounts
            if claims.account_status_id != 1 {
                return Err(ErrorForbidden("Account is Not Active"));
            }

            // Check permission level
            if claims.user_type_id <= USER_TYPE_LEVEL {
                Ok(RequireUserType(claims))
            } else {
                Err(ErrorForbidden(permission_error_message(ERR)))
            }
        })
    }
}

// --- GROUP PERMISSION EXTRACTOR ---
// Used for chat group routes only
// RequireGroup<{ user_type::STANDARD_USER }, { group_permission::MEMBER }, { errors::SEND_GROUP_MESSAGE }>

pub struct RequireGroup<
    const USER_TYPE_LEVEL: i64,
    const GROUP_PERMISSION_LEVEL: i64,
    const ERR: u8,
> {
    pub claims: JwtClaims,
    pub group_permission: i64,
}

impl<const USER_TYPE_LEVEL: i64, const GROUP_PERMISSION_LEVEL: i64, const ERR: u8> FromRequest
    for RequireGroup<USER_TYPE_LEVEL, GROUP_PERMISSION_LEVEL, ERR>
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
            if claims.user_type_id == user_type::BLOCKED {
                return Err(ErrorForbidden("Account is blocked"));
            }

            // Reject suspended or closed accounts
            if claims.account_status_id != 1 {
                return Err(ErrorForbidden("Account is Not Active"));
            }

            // Check user type level requirement for this route
            if claims.user_type_id > USER_TYPE_LEVEL {
                return Err(ErrorForbidden(permission_error_message(ERR)));
            }

            // Admins and super admins bypass group check entirely
            if claims.user_type_id <= user_type::ADMIN {
                return Ok(RequireGroup {
                    claims,
                    group_permission: group_permission::MODERATOR,
                });
            }

            // Get DB pool from app state ? why use app state differently unlike other functions?
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
            if group_permission == group_permission::BLOCKED {
                return Err(ErrorForbidden("You are Blocked From this Group"));
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
