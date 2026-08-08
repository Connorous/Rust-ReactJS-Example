use crate::controllers::chat_group_controller;
use crate::extractors::{errors, global, group, RequireGlobal, RequireGroup};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GetGroupRequestBody {
    pub group_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct NewGroupRequestBody {
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateGroupRequestBody {
    pub group_id: i64,
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct DeleteGroupRequestBody {
    pub group_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct ListMessagesRequestBody {
    pub group_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct SendMessageRequestBody {
    pub group_id: i64,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateMessageRequestBody {
    pub message_id: i64,
    pub group_id: i64,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct DeleteMessageRequestBody {
    pub message_id: i64,
    pub group_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct ListGroupPermissionsRequestBody {
    pub group_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct AddGroupPermissionRequestBody {
    pub group_id: i64,
    pub user_id: i64,
    pub permission_type_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateGroupPermissionRequestBody {
    pub group_id: i64,
    pub user_id: i64,
    pub permission_type_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct DeleteGroupPermissionRequestBody {
    pub group_id: i64,
    pub user_id: i64,
}

pub async fn list_groups(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }, { errors::READ_GROUP }>,
) -> HttpResponse {
    let result: HttpResponse = match chat_group_controller::list_groups(data, claims.0).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn get_group(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }, { errors::READ_GROUP }>,
    json: web::Json<GetGroupRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::get_group(data, claims.0, body.group_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn new_group(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::CREATE_GROUP }>,
    json: web::Json<NewGroupRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::new_group(data, claims.0, body.name).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn update_group(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::UPDATE_GROUP }>,
    json: web::Json<UpdateGroupRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::update_group(data, claims.0, body.group_id, body.name).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn delete_group(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::DELETE_GROUP }>,
    json: web::Json<DeleteGroupRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::delete_group(data, claims.0, body.group_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn list_messages(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }, { errors::READ_GROUP }>,
    json: web::Json<ListMessagesRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::list_messages(data, claims.0, body.group_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn send_message(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::SEND_GROUP_MESSAGE }>,
    json: web::Json<SendMessageRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match chat_group_controller::send_message(
        data,
        claims.0,
        body.group_id,
        body.message,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn update_message(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::UPDATE_GROUP_MESSAGE }>,
    json: web::Json<UpdateMessageRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match chat_group_controller::update_message(
        data,
        claims.0,
        body.message_id,
        body.group_id,
        body.message,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn delete_message(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::DELETE_GROUP_MESSAGE }>,
    json: web::Json<DeleteMessageRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::delete_message(data, claims.0, body.message_id, body.group_id)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn list_group_permissions(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::VIEWER }, { errors::READ_GROUP }>,
    json: web::Json<ListGroupPermissionsRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match chat_group_controller::list_group_permissions(data, claims.0, body.group_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn add_group_permission(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::ADD_GROUP_MEMBER }>,
    json: web::Json<AddGroupPermissionRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match chat_group_controller::add_group_permission(
        data,
        claims.0,
        body.group_id,
        body.user_id,
        body.permission_type_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn update_group_permission(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::UPDATE_GROUP_MEMBER }>,
    json: web::Json<UpdateGroupPermissionRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match chat_group_controller::update_group_permission(
        data,
        claims.0,
        body.group_id,
        body.user_id,
        body.permission_type_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn delete_group_permission(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ global::STANDARD_USER }, { errors::REMOVE_GROUP_MEMBER }>,
    json: web::Json<DeleteGroupPermissionRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match chat_group_controller::delete_group_permission(
        data,
        claims.0,
        body.group_id,
        body.user_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}
