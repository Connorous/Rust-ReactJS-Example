use crate::auth;
use actix_web::{middleware::from_fn, web};
mod chat_group_routes;
mod direct_message_routes;
mod login_routes;
mod relationship_routes;
pub(crate) mod user_routes;
mod ws_routes;

// --- LOGIN ROUTES ---
// Protected by app_auth — checks app secret, no JWT needed

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
// No middleware — RequireGlobal extractor handles auth and permissions

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/list", web::get().to(user_routes::list_users))
            .route("/search", web::post().to(user_routes::search_users))
            .route("/user-types", web::get().to(user_routes::list_user_types))
            .route(
                "/user-status-types",
                web::get().to(user_routes::list_user_status_types),
            )
            .route(
                "/user-account-status-types",
                web::get().to(user_routes::list_account_status_types),
            )
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
// No middleware — RequireGlobal extractor handles auth and permissions

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
                "/list-status-types",
                web::get().to(relationship_routes::list_relationship_status_types),
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
// No middleware — RequireGlobal extractor handles auth and permissions

pub fn configure_direct_message_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/direct-messages")
            .route(
                "/list",
                web::post().to(direct_message_routes::list_messages),
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
// No middleware — RequireGroup extractor handles auth and permissions

pub fn configure_chat_group_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
            // Group management
            .route("/list", web::get().to(chat_group_routes::list_groups))
            .route("/search", web::post().to(chat_group_routes::search_groups))
            .route("/group/get", web::post().to(chat_group_routes::get_group))
            .route("/group/new", web::post().to(chat_group_routes::new_group))
            .route("/group", web::put().to(chat_group_routes::update_group))
            .route("/group", web::delete().to(chat_group_routes::delete_group))
            .route(
                "/list-users-groups",
                web::post().to(chat_group_routes::list_user_groups),
            )
            .route(
                "/search-users-groups",
                web::post().to(chat_group_routes::search_user_groups),
            )
            .route(
                "/list-group-members",
                web::post().to(chat_group_routes::list_group_members),
            )
            .route(
                "/list-non-group-members",
                web::post().to(chat_group_routes::list_non_group_members),
            )
            .route(
                "/list-users-sent_group_messages",
                web::post().to(chat_group_routes::list_users_who_sent_group_messages),
            )
            // Group messages
            .route(
                "/messages",
                web::post().to(chat_group_routes::list_messages),
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
                "/permission-types",
                web::post().to(chat_group_routes::list_group_permission_types),
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
// Still needs auth middleware since WS connection upgrade
// can't use extractors the same way

pub fn configure_ws_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ws")
            .wrap(from_fn(auth::auth))
            .route("", web::get().to(ws_routes::ws_handler)),
    );
}
