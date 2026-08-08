use crate::auth;
use actix_web::{middleware::from_fn, web};
mod chat_group_routes;
mod direct_message_routes;
mod login_routes;
mod relationship_routes;
mod relationship_routes;
mod user_routes;
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
            .route("/user-types", web::get().to(user_routes::list_user_types))
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
                web::get().to(direct_message_routes::list_conversations),
            )
            .route(
                "/messages",
                web::post().to(direct_message_routes::list_messages),
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
            .route("/group/get", web::post().to(chat_group_routes::get_group))
            .route("/group/new", web::post().to(chat_group_routes::new_group))
            .route("/group", web::put().to(chat_group_routes::update_group))
            .route("/group", web::delete().to(chat_group_routes::delete_group))
            // Group messages
            .route(
                "/messages",
                web::post().to(chat_group_routes::list_messages),
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
// Still needs auth middleware since WS connection upgrade
// can't use extractors the same way

pub fn configure_ws_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ws")
            .wrap(from_fn(auth::auth))
            .route("", web::get().to(ws_routes::ws_handler)),
    );
}
