use crate::controllers::login_controller;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

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

#[derive(Deserialize, Clone)]
pub struct ResetPasswordRequestBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    data: web::Data<AppState>,
    json: web::Json<RegisterUserRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match login_controller::register_user(
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
        match login_controller::login_user(data, body.username, body.password).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn refresh_token(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let result: HttpResponse = match login_controller::refresh_token(data, req).await {
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
        match login_controller::reset_user_password(data, body.username, body.email, body.password)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}
