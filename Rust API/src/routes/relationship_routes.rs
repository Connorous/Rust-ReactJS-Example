use crate::controllers::relationship_controller;
use crate::extractors::{errors, user_type, RequireGlobal};
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UserId {
    pub user_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct NewRelationshipRequestBody {
    pub receiver_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateRelationshipRequestBody {
    pub relationship_id: i64,
    pub accepted: bool,
}

#[derive(Deserialize, Clone)]
pub struct BlockRelationshipRequestBody {
    pub relationship_id: i64,
    pub block: bool,
}

#[derive(Deserialize, Clone)]
pub struct DeleteRelationshipRequestBody {
    pub relationship_id: i64,
}

pub async fn list_relationships(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ user_type::VIEWER }, { errors::LIST_RELATIONSHIPS }>,
) -> HttpResponse {
    let result: HttpResponse =
        match relationship_controller::list_relationships(data, claims.0).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn list_user_relationships(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ user_type::ADMIN }, { errors::LIST_RELATIONSHIPS_ADMIN }>,
    json: web::Json<UserId>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match relationship_controller::list_user_relationships(
        data,
        claims.0,
        body.user_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn new_relationship(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ user_type::VIEWER }, { errors::SEND_FRIEND_REQUEST }>,
    json: web::Json<NewRelationshipRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match relationship_controller::new_relationship(data, claims.0, body.receiver_id).await {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}

pub async fn update_relationship(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ user_type::VIEWER }, { errors::UPDATE_RELATIONSHIP }>,
    json: web::Json<UpdateRelationshipRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse = match relationship_controller::update_relationship(
        data,
        claims.0,
        body.relationship_id,
        body.accepted,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn block_relationship(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ user_type::VIEWER }, { errors::UPDATE_RELATIONSHIP }>,
    json: web::Json<BlockRelationshipRequestBody>,
) -> HttpResponse {
    let body: BlockRelationshipRequestBody = json.clone();

    let result: HttpResponse = match relationship_controller::block_relationship(
        data,
        claims.0,
        body.relationship_id,
        body.block,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn delete_relationship(
    data: web::Data<AppState>,
    claims: RequireGlobal<{ user_type::VIEWER }, { errors::DELETE_RELATIONSHIP }>,
    json: web::Json<DeleteRelationshipRequestBody>,
) -> HttpResponse {
    let body = json.clone();

    let result: HttpResponse =
        match relationship_controller::delete_relationship(data, claims.0, body.relationship_id)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
        };

    result
}
