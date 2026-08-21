use crate::auth::{verify_access_token, JwtClaims};
use actix_web::HttpResponse;
use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized},
    FromRequest, HttpRequest,
};
use serde::Serialize;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

// App Error Handling

#[derive(Debug, Serialize)]
struct ErrorResponse {
    msg: String,
    success: bool,
}

#[derive(Debug)]
pub struct AppError {
    pub status: u16,
    pub msg: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            msg: self.msg.clone(),
            success: false,
        };

        match self.status {
            400 => HttpResponse::BadRequest().json(response),
            401 => HttpResponse::Unauthorized().json(response),
            403 => HttpResponse::Forbidden().json(response),
            500 => HttpResponse::InternalServerError().json(response),
            _ => HttpResponse::InternalServerError().json(response),
        }
    }
}

fn bad_request(msg: &str) -> actix_web::Error {
    AppError {
        status: 400,
        msg: msg.to_string(),
    }
    .into()
}

fn unauthorized(msg: &str) -> actix_web::Error {
    AppError {
        status: 401,
        msg: msg.to_string(),
    }
    .into()
}

fn forbidden(msg: &str) -> actix_web::Error {
    AppError {
        status: 403,
        msg: msg.to_string(),
    }
    .into()
}

fn internal_error(msg: &str) -> actix_web::Error {
    AppError {
        status: 500,
        msg: msg.to_string(),
    }
    .into()
}

// --- GLOBAL PERMISSION CONSTANTS ---

pub mod user_type {
    pub const SUPER_ADMIN: i64 = 1;
    pub const ADMIN: i64 = 2;
    pub const STANDARD_USER: i64 = 3;
    pub const VIEWER: i64 = 4;
    pub const BLOCKED: i64 = 5;
}

pub mod group_permission {
    pub const OWNER: i64 = 1;
    pub const MODERATOR: i64 = 2;
    pub const MEMBER: i64 = 3;
    pub const VIEWER: i64 = 4;
    pub const BLOCKED: i64 = 5;
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
    pub const LIST_GROUPS: u8 = 24;
    pub const LIST_GROUPS_ADMIN: u8 = 25;
    pub const SEND_GROUP_MESSAGE: u8 = 26;
    pub const UPDATE_GROUP_MESSAGE: u8 = 27;
    pub const DELETE_GROUP_MESSAGE: u8 = 28;
    pub const ADD_GROUP_MEMBER: u8 = 29;
    pub const UPDATE_GROUP_MEMBER: u8 = 30;
    pub const REMOVE_GROUP_MEMBER: u8 = 31;

    // Relationships
    pub const SEND_FRIEND_REQUEST: u8 = 40;
    pub const UPDATE_RELATIONSHIP: u8 = 41;
    pub const BLOCK_RELATIONSHIP: u8 = 42;
    pub const DELETE_RELATIONSHIP: u8 = 43;
    pub const LIST_RELATIONSHIPS: u8 = 44;
    pub const LIST_RELATIONSHIPS_ADMIN: u8 = 44;

    // Message ownership
    pub const UPDATE_MESSAGE_NOT_OWNER: u8 = 50;
    pub const DELETE_MESSAGE_NOT_OWNER: u8 = 51;

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
        errors::LIST_GROUPS => "You Do Not Have Permissions to View Chat Groups",
        errors::LIST_GROUPS_ADMIN => "You Do Not Have Permissions to View Other Users' Chat Groups",
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

async fn extract_and_verify(req: &HttpRequest) -> Result<JwtClaims, actix_web::Error> {
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h,
        None => {
            return Err(unauthorized("No Authorization header"));
        }
    };

    let auth_str = match auth_header.to_str() {
        Ok(s) => s,
        Err(_) => {
            return Err(unauthorized("Invalid Authorization header"));
        }
    };

    if (!auth_str.starts_with("Bearer ")) {
        return Err(unauthorized("Invalid token format, no Bearer"));
    }

    let token = &auth_str["Bearer ".len()..];

    let claims = match verify_access_token(token) {
        Ok(c) => c,
        Err(_) => {
            return Err(unauthorized("Invalid or expired token"));
        }
    };

    let data = match req.app_data::<actix_web::web::Data<crate::state::AppState>>() {
        Some(d) => d,
        None => {
            return Err(internal_error("Missing app state"));
        }
    };

    let pool = data.db.to_owned();

    let user = match sqlx::query!(
        "SELECT user_type_id, account_status_id FROM users WHERE id = $1",
        claims.user_id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(u) => u,
        Err(_) => {
            return Err(internal_error("DB error"));
        }
    };

    let user = match user {
        Some(existing_user) => existing_user,
        None => {
            return Err(unauthorized("User not found"));
        }
    };

    let fresh_claims = JwtClaims {
        sub: claims.sub,
        user_id: claims.user_id,
        user_type_id: claims.user_type_id,
        account_status_id: claims.account_status_id,
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
            if (claims.user_type_id == user_type::BLOCKED) {
                return Err(AppError {
                    status: 403,
                    msg: String::from("Account is Blocked"),
                }
                .into());
            }

            // Reject suspended or closed accounts
            if (claims.account_status_id != 1) {
                return Err(AppError {
                    status: 403,
                    msg: String::from("Account is Not Active"),
                }
                .into());
            }

            // Check user type level requirement for this route
            if (claims.user_type_id <= USER_TYPE_LEVEL) {
                return Err(AppError {
                    status: 403,
                    msg: String::from(permission_error_message(ERR)),
                }
                .into());
            }

            // Check permission level
            if claims.user_type_id <= USER_TYPE_LEVEL {
                Ok(RequireUserType(claims))
            } else {
                return Err(AppError {
                    status: 403,
                    msg: String::from(permission_error_message(ERR)),
                }
                .into());
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
>(pub JwtClaims);

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
            if (claims.user_type_id == user_type::BLOCKED) {
                return Err(AppError {
                    status: 403,
                    msg: String::from("Account is Blocked"),
                }
                .into());
            }

            // Reject suspended or closed accounts
            if (claims.account_status_id != 1) {
                return Err(AppError {
                    status: 403,
                    msg: String::from("Account is Not Active"),
                }
                .into());
            }

            // Check user type level requirement for this route
            if (claims.user_type_id <= USER_TYPE_LEVEL) {
                return Err(AppError {
                    status: 403,
                    msg: String::from(permission_error_message(ERR)),
                }
                .into());
            }

            // Admins and super admins bypass group check entirely
            if claims.user_type_id <= user_type::ADMIN {
                return Ok(RequireGroup(claims));
            }

            // Get DB pool from app state ? why use app state differently unlike other functions?
            let data = match req.app_data::<actix_web::web::Data<crate::state::AppState>>() {
                Some(appstate) => appstate,
                None => {
                    return Err(AppError {
                        status: 500,
                        msg: String::from("Missing App State"),
                    }
                    .into());
                }
            };

            let pool = data.db.to_owned();

            // Get group_id from request body extension
            // Set by the route handler before extractor runs
            let group_id = match req.headers().get("Group-Id") {
                Some(val) => match val.to_str() {
                    Ok(id) => match id.parse::<i64>() {
                        Ok(id) => Some(id),
                        Err(_) => {
                            return Err(AppError {
                                status: 401,
                                msg: String::from("Invalid group_id in header"),
                            }
                            .into());
                        }
                    },
                    Err(_) => {
                        return Err(AppError {
                            status: 401,
                            msg: String::from("Missing group_id in header"),
                        }
                        .into());
                    }
                },
                None => {
                    return Err(AppError {
                        status: 401,
                        msg: String::from("Missing group_id in header"),
                    }
                    .into());
                }
            };

            // Query to see if group exists
            let group = match sqlx::query_scalar!(
                "SELECT id FROM chat_groups
                 WHERE id = $1",
                group_id,
            )
            .fetch_optional(&data.db)
            .await
            {
                Ok(existing_group) => existing_group,
                Err(_) => {
                    return Err(AppError {
                        status: 500,
                        msg: String::from("DB Error"),
                    }
                    .into());
                }
            };

            match group {
                None => {
                    return Err(AppError {
                        status: 400,
                        msg: String::from("Group Could Not be Found"),
                    }
                    .into());
                }
                Some(existing_group) => {
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
                        Ok(permissions) => permissions,
                        Err(_) => {
                            return Err(AppError {
                                status: 500,
                                msg: String::from("DB Error"),
                            }
                            .into());
                        }
                    };

                    let group_permission = match group_perm {
                        Some(permission) => permission,
                        None => {
                            return Err(AppError {
                        status: 403,
                        msg: String::from(
                            "Group Permissions Could Not be Found for Your User for this Group",
                        ),
                    }
                    .into());
                        }
                    };

                    // Reject blocked group members
                    if group_permission == group_permission::BLOCKED {
                        return Err(AppError {
                            status: 403,
                            msg: String::from("You are Blocked From this Group"),
                        }
                        .into());
                    }

                    // Check group permission level
                    if group_permission <= GROUP_PERMISSION_LEVEL {
                        Ok(RequireGroup(claims))
                    } else {
                        return Err(AppError {
                            status: 403,
                            msg: String::from(permission_error_message(ERR)),
                        }
                        .into());
                    }
                }
            }
        })
    }
}
