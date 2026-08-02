use crate::state::AppState;
use actix_web::{error, web, HttpResponse};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct PageElement {
    id: i64,
    element_type_id: i64,
    parent_element_id: Option<i64>,
    page_id: i64,
    position: Option<i16>,
    content: Option<String>,
    link: Option<String>,
    css_class_name: Option<String>,
}

#[derive(Clone, Serialize)]
struct Response {
    msg: String,
    data: Option<PageElement>,
    success: bool,
}

#[derive(Clone, Serialize)]
struct ResponseList {
    msg: String,
    data: Option<Vec<PageElement>>,
    success: bool,
}

#[derive(Clone, Serialize)]
struct PageElementType {
    id: i64,
    r#type: String,
}

#[derive(Clone, Serialize)]
struct ResponseTypesList {
    msg: String,
    data: Option<Vec<PageElementType>>,
    success: bool,
}

#[derive(Deserialize, Clone)]
pub struct NewPageElement {
    element_type_id: i64,
    parent_element_id: Option<i64>,
    page_id: i64,
    position: i16,
    content: String,
    link: String,
    css_class_name: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdatingPageElement {
    id: i64,
    element_type_id: i64,
    parent_element_id: Option<i64>,
    page_id: i64,
    position: i16,
    content: String,
    link: String,
    css_class_name: String,
}

#[derive(Deserialize, Clone)]
pub struct DeletingPageElement {
    id: i64,
}

pub async fn list_page_elements(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

    if (!get_user.is_none()) {
        let user = get_user.unwrap();
        if (user.user_type_id <= 2) {
            let page_elements: Vec<PageElement> = sqlx::query_as!(
        PageElement,
        "SELECT id, element_type_id, parent_element_id, page_id, position, content, link, css_class_name FROM page_elements WHERE page_id = $1 ORDER BY position",
        page_id,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

            if (page_elements.len() == 0) {
                let empty_list: Vec<PageElement> = Vec::new();

                let response = ResponseList {
                    msg: String::from(
                        "Query failed, no Page Elements, none may exist for the page.",
                    ),
                    data: Some(empty_list),
                    success: true,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(page_elements),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            }
        } else if (user.user_type_id <= 4) {
            let get_page_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let permissions = get_page_permission.unwrap();
            if (permissions.permission_type_id <= 4) {
                let page_elements: Vec<PageElement> = sqlx::query_as!(
        PageElement,
        "SELECT id, element_type_id, parent_element_id, page_id, position, content, link, css_class_name FROM page_elements WHERE page_id = $1 ORDER BY position",
        page_id,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

                if (page_elements.len() == 0) {
                    let empty_list: Vec<PageElement> = Vec::new();

                    let response = ResponseList {
                        msg: String::from(
                            "Query failed, no Page Elements, none may exist for the page.",
                        ),
                        data: Some(empty_list),
                        success: true,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                } else {
                    let response = ResponseList {
                        msg: String::from("Success"),
                        data: Some(page_elements),
                        success: true,
                    };

                    Ok(HttpResponse::Ok().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have Permission to View Page."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Ok().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to View Page."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, User could not be found."),
            data: None,
            success: false,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn list_page_element_types(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

    if (!get_user.is_none()) {
        let user = get_user.unwrap();
        if (user.user_type_id <= 2) {
            let page_element_types: Vec<PageElementType> = sqlx::query_as!(
                PageElementType,
                "SELECT id, type FROM page_element_types ORDER BY id"
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            if (page_element_types.len() == 0) {
                let response = Response {
                    msg: String::from("Query failed, no Page Element Types seem to Exist."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseTypesList {
                    msg: String::from("Success"),
                    data: Some(page_element_types),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            }
        } else if (user.user_type_id <= 4) {
            let get_page_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let permissions = get_page_permission.unwrap();
            if (permissions.permission_type_id <= 4) {
                let page_element_types: Vec<PageElementType> = sqlx::query_as!(
                    PageElementType,
                    "SELECT id, type FROM page_element_types ORDER BY id"
                )
                .fetch_all(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                if (page_element_types.len() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, no Page Element Types seem to Exist."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                } else {
                    let response = ResponseTypesList {
                        msg: String::from("Success"),
                        data: Some(page_element_types),
                        success: true,
                    };

                    Ok(HttpResponse::Ok().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have Permissions to View Page."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Ok().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permission to View Page."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, User could not be found."),
            data: None,
            success: false,
        };

        Ok(HttpResponse::Ok().json(response))
    }
}

pub async fn new_page_elements(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
    new_page_elements: Vec<NewPageElement>,
) -> Result<HttpResponse, actix_web::Error> {
    if (new_page_elements.len() > 0) {
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

        let get_user = sqlx::query!(
            "SELECT id, user_type_id FROM users WHERE id = $1",
            session_user_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;

        if (!get_user.is_none()) {
            let user = get_user.unwrap();

            if (user.user_type_id <= 2) {
                let mut element_type_ids: Vec<i64> = Vec::with_capacity(new_page_elements.len());
                let mut parent_element_ids: Vec<Option<i64>> =
                    Vec::with_capacity(new_page_elements.len());
                let mut page_ids: Vec<i64> = Vec::with_capacity(new_page_elements.len());
                let mut positions: Vec<i16> = Vec::with_capacity(new_page_elements.len());
                let mut contents: Vec<String> = Vec::with_capacity(new_page_elements.len());
                let mut links: Vec<String> = Vec::with_capacity(new_page_elements.len());
                let mut css_class_names: Vec<String> = Vec::with_capacity(new_page_elements.len());

                for new_page_element in new_page_elements {
                    element_type_ids.push(new_page_element.element_type_id);
                    parent_element_ids.push(new_page_element.parent_element_id);
                    page_ids.push(new_page_element.page_id);
                    positions.push(new_page_element.position);
                    contents.push(new_page_element.content);
                    links.push(new_page_element.link);
                    css_class_names.push(new_page_element.css_class_name);
                }

                let result: sqlx::postgres::PgQueryResult = sqlx::query(
                "INSERT INTO page_elements (element_type_id, parent_element_id, page_id, position, content, link, css_class_name) 
         SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::bigint[], $4::smallint[], $5::varchar[], $6::varchar[], $7::varchar[])",
            )
            .bind(&element_type_ids)
            .bind(&parent_element_ids)
            .bind(&page_ids)
            .bind(&positions)
            .bind(&contents)
            .bind(&links)
            .bind(&css_class_names)
            .execute(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

                if (result.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, no new Page Elements created."),
                        data: None,
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                } else {
                    let response = Response {
                        msg: String::from("Success"),
                        data: None,
                        success: true,
                    };

                    Ok(HttpResponse::Ok().json(response))
                }
            } else if (user.user_type_id == 3) {
                let get_page_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                if (!get_page_permission.is_none()) {
                    let page_permissions = get_page_permission.unwrap();

                    if (page_permissions.permission_type_id <= 2) {
                        let mut element_type_ids: Vec<i64> =
                            Vec::with_capacity(new_page_elements.len());
                        let mut parent_element_ids: Vec<Option<i64>> =
                            Vec::with_capacity(new_page_elements.len());
                        let mut page_ids: Vec<i64> = Vec::with_capacity(new_page_elements.len());
                        let mut positions: Vec<i16> = Vec::with_capacity(new_page_elements.len());
                        let mut contents: Vec<String> = Vec::with_capacity(new_page_elements.len());
                        let mut links: Vec<String> = Vec::with_capacity(new_page_elements.len());
                        let mut css_class_names: Vec<String> =
                            Vec::with_capacity(new_page_elements.len());

                        for new_page_element in new_page_elements {
                            element_type_ids.push(new_page_element.element_type_id);
                            parent_element_ids.push(new_page_element.parent_element_id);
                            page_ids.push(new_page_element.page_id);
                            positions.push(new_page_element.position);
                            contents.push(new_page_element.content);
                            links.push(new_page_element.link);
                            css_class_names.push(new_page_element.css_class_name);
                        }

                        let result: sqlx::postgres::PgQueryResult = sqlx::query(
                "INSERT INTO page_elements (element_type_id, parent_element_id, page_id, position, content, link, css_class_name) 
         SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::bigint[], $4::smallint[], $5::varchar[], $6::varchar[], $7::varchar[])",
            )
            .bind(&element_type_ids)
            .bind(&parent_element_ids)
            .bind(&page_ids)
            .bind(&positions)
            .bind(&contents)
            .bind(&links)
            .bind(&css_class_names)
            .execute(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

                        if (!result.rows_affected() == 0) {
                            let response = Response {
                                msg: String::from("Query failed, no new Page Element created."),
                                data: None,
                                success: false,
                            };
                            Ok(HttpResponse::BadRequest().json(response))
                        } else {
                            let response = Response {
                                msg: String::from("Success"),
                                data: None,
                                success: true,
                            };

                            Ok(HttpResponse::Ok().json(response))
                        }
                    } else {
                        let response = Response {
                            msg: String::from(
                                "Query failed, User lacks Permission to Create Page Elements.",
                            ),
                            data: None,
                            success: false,
                        };
                        return Ok(HttpResponse::BadRequest().json(response));
                    }
                } else {
                    let response = Response {
                        msg: String::from(
                            "Query failed, User lacks Permission to Create Page Elements.",
                        ),
                        data: None,
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from(
                        "Query failed, User lacks Permissions to Create Page Elements.",
                    ),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, User could not be found."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("No new Page Elements Received."),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn update_page_elements(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
    updating_page_elements: Vec<UpdatingPageElement>,
) -> Result<HttpResponse, actix_web::Error> {
    if (updating_page_elements.len() > 0) {
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

        let get_user = sqlx::query!(
            "SELECT id, user_type_id FROM users WHERE id = $1",
            session_user_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;

        if (!get_user.is_none()) {
            let user = get_user.unwrap();

            if (user.user_type_id <= 2) {
                let mut ids: Vec<i64> = Vec::with_capacity(updating_page_elements.len());
                let mut element_type_ids: Vec<i64> =
                    Vec::with_capacity(updating_page_elements.len());
                let mut parent_element_ids: Vec<Option<i64>> =
                    Vec::with_capacity(updating_page_elements.len());
                let mut page_ids: Vec<i64> = Vec::with_capacity(updating_page_elements.len());
                let mut positions: Vec<i16> = Vec::with_capacity(updating_page_elements.len());
                let mut contents: Vec<String> = Vec::with_capacity(updating_page_elements.len());
                let mut links: Vec<String> = Vec::with_capacity(updating_page_elements.len());
                let mut css_class_names: Vec<String> =
                    Vec::with_capacity(updating_page_elements.len());

                for updating_page_element in updating_page_elements {
                    ids.push(updating_page_element.id);
                    element_type_ids.push(updating_page_element.element_type_id);
                    parent_element_ids.push(updating_page_element.parent_element_id);
                    page_ids.push(updating_page_element.page_id);
                    positions.push(updating_page_element.position);
                    contents.push(updating_page_element.content);
                    links.push(updating_page_element.link);
                    css_class_names.push(updating_page_element.css_class_name);
                }

                let result: sqlx::postgres::PgQueryResult = sqlx::query(
                "UPDATE page_elements SET element_type_id = data.element_type_id, parent_element_id = data.parent_element_id, page_id = data.page_id, position = data.position, content = data.content, link = data.link, css_class_name = data.css_class_name FROM 
         (SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::bigint[], $4::bigint[], $5::smallint[], $6::text[], $7::text[], $8::text[]) AS data(id, element_type_id, parent_element_id, page_id, position, content, link, css_class_name)) AS data WHERE page_elements.id = data.id"
            )
            .bind(&ids)
            .bind(&element_type_ids)
            .bind(&parent_element_ids)
            .bind(&page_ids)
            .bind(&positions)
            .bind(&contents)
            .bind(&links)
            .bind(&css_class_names)
            .execute(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

                if (result.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from(
                            "Query failed, no Page Elements updated, they may not exist.",
                        ),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                } else {
                    let response = Response {
                        msg: String::from("Success"),
                        data: None,
                        success: true,
                    };

                    Ok(HttpResponse::Ok().json(response))
                }
            } else if (user.user_type_id == 3) {
                let get_page_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                if (!get_page_permission.is_none()) {
                    let page_permissions = get_page_permission.unwrap();

                    if (page_permissions.permission_type_id <= 2) {
                        let mut ids: Vec<i64> = Vec::with_capacity(updating_page_elements.len());
                        let mut element_type_ids: Vec<i64> =
                            Vec::with_capacity(updating_page_elements.len());
                        let mut parent_element_ids: Vec<Option<i64>> =
                            Vec::with_capacity(updating_page_elements.len());
                        let mut page_ids: Vec<i64> =
                            Vec::with_capacity(updating_page_elements.len());
                        let mut positions: Vec<i16> =
                            Vec::with_capacity(updating_page_elements.len());
                        let mut contents: Vec<String> =
                            Vec::with_capacity(updating_page_elements.len());
                        let mut links: Vec<String> =
                            Vec::with_capacity(updating_page_elements.len());
                        let mut css_class_names: Vec<String> =
                            Vec::with_capacity(updating_page_elements.len());

                        for updating_page_element in updating_page_elements {
                            ids.push(updating_page_element.id);
                            element_type_ids.push(updating_page_element.element_type_id);
                            parent_element_ids.push(updating_page_element.parent_element_id);
                            page_ids.push(updating_page_element.page_id);
                            positions.push(updating_page_element.position);
                            contents.push(updating_page_element.content);
                            links.push(updating_page_element.link);
                            css_class_names.push(updating_page_element.css_class_name);
                        }

                        let result: sqlx::postgres::PgQueryResult = sqlx::query(
                "UPDATE page_elements SET element_type_id = data.element_type_id, parent_element_id = data.parent_element_id, page_id = data.page_id, position = data.position, content = data.content, link = data.link, css_class_name = data.css_class_name FROM 
         (SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::bigint[], $4::bigint[], $5::smallint[], $6::text[], $7::text[], $8::text[]) AS data(id, element_type_id, parent_element_id, page_id, position, content, link, css_class_name)) AS data WHERE page_elements.id = data.id"
            )
            .bind(&ids)
            .bind(&element_type_ids)
            .bind(&parent_element_ids)
            .bind(&page_ids)
            .bind(&positions)
            .bind(&contents)
            .bind(&links)
            .bind(&css_class_names)
            .execute(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

                        if (result.rows_affected() == 0) {
                            let response = Response {
                                msg: String::from(
                                    "Query failed, no Page Elements updated, they may not exist.",
                                ),
                                data: None,
                                success: false,
                            };
                            Ok(HttpResponse::BadRequest().json(response))
                        } else {
                            let response = Response {
                                msg: String::from("Success"),
                                data: None,
                                success: true,
                            };

                            Ok(HttpResponse::Ok().json(response))
                        }
                    } else {
                        let response = Response {
                            msg: String::from(
                                "Query failed, User lacks Permission to Create Page Elements.",
                            ),
                            data: None,
                            success: false,
                        };
                        return Ok(HttpResponse::BadRequest().json(response));
                    }
                } else {
                    let response = Response {
                        msg: String::from(
                            "Query failed, User lacks Permission to Update Page Elements.",
                        ),
                        data: None,
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from(
                        "Query failed, User lacks Permissions to Update Page Elements.",
                    ),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, User could not be found."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("No updating Page Elements Received."),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn delete_page_elements(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
    deleting_page_elements: Vec<DeletingPageElement>,
) -> Result<HttpResponse, actix_web::Error> {
    if (deleting_page_elements.len() > 0) {
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

        let get_user = sqlx::query!(
            "SELECT id, user_type_id FROM users WHERE id = $1",
            session_user_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;

        if (!get_user.is_none()) {
            let user = get_user.unwrap();

            if (user.user_type_id <= 2) {
                let mut ids: Vec<i64> = Vec::with_capacity(deleting_page_elements.len());

                for deleting_page_element in deleting_page_elements {
                    ids.push(deleting_page_element.id);
                }

                let result: sqlx::postgres::PgQueryResult = sqlx::query(
                    "DELETE FROM page_elements
WHERE id IN (SELECT UNNEST($1::bigint[]));",
                )
                .bind(&ids)
                .execute(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                if (result.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from(
                            "Query failed, no Page Elements Deleted, they may not Exist.",
                        ),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                } else {
                    let response = Response {
                        msg: String::from("Success"),
                        data: None,
                        success: true,
                    };

                    Ok(HttpResponse::Ok().json(response))
                }
            } else if (user.user_type_id == 3) {
                let get_page_permission = sqlx::query!(
        "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                if (!get_page_permission.is_none()) {
                    let page_permissions = get_page_permission.unwrap();

                    if (page_permissions.permission_type_id <= 2) {
                        let mut ids: Vec<i64> = Vec::with_capacity(deleting_page_elements.len());

                        for deleting_page_element in deleting_page_elements {
                            ids.push(deleting_page_element.id);
                        }

                        let result: sqlx::postgres::PgQueryResult = sqlx::query(
                            "DELETE FROM page_elements
WHERE id IN (SELECT UNNEST($1::bigint[]));",
                        )
                        .bind(&ids)
                        .execute(&pool)
                        .await
                        .map_err(|e| {
                            error::ErrorBadRequest(
                                serde_json::to_string(&e.to_string())
                                    .unwrap_or_else(|_| "{}".to_string()),
                            )
                        })?;

                        if (result.rows_affected() == 0) {
                            let response = Response {
                                msg: String::from(
                                    "Query failed, no Page Elements Deleted, they may not Exist.",
                                ),
                                data: None,
                                success: false,
                            };
                            Ok(HttpResponse::BadRequest().json(response))
                        } else {
                            let response = Response {
                                msg: String::from("Success"),
                                data: None,
                                success: true,
                            };

                            Ok(HttpResponse::Ok().json(response))
                        }
                    } else {
                        let response = Response {
                            msg: String::from(
                                "Query failed, User lacks Permission to Delete Page Elements.",
                            ),
                            data: None,
                            success: false,
                        };
                        return Ok(HttpResponse::BadRequest().json(response));
                    }
                } else {
                    let response = Response {
                        msg: String::from(
                            "Query failed, User lacks Permission to Delete Page Elements.",
                        ),
                        data: None,
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from(
                        "Query failed, User lacks Permissions to Delete Page Elements.",
                    ),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, User could not be found."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("No deleting Page Elements Received."),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}
