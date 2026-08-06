use crate::auth::JwtClaims;
use crate::controllers::user_controller;
use crate::extractors::RequireGlobal;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

// --- REQUEST BODIES ---

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterUserRequestBody {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUserRequestBody {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewUserRequestBody {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub user_type_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateUserRequestBody {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub name: String,
    pub user_type_id: i64,
    pub account_status_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateProfileRequestBody {
    pub name: String,
    pub bio_info: Option<String>,
    pub theme_id: Option<i64>,
    pub theme_dark_mode: bool,
    pub light_theme_primary_colour: String,
    pub light_theme_secondary_colour: String,
    pub light_theme_accent_colour: String,
    pub light_theme_sent_colour: String,
    pub light_theme_received_colour: String,
    pub light_theme_dark_text_colour: String,
    pub light_theme_light_text_colour: String,
    pub dark_theme_primary_colour: String,
    pub dark_theme_secondary_colour: String,
    pub dark_theme_accent_colour: String,
    pub dark_theme_sent_colour: String,
    pub dark_theme_received_colour: String,
    pub dark_theme_dark_text_colour: String,
    pub dark_theme_light_text_colour: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateStatusRequestBody {
    pub status_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct ResetPasswordRequestBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct ChangePasswordRequestBody {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Clone)]
pub struct DeleteUserRequestBody {
    pub id: i64,
}

// --- LOGIN ROUTES ---

pub async fn register_user(
    data: web::Data<AppState>,
    json: web::Json<RegisterUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::register_user(
        data,
        body.username,
        body.email,
        body.name,
        body.password,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn login_user(
    data: web::Data<AppState>,
    json: web::Json<LoginUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match user_controller::login_user(data, body.username, body.password).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn refresh_token(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let result: HttpResponse = match user_controller::refresh_token(data, req).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn logout_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::logout_user(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn reset_user_password(
    data: web::Data<AppState>,
    json: web::Json<ResetPasswordRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match user_controller::reset_user_password(data, body.username, body.email, body.password)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

// --- USER ROUTES ---

pub async fn list_users(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::list_users(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn list_user_types(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::list_user_types(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn list_themes(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::list_themes(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn get_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }>,
    id: web::Path<i64>,
) -> HttpResponse {
    let user_id: i64 = id.into_inner();

    let result: HttpResponse = match user_controller::get_user(data, claims.0, user_id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn new_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }>,
    json: web::Json<NewUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::new_user(
        data,
        claims.0,
        body.username,
        body.email,
        body.name,
        body.password,
        body.user_type_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn update_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }>,
    json: web::Json<UpdateUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::update_user(
        data,
        claims.0,
        body.id,
        body.username,
        body.email,
        body.name,
        body.user_type_id,
        body.account_status_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn update_profile(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }>,
    json: web::Json<UpdateProfileRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::update_profile(data, claims.0, body).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn update_status(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }>,
    json: web::Json<UpdateStatusRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match user_controller::update_status(data, claims.0, body.status_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn change_password(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }>,
    json: web::Json<ChangePasswordRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::change_password(
        data,
        claims.0,
        body.current_password,
        body.new_password,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn delete_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }>,
    json: web::Json<DeleteUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::delete_user(data, claims.0, body.id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}
