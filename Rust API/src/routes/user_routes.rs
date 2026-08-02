use crate::controllers::user_controller;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NewUserRequestBody {
    admin_id: i64,
    username: String,
    email: String,
    name: String,
    password: String,
    user_type_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateUserRequestBody {
    admin_id: i64,
    id: i64,
    username: String,
    email: String,
    name: String,
    user_type_id: i64,
    original_user_type: i64,
}

#[derive(Deserialize, Clone)]
pub struct ResetPasswordUserRequestBody {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterUserRequestBody {
    username: String,
    email: String,
    name: String,
    password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUserRequestBody {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
pub struct DeleteUserRequestBody {
    admin_id: i64,
    id: i64,
}

pub async fn register_user(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<RegisterUserRequestBody>,
) -> HttpResponse {
    let new_user_info = json.clone();

    let result: HttpResponse = match user_controller::register_user(
        data,
        new_user_info.username,
        new_user_info.email,
        new_user_info.name,
        new_user_info.password,
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
    req: HttpRequest,
    json: web::Json<LoginUserRequestBody>,
) -> HttpResponse {
    let new_user_info = json.clone();

    let result: HttpResponse =
        match user_controller::login_user(data, new_user_info.username, new_user_info.password)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };
    result
}

pub async fn list_users(data: web::Data<AppState>, id: web::Path<i64>) -> HttpResponse {
    let get_auth_user_id: i64 = id.into_inner();
    let result: HttpResponse = match user_controller::list_users(data, get_auth_user_id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn list_user_types(data: web::Data<AppState>, id: web::Path<i64>) -> HttpResponse {
    let get_auth_user_id: i64 = id.into_inner();
    let result: HttpResponse = match user_controller::list_user_types(data, get_auth_user_id).await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

/*pub async fn get_user(
    data: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i64>,
) -> HttpResponse {
    let get_user_id: i64 = id.into_inner();
    let result: HttpResponse = match user_controller::get_user(data, get_user_id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}*/

pub async fn new_user(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<NewUserRequestBody>,
) -> HttpResponse {
    let new_user_info = json.clone();

    let result: HttpResponse = match user_controller::new_user(
        data,
        new_user_info.admin_id,
        new_user_info.username,
        new_user_info.email,
        new_user_info.name,
        new_user_info.password,
        new_user_info.user_type_id,
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
    req: HttpRequest,
    json: web::Json<UpdateUserRequestBody>,
) -> HttpResponse {
    let update_user_info = json.clone();
    let result: HttpResponse = match user_controller::update_user(
        data,
        update_user_info.admin_id,
        update_user_info.id,
        update_user_info.username,
        update_user_info.email,
        update_user_info.name,
        update_user_info.user_type_id,
        update_user_info.original_user_type,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn reset_user_password(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<ResetPasswordUserRequestBody>,
) -> HttpResponse {
    let password_reset_user_info = json.clone();
    let result: HttpResponse = match user_controller::reset_user_password(
        data,
        password_reset_user_info.username,
        password_reset_user_info.email,
        password_reset_user_info.password,
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
    req: HttpRequest,
    json: web::Json<DeleteUserRequestBody>,
) -> HttpResponse {
    let delete_user_info = json.clone();
    let result: HttpResponse =
        match user_controller::delete_user(data, delete_user_info.admin_id, delete_user_info.id)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}
