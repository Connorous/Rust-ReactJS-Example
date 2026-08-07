use crate::controllers::user_controller;
use crate::extractors::{errors, global, RequireGlobal};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

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
pub struct GetUserRequestBody {
    pub id: i64,
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
pub struct DeleteUserRequestBody {
    pub id: i64,
}

pub async fn list_users(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }, { errors::LIST_USERS }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::list_users(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn list_user_types(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }, { errors::LIST_USER_TYPES }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::list_user_types(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn get_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }, { errors::GET_USER }>,
    json: web::Json<GetUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::get_user(data, claims.0, body.id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn new_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }, { errors::CREATE_USER }>,
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
    claims: RequireGlobal<{ global::ADMIN }, { errors::UPDATE_USER }>,
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
    claims: RequireGlobal<{ global::VIEWER }, { errors::UPDATE_PROFILE }>,
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
    claims: RequireGlobal<{ global::VIEWER }, { errors::UPDATE_STATUS }>,
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

pub async fn logout_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }, { errors::DEFAULT }>,
) -> HttpResponse {
    let result: HttpResponse = match user_controller::logout_user(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn delete_user(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::ADMIN }, { errors::DELETE_USER }>,
    json: web::Json<DeleteUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match user_controller::delete_user(data, claims.0, body.id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}
