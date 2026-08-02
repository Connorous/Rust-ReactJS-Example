use crate::controllers::page_elements_controller;
use crate::controllers::page_elements_controller::DeletingPageElement;
use crate::controllers::page_elements_controller::NewPageElement;
use crate::controllers::page_elements_controller::UpdatingPageElement;
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ListElementsRequestBody {
    session_user_id: i64,
    page_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct NewPageElementListRequestBody {
    session_user_id: i64,
    page_id: i64,
    new_page_elements: Vec<NewPageElement>,
}

#[derive(Deserialize, Clone)]
pub struct UpdatingPageElementListRequestBody {
    session_user_id: i64,
    page_id: i64,
    updating_page_elements: Vec<UpdatingPageElement>,
}

#[derive(Deserialize, Clone)]
pub struct DeletingPageElementListRequestBody {
    session_user_id: i64,
    page_id: i64,
    deleting_page_elements: Vec<DeletingPageElement>,
}

pub async fn list_page_elements(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<ListElementsRequestBody>,
) -> HttpResponse {
    let list_page_elements_info: ListElementsRequestBody = json.clone();
    let result: HttpResponse = match page_elements_controller::list_page_elements(
        data,
        list_page_elements_info.session_user_id,
        list_page_elements_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn list_page_element_types(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<ListElementsRequestBody>,
) -> HttpResponse {
    let list_page_element_types_info: ListElementsRequestBody = json.clone();
    let result: HttpResponse = match page_elements_controller::list_page_element_types(
        data,
        list_page_element_types_info.session_user_id,
        list_page_element_types_info.page_id,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn new_page_elements(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<NewPageElementListRequestBody>,
    body: String,
) -> HttpResponse {
    let new_page_elements_info = json.clone();

    let result: HttpResponse = match page_elements_controller::new_page_elements(
        data,
        new_page_elements_info.session_user_id,
        new_page_elements_info.page_id,
        new_page_elements_info.new_page_elements,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
    //HttpResponse::Ok().body("Hello world!")
}

pub async fn update_page_elements(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<UpdatingPageElementListRequestBody>,
) -> HttpResponse {
    let updating_page_elements_info = json.clone();
    let result: HttpResponse = match page_elements_controller::update_page_elements(
        data,
        updating_page_elements_info.session_user_id,
        updating_page_elements_info.page_id,
        updating_page_elements_info.updating_page_elements,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}

pub async fn delete_page_element(
    data: web::Data<AppState>,
    req: HttpRequest,
    json: web::Json<DeletingPageElementListRequestBody>,
) -> HttpResponse {
    let deleting_page__elements_info = json.clone();
    let result: HttpResponse = match page_elements_controller::delete_page_elements(
        data,
        deleting_page__elements_info.session_user_id,
        deleting_page__elements_info.page_id,
        deleting_page__elements_info.deleting_page_elements,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => HttpResponse::BadRequest().body("Server Error"),
    };

    result
}
