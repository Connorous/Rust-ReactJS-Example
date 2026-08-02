use crate::state::AppState;
use actix_web::{
    error,
    web::{self, get},
    HttpResponse,
};
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::postgres::PgQueryResult;

#[derive(Debug, Clone, Serialize)]
struct PageCSS {
    id: i64,
    page_id: i64,
    created_by_id: i64,
    sheet_name: Option<String>,
    css: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    data: Option<PageCSS>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseList {
    msg: String,
    data: Option<Vec<PageCSS>>,
    success: bool,
}

pub async fn list_page_css(
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
            let css: Vec<PageCSS> = sqlx::query_as!(
        PageCSS,
        "SELECT id, page_id, created_by_id, sheet_name, css FROM page_css WHERE page_id = $1 ORDER BY id",
        page_id
    )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            if (!css.is_empty()) {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(css),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Query failed, no CSS related to Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else if (user.user_type_id <= 3) {
            let get_permissions = sqlx::query!(
                "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id, page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

            if (!get_permissions.is_none()) {
                let permissions = get_permissions.unwrap();
                if (permissions.permission_type_id <= 2) {
                    let get_css = sqlx::query_as!(PageCSS,
                "SELECT id, page_id, sheet_name, css, created_by_id FROM page_css WHERE page_id = $1 ORDER BY id",
                page_id,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

                    if (!get_css.is_empty()) {
                        let response = ResponseList {
                            msg: String::from("Success"),
                            data: Some(get_css),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    } else {
                        let response = Response {
                            msg: String::from("Query failed, no CSS related to Page."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("Query failed, User lacks permissions to edit Page."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("Query failed, User has no Permissions related to Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, User lacks Permissions to get all CSS of Page."),
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
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn get_page_css(
    data: web::Data<AppState>,
    selected_css_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_css = sqlx::query_as!(
        PageCSS,
        "SELECT id, page_id, sheet_name, css, created_by_id FROM page_css WHERE id = $1",
        selected_css_id,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

    if (!get_css.is_none()) {
        let css = get_css.unwrap();
        let response = Response {
            msg: String::from("Success"),
            data: Some(css),
            success: true,
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Query failed, User could not be found."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn new_page_css(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
    sheet_name: String,
    css: String,
) -> Result<HttpResponse, actix_web::Error> {
    if (sheet_name.to_lowercase() == "default") {
        let response = Response {
            msg: String::from("Cannot create a Default CSS Sheet, as one should already Exist."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    } else {
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
            let session_user = get_user.unwrap();
            if (session_user.user_type_id <= 2) {
                let new_css =
                sqlx::query("INSERT INTO page_css (page_id, created_by_id, sheet_name, css) VALUES ($1, $2, $3, $4)").bind(page_id).bind(session_user_id).bind(sheet_name).bind(css)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;
                if (!new_css.rows_affected() > 0) {
                    let response = Response {
                        msg: String::from("Success"),
                        data: None,
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                } else {
                    let response = Response {
                        msg: String::from("Query failed, no Page CSS Created."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else if (session_user.user_type_id == 3) {
                let get_permissions = sqlx::query!(
                "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id, page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

                if (!get_permissions.is_none()) {
                    let permissions = get_permissions.unwrap();
                    if (permissions.permission_type_id <= 2) {
                        let new_css =
                sqlx::query("INSERT INTO page_css (page_id, created_by_id, sheet_name, css) VALUES ($1, $2, $3, $4)").bind(page_id).bind(session_user_id).bind(sheet_name).bind(css)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;
                        if (!new_css.rows_affected() > 0) {
                            let response = Response {
                                msg: String::from("Success"),
                                data: None,
                                success: true,
                            };
                            Ok(HttpResponse::Ok().json(response))
                        } else {
                            let response = Response {
                                msg: String::from("Query failed, no Page CSS Created."),
                                data: None,
                                success: false,
                            };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    } else {
                        let response = Response {
                        msg: String::from(
                            "User does not have Permissions to Create a Css Sheet to this Page.",
                        ),
                        data: None,
                        success: false,
                    };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("User does not have Permissions to Create a Css Sheet."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have Permission to Create any Css Sheets."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, no user found."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

pub async fn update_page_css(
    data: web::Data<AppState>,
    id: i64,
    session_user_id: i64,
    page_id: i64,
    css: String,
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
        let session_user = get_user.unwrap();
        if (session_user.user_type_id <= 2) {
            let update_css: sqlx::postgres::PgQueryResult =
                sqlx::query("UPDATE page_css SET css = $1 WHERE id = $2")
                    .bind(css)
                    .bind(id)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;
            if (update_css.rows_affected() == 0) {
                let response = Response {
                    msg: String::from("Query failed, no Page CSS updated, it may not exist."),
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
        } else if (session_user.user_type_id == 3) {
            let get_permissions = sqlx::query!(
                "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id, page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

            if (!get_permissions.is_none()) {
                let permissions = get_permissions.unwrap();
                if (permissions.permission_type_id <= 2) {
                    let update_css: sqlx::postgres::PgQueryResult =
                        sqlx::query("UPDATE page_css SET css = $1 WHERE id = $2")
                            .bind(css)
                            .bind(id)
                            .execute(&pool)
                            .await
                            .map_err(|e| {
                                error::ErrorBadRequest(
                                    serde_json::to_string(&e.to_string())
                                        .unwrap_or_else(|_| "{}".to_string()),
                                )
                            })?;

                    if (update_css.rows_affected() == 0) {
                        let response = Response {
                            msg: String::from(
                                "Query failed, no Page CSS Updated, it may not exist.",
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
                            "User does not have Permissions to Edit Css of this Page.",
                        ),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have Permissions to Edit Css."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permission to Edit any Css."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, no user found."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn delete_page_css(
    data: web::Data<AppState>,
    id: i64,
    session_user_id: i64,
    page_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_css = sqlx::query!("SELECT id, sheet_name FROM page_css WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;
    if (!get_css.is_none()) {
        let css = get_css.unwrap();
        if (css.sheet_name.to_lowercase() == "default") {
            let response = Response {
                msg: String::from("Cannot Delete Default Page Css."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        } else {
            let get_page = sqlx::query!(
                "SELECT id, selected_css_id FROM pages WHERE id = $1",
                page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;
            if (!get_page.is_none()) {
                let page = get_page.unwrap();
                if (page.selected_css_id == Some(id)) {
                    let response = Response {
                        msg: String::from("Cannot Delete Css currently assigned to Page."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                } else {
                    let get_user = sqlx::query!(
                        "SELECT id, user_type_id FROM users WHERE id = $1",
                        session_user_id
                    )
                    .fetch_optional(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;
                    if (!get_user.is_none()) {
                        let session_user = get_user.unwrap();
                        if (session_user.user_type_id <= 2) {
                            let result: sqlx::postgres::PgQueryResult =
                                sqlx::query("DELETE FROM page_css WHERE id = $1")
                                    .bind(id)
                                    .execute(&pool)
                                    .await
                                    .map_err(|e| error::ErrorBadRequest(e))?;

                            if (result.rows_affected() == 0) {
                                let response = Response {
                                    msg: String::from(
                                        "Query failed, no Page CSS Deleted, may not Exist.",
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
                        } else if (session_user.user_type_id == 3) {
                            let get_permissions = sqlx::query!(
                "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id, page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

                            if (!get_permissions.is_none()) {
                                let permissions = get_permissions.unwrap();
                                if (permissions.permission_type_id == 1) {
                                    let result: sqlx::postgres::PgQueryResult =
                                        sqlx::query("DELETE FROM page_css WHERE id = $1")
                                            .bind(id)
                                            .execute(&pool)
                                            .await
                                            .map_err(|e| error::ErrorBadRequest(e))?;

                                    if (result.rows_affected() == 0) {
                                        let response = Response {
                                            msg: String::from(
                                                "Query failed, no Page CSS Deleted, may not exist.",
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
                                    "User does not have Permissions to Delete Css of this Page.",
                                ),
                                data: None,
                                success: false,
                            };
                                    Ok(HttpResponse::BadRequest().json(response))
                                }
                            } else {
                                let response = Response {
                                    msg: String::from(
                                        "User does not have Permissions to Delete Css.",
                                    ),
                                    data: None,
                                    success: false,
                                };
                                Ok(HttpResponse::BadRequest().json(response))
                            }
                        } else {
                            let response = Response {
                                msg: String::from(
                                    "User does not have Permission to Delete any Css.",
                                ),
                                data: None,
                                success: false,
                            };
                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    } else {
                        let response = Response {
                            msg: String::from("Query failed, no user found."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                }
            } else {
                let response = Response {
                    msg: String::from("Query failed, no page found."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, no page css found."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}
