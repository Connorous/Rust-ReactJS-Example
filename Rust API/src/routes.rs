use crate::auth;
use actix_web::{middleware::from_fn, web};
mod page_css_routes;
mod page_elements_routes;
mod page_permissions_routes;
mod page_routes;
mod user_routes;

pub fn configure_login_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .wrap(from_fn(auth::app_auth))
            .route("/register-user", web::post().to(user_routes::register_user))
            .route("/login-user", web::post().to(user_routes::login_user))
            .route(
                "/reset-user-password",
                web::post().to(user_routes::reset_user_password),
            ),
    );
}

/*pub fn configure_auth_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .wrap(from_fn(auth::auth))
            .route("/try-auth", web::post().to(auth::try_auth)),
    );
}*/

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(from_fn(auth::auth))
            .route("/list/{i}", web::get().to(user_routes::list_users))
            .route(
                "/user-types/{i}",
                web::get().to(user_routes::list_user_types),
            )
            //.route("/user/{i}", web::get().to(user_routes::get_user))
            .route("/user", web::post().to(user_routes::new_user))
            .route("/user", web::put().to(user_routes::update_user))
            .route("/user", web::delete().to(user_routes::delete_user)),
    );
}

pub fn configure_page_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pages")
            .wrap(from_fn(auth::auth))
            .route(
                "/list-pages-usermade",
                web::post().to(page_routes::list_pages_usermade),
            )
            .route("/list/{i}", web::get().to(page_routes::list_all_pages))
            .route(
                "/list-creators/{i}",
                web::get().to(page_routes::list_all_page_creators),
            )
            .route("/page/get", web::post().to(page_routes::get_page))
            .route("/page/post", web::post().to(page_routes::new_page))
            .route("/page", web::put().to(page_routes::update_page))
            .route("/page", web::delete().to(page_routes::delete_page)),
    );
}

pub fn configure_page_element_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/page-elements")
            .wrap(from_fn(auth::auth))
            .route(
                "/list-elements",
                web::post().to(page_elements_routes::list_page_elements),
            )
            .route(
                "/page-elements-types",
                web::post().to(page_elements_routes::list_page_element_types),
            )
            .route(
                "/new-elements",
                web::post().to(page_elements_routes::new_page_elements),
            )
            .route(
                "/update-elements",
                web::put().to(page_elements_routes::update_page_elements),
            )
            .route(
                "/delete-elements",
                web::delete().to(page_elements_routes::delete_page_element),
            ),
    );
}

pub fn configure_page_permission_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/page-permissions")
            .wrap(from_fn(auth::auth))
            .route(
                "/all-page-user-permissions",
                web::post().to(page_permissions_routes::list_user_pages_permissions),
            )
            .route(
                "/all-page-users-with-permissions",
                web::post().to(page_permissions_routes::list_users_with_page_permissions),
            )
            .route(
                "/all-page-users-without-permissions",
                web::post().to(page_permissions_routes::list_users_without_page_permissions),
            )
            .route(
                "/page-permission-types",
                web::post().to(page_permissions_routes::list_page_permission_types),
            )
            .route(
                "/user-page-permissions",
                web::post().to(page_permissions_routes::get_user_page_permission),
            )
            .route(
                "/page-permission",
                web::post().to(page_permissions_routes::new_user_page_permission),
            )
            .route(
                "/page-permission",
                web::put().to(page_permissions_routes::update_user_page_permission),
            )
            .route(
                "/page-permission",
                web::delete().to(page_permissions_routes::delete_page_permission),
            ),
    );
}

pub fn configure_page_css_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/page-css")
            .wrap(from_fn(auth::auth))
            .route("/list-css", web::post().to(page_css_routes::list_page_css))
            .route("/css/{i}", web::get().to(page_css_routes::get_page_css))
            .route("/css", web::post().to(page_css_routes::new_page_css))
            .route("/css", web::put().to(page_css_routes::update_page_css))
            .route("/css", web::delete().to(page_css_routes::delete_page_css)),
    );
}
