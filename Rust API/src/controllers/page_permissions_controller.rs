use crate::state::AppState;
use actix_web::{error, web, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct PagePermission {
    id: i64,
    user_id: i64,
    permission_type_id: i64,
}

#[derive(Debug, Clone, Serialize)]
struct PagePermissionUser {
    id: i64,
    username: String,
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    data: Option<PagePermission>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseList {
    msg: String,
    data: Option<Vec<PagePermission>>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseUserList {
    msg: String,
    data: Option<Vec<PagePermissionUser>>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct PagePermissionType {
    id: i64,
    r#type: String,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseTypesList {
    msg: String,
    data: Option<Vec<PagePermissionType>>,
    success: bool,
}

pub async fn list_users_page_permissions(
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
            let users_page_permissions: Vec<PagePermission> = sqlx::query_as!(
                PagePermission,
                "SELECT id, user_id, permission_type_id FROM page_permissions WHERE page_id = $1 ORDER BY id",
                page_id,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;

            if (users_page_permissions.len() == 0) {
                let response = Response {
                    msg: String::from("Query failed, no Page Permissions seem to Exist."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(users_page_permissions),
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
                let page_permission = get_page_permission.unwrap();
                if (page_permission.permission_type_id == 1) {
                    let users_page_permissions: Vec<PagePermission> = sqlx::query_as!(
        PagePermission,
        "SELECT id, user_id, permission_type_id FROM page_permissions WHERE page_id = $1 ORDER BY id",
        page_id,
    )
                    .fetch_all(&pool)
                    .await
                    .map_err(|e| error::ErrorBadRequest(e))?;

                    if (users_page_permissions.len() == 0) {
                        let response = Response {
                            msg: String::from("Query failed, no Page Permissions seem to Exist."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let response = ResponseList {
                            msg: String::from("Success"),
                            data: Some(users_page_permissions),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("User does not have Permissions for this Page."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have any Permissions for this Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to see Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn list_users_with_page_permissions(
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
            let users_with_permissions: Vec<PagePermissionUser> = sqlx::query_as!(
                PagePermissionUser,
                "SELECT id, username FROM users WHERE id IN (SELECT user_id FROM page_permissions WHERE page_id = $1)",
                page_id,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;

            if (users_with_permissions.len() == 0) {
                let response = Response {
                    msg: String::from("Query failed, no Page Permissions seem to Exist."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseUserList {
                    msg: String::from("Success"),
                    data: Some(users_with_permissions),
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
                let page_permission = get_page_permission.unwrap();
                if (page_permission.permission_type_id == 1) {
                    let users_with_permissions: Vec<PagePermissionUser> = sqlx::query_as!(
                PagePermissionUser,
                "SELECT id, username FROM users WHERE id IN (SELECT user_id FROM page_permissions WHERE page_id = $1)",
                page_id
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;
                    if (users_with_permissions.len() == 0) {
                        let response = Response {
                            msg: String::from("Query failed, no Page Permissions seem to Exist."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let response = ResponseUserList {
                            msg: String::from("Success"),
                            data: Some(users_with_permissions),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("User does not have Permissions for this Page."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have any Permissions for this Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to see Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn list_users_without_page_permissions(
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
            let users_without_permissions: Vec<PagePermissionUser> = sqlx::query_as!(
                PagePermissionUser,
                "SELECT id, username FROM users WHERE id NOT IN (SELECT user_id FROM page_permissions WHERE page_id = $1)",
                page_id,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;

            if (users_without_permissions.len() == 0) {
                let response = Response {
                    msg: String::from("Query failed, no Page Permissions seem to Exist."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseUserList {
                    msg: String::from("Success"),
                    data: Some(users_without_permissions),
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
                let page_permission = get_page_permission.unwrap();
                if (page_permission.permission_type_id == 1) {
                    let users_without_permissions: Vec<PagePermissionUser> = sqlx::query_as!(
                PagePermissionUser,
                "SELECT id, username FROM users WHERE id NOT IN (SELECT user_id FROM page_permissions WHERE page_id = $1)",
                page_id,
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;
                    if (users_without_permissions.len() == 0) {
                        let response = Response {
                            msg: String::from("Query failed, no Page Permissions seem to Exist."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let response = ResponseUserList {
                            msg: String::from("Success"),
                            data: Some(users_without_permissions),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("User does not have Permissions for this Page."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have any Permissions for this Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to see Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn list_page_permission_types(
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
            let page_permissions_types: Vec<PagePermissionType> = sqlx::query_as!(
                PagePermissionType,
                "SELECT id, type FROM page_permission_types ORDER BY id"
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;

            if (page_permissions_types.len() == 0) {
                let response = Response {
                    msg: String::from("Query failed, no Page Permission Types seem to Exist."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseTypesList {
                    msg: String::from("Success"),
                    data: Some(page_permissions_types),
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
                let page_permission = get_page_permission.unwrap();
                if (page_permission.permission_type_id == 1) {
                    let page_permissions_types: Vec<PagePermissionType> = sqlx::query_as!(
                        PagePermissionType,
                        "SELECT id, type FROM page_permission_types ORDER BY id"
                    )
                    .fetch_all(&pool)
                    .await
                    .map_err(|e| error::ErrorBadRequest(e))?;

                    if (page_permissions_types.len() == 0) {
                        let response = Response {
                            msg: String::from(
                                "Query failed, no Page Permission Types seem to Exist.",
                            ),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    } else {
                        let response = ResponseTypesList {
                            msg: String::from("Success"),
                            data: Some(page_permissions_types),
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("User does not have Permissions for this Page."),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have any Permissions for this Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to see Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn get_user_page_permission(
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
            let get_permissions = sqlx::query!(
        "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
            if (!get_permissions.is_none()) {
                let actual = get_permissions.unwrap();

                let page_permission = PagePermission {
                    id: actual.id,
                    user_id: actual.user_id,
                    permission_type_id: actual.permission_type_id,
                };
                let response = Response {
                    msg: String::from("Success"),
                    data: Some(page_permission),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
            msg: String::from("Query failed, Page Permission not found, user may not have Permissions to View the Page."),
            data: None,
            success: false,
        };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } else if (user.user_type_id <= 4) {
            let get_page_permission = sqlx::query!(
        "SELECT id, permission_type_id, user_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
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
                let page_permission = get_page_permission.unwrap();

                let page_permission = PagePermission {
                    id: page_permission.id,
                    user_id: page_permission.user_id,
                    permission_type_id: page_permission.permission_type_id,
                };
                let response = Response {
                    msg: String::from("Success"),
                    data: Some(page_permission),
                    success: true,
                };
                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Query failed, Page Permission not found, user may not have permissions to view the page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to see Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn new_user_page_permission(
    data: web::Data<AppState>,
    session_user_id: i64,
    user_id: i64,
    page_id: i64,
    permission_type_id: i64,
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

        let get_permissions = sqlx::query!(
        "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        session_user_id, page_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
        if (!get_permissions.is_none()) {
            let user_permission = get_permissions.unwrap();
            if (user_id != session_user_id) {
                if (user.user_type_id <= 2
                    || (user.user_type_id == 3 && user_permission.permission_type_id == 1))
                {
                    let search_existing = sqlx::query!(
        "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
        user_id, page_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
                    if (search_existing.is_none()) {
                        let add_page_permission: sqlx::postgres::PgQueryResult = sqlx::query(
        "INSERT INTO page_permissions (user_id, page_id, permission_type_id) VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(page_id)
    .bind(permission_type_id)
    .execute(&pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
                        if (add_page_permission.rows_affected() == 0) {
                            let response = Response {
                                msg: String::from("Query failed, no new Page Permission created."),
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
                            msg: String::from("A Page Permission for this User Already Exists."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from(
                            "User does not have Permissions to Create Page Permissions.",
                        ),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("Page Permissions for you already Exist."),
                    data: None,
                    success: true,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            if (user.user_type_id <= 2) {
                let add_page_permission: sqlx::postgres::PgQueryResult = sqlx::query(
        "INSERT INTO page_permissions (user_id, page_id, permission_type_id) VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(page_id)
    .bind(permission_type_id)
    .execute(&pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e))?;
                if (add_page_permission.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, no new Page Permission created."),
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
                    msg: String::from("User does not have Permissions to Create Page Permissions."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn update_user_page_permission(
    data: web::Data<AppState>,
    id: i64,
    session_user_id: i64,
    user_id: i64,
    page_id: i64,
    permission_type_id: i64,
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
            let update_page_permission: sqlx::postgres::PgQueryResult =
                sqlx::query("UPDATE page_permissions SET permission_type_id = $1 WHERE id = $2")
                    .bind(permission_type_id)
                    .bind(id)
                    .execute(&pool)
                    .await
                    .map_err(|e| error::ErrorBadRequest(e))?;
            if (update_page_permission.rows_affected() == 0) {
                let response = Response {
                    msg: String::from(
                        "Query failed, no Page Permission updated, it may not exist.",
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
                let page_permission = get_page_permission.unwrap();
                if (page_permission.permission_type_id == 1) {
                    if (session_user_id != user_id) {
                        let update_page_permission: sqlx::postgres::PgQueryResult = sqlx::query(
                            "UPDATE page_permissions SET permission_type_id = $1 WHERE id = $2",
                        )
                        .bind(permission_type_id)
                        .bind(id)
                        .execute(&pool)
                        .await
                        .map_err(|e| error::ErrorBadRequest(e))?;
                        if (update_page_permission.rows_affected() == 0) {
                            let response = Response {
                                msg: String::from(
                                    "Query failed, no Page Permission updated, it may not exist.",
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
                            msg: String::from("User Cannot Update their own Page Permission."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from(
                            "User does not have Permission to Update Page Permissions.",
                        ),
                        data: None,
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have Permissions for this Page."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permissions to Update Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn delete_page_permission(
    data: web::Data<AppState>,
    id: i64,
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
    .map_err(|e| error::ErrorBadRequest(e))?;

    if (!get_user.is_none()) {
        let user = get_user.unwrap();
        if (user.user_type_id <= 2) {
            let get_count_page_permissions: i64 =
                sqlx::query_scalar("SELECT Count(*) FROM page_permissions WHERE page_id = $1")
                    .bind(page_id)
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| error::ErrorBadRequest(e))?;
            if (get_count_page_permissions > 1) {
                let result: sqlx::postgres::PgQueryResult =
                    sqlx::query("DELETE FROM page_permissions WHERE id = $1")
                        .bind(id)
                        .execute(&pool)
                        .await
                        .map_err(|e| error::ErrorBadRequest(e))?;
                if (result.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from(
                            "Query failed, no Page Permission Deleted, it may not Exist.",
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
                        "Cannot Delete Page Permission as there always must be at least one.",
                    ),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Ok().json(response))
            }
        } else if (user.user_type_id == 3) {
            let get_permissions = sqlx::query!(
                "SELECT id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id, page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e))?;
            if (!get_permissions.is_none()) {
                let permissions = get_permissions.unwrap();

                if (permissions.permission_type_id == 1) {
                    if (!(permissions.id == id)) {
                        let get_count_page_permissions: i64 = sqlx::query_scalar(
                            "SELECT Count(*) FROM page_permissions WHERE page_id = $1",
                        )
                        .bind(page_id)
                        .fetch_one(&pool)
                        .await
                        .map_err(|e| error::ErrorBadRequest(e))?;

                        if (get_count_page_permissions > 1) {
                            let result: sqlx::postgres::PgQueryResult =
                                sqlx::query("DELETE FROM page_permissions WHERE id = $1")
                                    .bind(id)
                                    .execute(&pool)
                                    .await
                                    .map_err(|e| error::ErrorBadRequest(e))?;

                            if (result.rows_affected() == 0) {
                                let response = Response {
                                msg: String::from(
                                    "Query failed, no Page Permission Deleted, it may not Exist.",
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
                                "Cannot Delete Page Permission as there always must be at least one.",),
                                data: None,
                                success: false,
                            };

                            Ok(HttpResponse::BadRequest().json(response))
                        }
                    } else {
                        let response = Response {
                            msg: String::from("User Cannot Delete their own Page Permission."),
                            data: None,
                            success: false,
                        };
                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from(
                            "User does not have Permissions to Delete Page Permissions.",
                        ),
                        data: None,
                        success: false,
                    };
                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("User does not have Permission to Delete Page Permissions."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have Permission to Delete Page Permissions."),
                data: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Cannot find User."),
            data: None,
            success: false,
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}
