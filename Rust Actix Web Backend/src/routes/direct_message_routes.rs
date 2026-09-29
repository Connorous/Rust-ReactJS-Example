use crate::controllers::direct_message_controller;
use crate::extractors::{errors, user_type, RequireUserType};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ListMessagesRequestBody {
    pub relationship_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct SearchMessagesRequestBody {
    pub relationship_id: i64,
    pub message_content: String,
}

#[derive(Deserialize, Clone)]
pub struct SendMessageRequestBody {
    pub relationship_id: i64,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateMessageRequestBody {
    pub message_id: i64,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct DeleteMessageRequestBody {
    pub message_id: i64,
}

pub async fn list_messages(
    data: web::Data<AppState>,
    claims: RequireUserType<{ user_type::VIEWER }, { errors::READ_DIRECT_MESSAGES }>,
    json: web::Json<ListMessagesRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match direct_message_controller::list_messages(
        data,
        claims.0,
        body.relationship_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn search_messages(
    data: web::Data<AppState>,
    claims: RequireUserType<{ user_type::VIEWER }, { errors::READ_DIRECT_MESSAGES }>,
    json: web::Json<SearchMessagesRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match direct_message_controller::search_messages(
        data,
        claims.0,
        body.relationship_id,
        body.message_content,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn send_message(
    data: web::Data<AppState>,
    claims: RequireUserType<{ user_type::STANDARD_USER }, { errors::SEND_DIRECT_MESSAGE }>,
    json: web::Json<SendMessageRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match direct_message_controller::send_message(
        data,
        claims.0,
        body.relationship_id,
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
    claims: RequireUserType<{ user_type::STANDARD_USER }, { errors::UPDATE_DIRECT_MESSAGE }>,
    json: web::Json<UpdateMessageRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match direct_message_controller::update_message(
        data,
        claims.0,
        body.message_id,
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
    claims: RequireUserType<{ user_type::STANDARD_USER }, { errors::DELETE_DIRECT_MESSAGE }>,
    json: web::Json<DeleteMessageRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match direct_message_controller::delete_message(data, claims.0, body.message_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}
