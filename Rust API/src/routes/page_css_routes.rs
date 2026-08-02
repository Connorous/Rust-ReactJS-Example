use crate::controllers::page_css_controller;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GetAllPageCssRequestBody {
    session_user_id: i64,
    page_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct NewPageCssRequestBody {
    page_id: i64,
    sheet_name: String,
    css: String,
    session_user_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePageCssRequestBody {
    id: i64,
    session_user_id: i64,
    page_id: i64,
    css: String,
}

#[derive(Deserialize, Clone)]
pub struct DeletePageCssRequestBody {
    id: i64,
    session_user_id: i64,
    page_id: i64,
}

pub async fn list_page_css(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<GetAllPageCssRequestBody>,
) -> HttpResponse {
    let list_page_css_info = json.clone();
    let result: HttpResponse = match page_css_controller::list_page_css(
        data,
        list_page_css_info.session_user_id,
        list_page_css_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn get_page_css(
    data: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<i64>,
) -> HttpResponse {
    let selected_css_id = id.into_inner();
    let result: HttpResponse = match page_css_controller::get_page_css(data, selected_css_id).await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn new_page_css(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<NewPageCssRequestBody>,
) -> HttpResponse {
    let new_page_css_info = json.clone();
    let result: HttpResponse = match page_css_controller::new_page_css(
        data,
        new_page_css_info.session_user_id,
        new_page_css_info.page_id,
        new_page_css_info.sheet_name,
        new_page_css_info.css,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn update_page_css(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<UpdatePageCssRequestBody>,
) -> HttpResponse {
    let update_page_permission_info = json.clone();

    let result: HttpResponse = match page_css_controller::update_page_css(
        data,
        update_page_permission_info.id,
        update_page_permission_info.session_user_id,
        update_page_permission_info.page_id,
        update_page_permission_info.css,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn delete_page_css(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<DeletePageCssRequestBody>,
) -> HttpResponse {
    let delete_page_info = json.clone();
    let result: HttpResponse = match page_css_controller::delete_page_css(
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
