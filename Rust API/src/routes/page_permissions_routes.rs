use crate::controllers::page_permissions_controller;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ListUserPagePermissionRequestBody {
    session_user_id: i64,
    page_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct GetUserPagePermissionRequestBody {
    user_id: i64,
    page_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct NewPagePermissionRequestBody {
    session_user_id: i64,
    user_id: i64,
    page_id: i64,
    permission_type_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePagePermissionRequestBody {
    id: i64,
    session_user_id: i64,
    user_id: i64,
    page_id: i64,
    permission_type_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct DeletePagePermissionRequestBody {
    id: i64,
    session_user_id: i64,
    page_id: i64,
}

pub async fn list_user_pages_permissions(
    data: web::Data<AppState>,
    json: web::Json<ListUserPagePermissionRequestBody>,
) -> HttpResponse {
    let user_page_permission_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::list_users_page_permissions(
        data,
        user_page_permission_info.session_user_id,
        user_page_permission_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn list_users_with_page_permissions(
    data: web::Data<AppState>,
    json: web::Json<ListUserPagePermissionRequestBody>,
) -> HttpResponse {
    let user_page_permission_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::list_users_with_page_permissions(
        data,
        user_page_permission_info.session_user_id,
        user_page_permission_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };
    result
}

pub async fn list_users_without_page_permissions(
    data: web::Data<AppState>,
    json: web::Json<ListUserPagePermissionRequestBody>,
) -> HttpResponse {
    let user_page_permission_info = json.clone();
    let result: HttpResponse =
        match page_permissions_controller::list_users_without_page_permissions(
            data,
            user_page_permission_info.session_user_id,
            user_page_permission_info.page_id,
        )
        .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body("Server Error"),
        };
    result
}

pub async fn list_page_permission_types(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<ListUserPagePermissionRequestBody>,
) -> HttpResponse {
    let user_page_permission_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::list_page_permission_types(
        data,
        user_page_permission_info.session_user_id,
        user_page_permission_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn get_user_page_permission(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<GetUserPagePermissionRequestBody>,
) -> HttpResponse {
    let user_page_permission_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::get_user_page_permission(
        data,
        user_page_permission_info.user_id,
        user_page_permission_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn new_user_page_permission(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<NewPagePermissionRequestBody>,
) -> HttpResponse {
    let new_page_permission_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::new_user_page_permission(
        data,
        new_page_permission_info.session_user_id,
        new_page_permission_info.user_id,
        new_page_permission_info.page_id,
        new_page_permission_info.permission_type_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn update_user_page_permission(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<UpdatePagePermissionRequestBody>,
) -> HttpResponse {
    let update_page_permission_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::update_user_page_permission(
        data,
        update_page_permission_info.id,
        update_page_permission_info.session_user_id,
        update_page_permission_info.user_id,
        update_page_permission_info.page_id,
        update_page_permission_info.permission_type_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn delete_page_permission(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<DeletePagePermissionRequestBody>,
) -> HttpResponse {
    let delete_page_info = json.clone();
    let result: HttpResponse = match page_permissions_controller::delete_page_permission(
        data,
        delete_page_info.id,
        delete_page_info.session_user_id,
        delete_page_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}
