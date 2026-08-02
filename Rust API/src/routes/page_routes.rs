use crate::controllers::page_controller;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ListUserPagesRequestBody {
    session_user_id: i64,
    pages_user_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct GetPageRequestBody {
    session_user_id: i64,
    page_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct NewPageRequestBody {
    created_by_id: i64,
    title: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePageRequestBody {
    session_user_id: i64,
    page_id: i64,
    published: bool,
    title: String,
    selected_css_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct DeletePageRequestBody {
    session_user_id: i64,
    page_id: i64,
}

pub async fn list_pages_usermade(
    data: web::Data<AppState>,
    json: web::Json<ListUserPagesRequestBody>,
) -> HttpResponse {
    let list_pages_info = json.clone();

    let result: HttpResponse = match page_controller::list_pages_usermade(
        data,
        list_pages_info.session_user_id,
        list_pages_info.pages_user_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn list_all_pages(data: web::Data<AppState>, id: web::Path<i64>) -> HttpResponse {
    let user_id: i64 = id.into_inner();
    let result: HttpResponse = match page_controller::list_all_pages(data, user_id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn list_all_page_creators(
    data: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i64>,
) -> HttpResponse {
    let user_id: i64 = id.into_inner();
    let result: HttpResponse = match page_controller::list_all_page_creators(data, user_id).await {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body(format!("Server Error : {}", e)),
    };

    result
}

pub async fn get_page(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<GetPageRequestBody>,
) -> HttpResponse {
    let get_page_info: GetPageRequestBody = json.clone();
    let result: HttpResponse =
        match page_controller::get_page(data, get_page_info.session_user_id, get_page_info.page_id)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body("Server Error"),
        };

    result
}

pub async fn new_page(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<NewPageRequestBody>,
) -> HttpResponse {
    let new_page_info = json.clone();
    let result: HttpResponse =
        match page_controller::new_page(data, new_page_info.created_by_id, new_page_info.title)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body("Server Error"),
        };

    result
}

pub async fn update_page(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<UpdatePageRequestBody>,
) -> HttpResponse {
    let update_page_info = json.clone();
    let result: HttpResponse = match page_controller::update_page(
        data,
        update_page_info.session_user_id,
        update_page_info.page_id,
        update_page_info.published,
        update_page_info.title,
        update_page_info.selected_css_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn delete_page(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<DeletePageRequestBody>,
) -> HttpResponse {
    let delete_page_info = json.clone();
    let result: HttpResponse = match page_controller::delete_page(
        data,
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

#[derive(Deserialize, Clone)]
pub struct DeletePagesRequestBody {
    created_by_id: i64,
}

/*
pub async fn delete_all_pages_usermade(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<DeletePagesRequestBody>,
) -> HttpResponse {
    let delete_pages_info = json.clone();
    let result: HttpResponse =
        match page_controller::delete_all_pages_usermade(data, delete_pages_info.created_by_id)
            .await
        {
            Ok(res) => res,
            Err(e) => HttpResponse::BadRequest().body("Server Error"),
        };

    result
}
    */
