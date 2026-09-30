use crate::auth;
use actix_web::{middleware::from_fn, web};
mod chat_group_routes;
mod direct_message_routes;
mod login_routes;
mod relationship_routes;
pub(crate) mod user_routes;
mod ws_routes;

pub fn configure_login_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .wrap(from_fn(auth::app_auth))
            .route("/register", web::post().to(login_routes::register_user))
            .route("/login", web::post().to(login_routes::login_user))
            .route("/refresh", web::post().to(login_routes::refresh_token))
            .route(
                "/reset-password",
                web::post().to(login_routes::reset_user_password),
            ),
    );
}

// --- USER ROUTES ---

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/list", web::post().to(user_routes::list_users))
            .route("/user/new", web::post().to(user_routes::new_user))
            .route("/user/get", web::post().to(user_routes::get_user))
            .route("/user", web::put().to(user_routes::update_user))
            .route("/user", web::delete().to(user_routes::delete_user))
            .route("/logout", web::post().to(user_routes::logout_user))
            .route("/profile", web::put().to(user_routes::update_profile))
            .route("/status", web::put().to(user_routes::update_status)),
    );
}

// --- RELATIONSHIP ROUTES ---

pub fn configure_relationship_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/relationships")
            .route(
                "/list",
                web::get().to(relationship_routes::list_relationships),
            )
            .route(
                "/search",
                web::post().to(relationship_routes::search_relationships),
            )
            .route(
                "/get/relationship",
                web::post().to(relationship_routes::get_relationship),
            )
            .route(
                "/relationship",
                web::post().to(relationship_routes::new_relationship),
            )
            .route(
                "/relationship",
                web::put().to(relationship_routes::update_relationship),
            )
            .route(
                "/relationship",
                web::delete().to(relationship_routes::delete_relationship),
            )
            .route(
                "/block-relationship",
                web::post().to(relationship_routes::block_relationship),
            )
            .route(
                "/list-users-relationships",
                web::post().to(relationship_routes::list_user_relationships),
            )
            .route(
                "/search-users-relationships",
                web::post().to(relationship_routes::search_user_relationships),
            )
            .route(
                "/list-users-relationship-users",
                web::post().to(relationship_routes::list_user_users_in_relationship_with),
            )
            .route(
                "/list-users-non-relationship-users",
                web::post().to(relationship_routes::list_user_users_not_in_relationship_with),
            ),
    );
}

// --- DIRECT MESSAGE ROUTES ---

pub fn configure_direct_message_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/direct-messages")
            .route(
                "/list",
                web::post().to(direct_message_routes::list_messages),
            )
            .route(
                "/list-around-message",
                web::post().to(direct_message_routes::get_messages_around),
            )
            .route(
                "/search",
                web::post().to(direct_message_routes::search_messages),
            )
            .route(
                "/message",
                web::post().to(direct_message_routes::send_message),
            )
            .route(
                "/message",
                web::put().to(direct_message_routes::update_message),
            )
            .route(
                "/message",
                web::delete().to(direct_message_routes::delete_message),
            ),
    );
}

// --- CHAT GROUP ROUTES ---

pub fn configure_chat_group_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
            // User's own groups
            .route("/list", web::get().to(chat_group_routes::list_groups))
            /* .route("/search", web::post().to(chat_group_routes::search_groups))*/
            // Public group discovery
            .route(
                "/search-public-groups",
                web::post().to(chat_group_routes::search_public_groups),
            )
            .route(
                "/get-public-group-details",
                web::post().to(chat_group_routes::get_public_group_details),
            )
            .route(
                "/request-to-join-group",
                web::post().to(chat_group_routes::group_request),
            )
            // Group management
            .route("/group/get", web::post().to(chat_group_routes::get_group))
            .route("/group/new", web::post().to(chat_group_routes::new_group))
            .route("/group", web::put().to(chat_group_routes::update_group))
            .route("/group", web::delete().to(chat_group_routes::delete_group))
            // Admin group routes
            .route(
                "/list-users-groups",
                web::post().to(chat_group_routes::list_user_groups),
            )
            .route(
                "/search-users-groups",
                web::post().to(chat_group_routes::search_user_groups),
            )
            // Group members
            .route(
                "/list-group-members",
                web::post().to(chat_group_routes::list_group_members),
            )
            .route(
                "/list-non-group-members",
                web::post().to(chat_group_routes::list_non_group_members),
            )
            .route(
                "/list-users-sent-group-messages",
                web::post().to(chat_group_routes::list_users_who_sent_group_messages),
            )
            // Group messages
            .route(
                "/messages",
                web::post().to(chat_group_routes::list_messages),
            )
            .route(
                "/list-messages-around",
                web::post().to(chat_group_routes::get_messages_around),
            )
            .route(
                "/search-messages",
                web::post().to(chat_group_routes::search_messages),
            )
            .route("/message", web::post().to(chat_group_routes::send_message))
            .route("/message", web::put().to(chat_group_routes::update_message))
            .route(
                "/message",
                web::delete().to(chat_group_routes::delete_message),
            )
            // Group permissions
            .route(
                "/permissions",
                web::post().to(chat_group_routes::list_group_permissions),
            )
            .route(
                "/permission/new",
                web::post().to(chat_group_routes::add_group_permission),
            )
            .route(
                "/permission",
                web::put().to(chat_group_routes::update_group_permission),
            )
            .route(
                "/permission",
                web::delete().to(chat_group_routes::delete_group_permission),
            ),
    );
}

// --- WEBSOCKET ROUTE ---

pub fn configure_ws_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ws")
            .wrap(from_fn(auth::auth))
            .route("", web::get().to(ws_routes::ws_handler)),
    );
}
