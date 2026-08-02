use crate::state::AppState;
use actix_web::{error, web, HttpResponse};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Page {
    id: i64,
    date_created: NaiveDate,
    created_by_id: i64,
    published: bool,
    title: String,
}

#[derive(Debug, Clone, Serialize)]
struct FullPage {
    id: i64,
    date_created: NaiveDate,
    created_by_id: i64,
    published: bool,
    title: String,
    selected_css_id: Option<i64>
}

#[derive(Debug, Clone, Serialize)]
struct Username {
    id: i64,
    username: String,
}

#[derive(Debug, Clone, Serialize)]
struct PageToDelete {
    id: i64,
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    data: Option<Page>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseFull {
    msg: String,
    data: Option<FullPage>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct NewPageResponse {
    msg: String,
    data: i64,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseList {
    msg: String,
    data: Option<Vec<Page>>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct UsernameResponseList {
    msg: String,
    data: Option<Vec<Username>>,
    success: bool,
}

pub async fn list_pages_usermade(
    data: web::Data<AppState>,
    session_user_id: i64,
    pages_user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
       
    if (!get_user.is_none()) {
        let user_type = get_user.unwrap().user_type_id;
        if (user_type <= 2){
            let pages: Vec<Page> = sqlx::query_as!(
                Page,
                "SELECT id, date_created, created_by_id, published, title FROM pages ORDER BY date_created DESC"
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
                
            if (!pages.is_empty()) {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(pages),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = ResponseList {
                        msg: String::from("No Pages found, None Published may Exist that match the User who Created them."),
                        data: Some(pages),
                        success: true,
                    };

                Ok(HttpResponse::Ok().json(response))
            }
        } else if (user_type <= 4) {
     
                let pages = sqlx::query_as!(Page,
                "SELECT id, date_created, created_by_id, published, title FROM pages WHERE (id IN (SELECT page_id FROM page_permissions WHERE user_id = $1 AND permission_type_id <= 4) AND published = true) OR (id IN (SELECT page_id FROM page_permissions WHERE user_id = $1 AND permission_type_id <= 2)) ORDER BY date_created DESC", session_user_id)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
            

            if (!pages.is_empty()) {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(pages),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = ResponseList {
                        msg: String::from("No Pages found, None Published may Exist that match the Users Permission or the User who Created them."),
                        data: Some(pages),
                        success: true,
                    };

                Ok(HttpResponse::Ok().json(response))
            }
        } else {
            let response = ResponseList {
                msg: String::from("Noob User does not have Permission to View Pages."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Session User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn list_all_pages(
    data: web::Data<AppState>,
    session_user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

    if (!get_user.is_none()) {
        let user_type = get_user.unwrap().user_type_id;
        if (user_type <= 2) {
            let pages: Vec<Page> = sqlx::query_as!(
                Page,
                "SELECT id, date_created, created_by_id, published, title FROM pages ORDER BY date_created DESC"
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!pages.is_empty()) {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(pages),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = ResponseList {
                        msg: String::from("No Pages found, None may Exist"),
                        data: Some(pages),
                        success: true,
                    };

                Ok(HttpResponse::Ok().json(response))
            }
        } else if (user_type <= 4) {
            let pages: Vec<Page> = sqlx::query_as!(Page,
                "SELECT id, date_created, created_by_id, published, title FROM pages WHERE (id IN (SELECT page_id FROM page_permissions WHERE user_id = $1 AND permission_type_id <= 4) AND published = true) OR (id IN (SELECT page_id FROM page_permissions WHERE user_id = $1 AND permission_type_id <= 2)) ORDER BY date_created DESC", session_user_id)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!pages.is_empty()) {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(pages),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = ResponseList {
                        msg: String::from("No Pages found, None Published may Exist that match the Users Permissions."),
                        data: Some(pages),
                        success: true,
                    };

                Ok(HttpResponse::Ok().json(response))
            }
        } else {
            let response = ResponseList {
                msg: String::from("Noob User does not have Permission to View Pages."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Session User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}


pub async fn list_all_page_creators(
    data: web::Data<AppState>,
    session_user_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

    if (!get_user.is_none()) {
        let user_type = get_user.unwrap().user_type_id;
        if (user_type <= 2) {
            let usernames:Vec<Username> = sqlx::query_as!(
                Username, "SELECT id, username FROM users WHERE id IN (SELECT created_by_id FROM pages)"
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!usernames.is_empty()) {
                let response = UsernameResponseList {
                    msg: String::from("Success"),
                    data: Some(usernames),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = UsernameResponseList {
                        msg: String::from("No Page Usernames found, None may Exist"),
                        data: Some(usernames),
                        success: true,
                    };

                Ok(HttpResponse::Ok().json(response))
            }
        } else if (user_type <= 4) {
            let usernames: Vec<Username> = sqlx::query_as!(Username,
                "SELECT id, username FROM users WHERE id IN (SELECT created_by_id FROM pages WHERE (id IN (SELECT page_id FROM page_permissions WHERE user_id = $1 AND permission_type_id <= 4) AND published = true OR id IN (SELECT page_id FROM page_permissions WHERE user_id = $1 AND permission_type_id <= 2)))", session_user_id)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!usernames.is_empty()) {
                let response = UsernameResponseList {
                    msg: String::from("Success"),
                    data: Some(usernames),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = UsernameResponseList {
                        msg: String::from("No Page Usernames found, None Published may Exist that match the Users Permissions."),
                        data: Some(usernames),
                        success: true,
                    };

                Ok(HttpResponse::Ok().json(response))
            }
        } else {
            let response = ResponseList {
                msg: String::from("Noob User does not have Permission to View Pages."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Session User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}


//next on the list
pub async fn get_page(
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
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

    if (!get_user.is_none()) {
        let user_type = get_user.unwrap().user_type_id;
        if (user_type <= 2) {
            let page =
                sqlx::query_as!(FullPage,
                "SELECT id, date_created, created_by_id, published, title, selected_css_id FROM pages WHERE id = $1",
                page_id
            )
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!page.is_none()) {
                let response = ResponseFull {
                    msg: String::from("Success"),
                    data: page,
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = Response {
                    msg: String::from("Query failed, Page not found"),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } else if (user_type <= 4) {
            let page_permission =
                sqlx::query!("SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id, page_id
            )
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!page_permission.is_none()) {
                if (page_permission.unwrap().permission_type_id <= 4) {
                    let page = sqlx::query_as!(FullPage,
                "SELECT id, date_created, created_by_id, published, title, selected_css_id FROM pages WHERE id = $1",
                page_id
            )
                    .fetch_optional(&pool)
                    .await
                    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

                    if (!page.is_none()) {
                        let response = ResponseFull {
                            msg: String::from("Success"),
                            data: page,
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response))
                    } else {
                        let response = Response {
                            msg: String::from("Query failed, Page not found"),
                            data: None,
                            success: false,
                        };

                        Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = ResponseList {
                        msg: String::from("User does not have Permission to View Page"),
                        data: None,
                        success: false,
                    };

                    Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = ResponseList {
                    msg: String::from("User does not have Permission to View Page"),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = ResponseList {
                msg: String::from("Noob User does not have Permission to View Pages."),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn new_page(
    data: web::Data<AppState>,
    created_by_id: i64,
    title: String,
) -> Result<HttpResponse, actix_web::Error> {
    if title.trim() == "" {
        let response = Response {
            msg: String::from("Page Title must not be empty!"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
    else {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();
    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        created_by_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
    if (!get_user.is_none()) {
        let user_type_id = get_user.unwrap().user_type_id;
        if (user_type_id <= 3) {
            let create_page = sqlx::query!(
                "INSERT INTO pages (created_by_id, title, published) VALUES ($1, $2, false) RETURNING id",
                created_by_id,
                title
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
            if (!create_page.is_none()) {
                let page_id = create_page.unwrap().id;
                let create_page_permissions = sqlx::query!("INSERT INTO page_permissions (user_id, page_id, permission_type_id) VALUES ($1, $2, $3)", created_by_id, page_id, 1)
            .execute(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
                if (create_page_permissions.rows_affected() > 0) {
                    let create_page_css = sqlx::query!("INSERT INTO page_css (page_id, sheet_name, css, created_by_id) VALUES ($1, $2, $3, $4) RETURNING id", page_id, "Default", "", created_by_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

                    if (!create_page_css.is_none()) {
                        let page_css_id = create_page_css.unwrap().id;

                        let update_page_with_css = sqlx::query!(
                            "UPDATE pages SET selected_css_id = $1 WHERE id = $2",
                            page_css_id,
                            page_id
                        )
                        .execute(&pool)
                        .await
                        .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

                        if (!update_page_with_css.rows_affected() > 0) {
                            let response = NewPageResponse {
                                msg: String::from("Success"),
                                data: page_id,
                                success: true,
                            };
                            Ok(HttpResponse::Ok().json(response))
                        } else {
                            let response = Response {
                                msg: String::from("Query failed, CSS not assigned to new Page."),
                                data: None,
                                success: false,
                            };
                             Ok(HttpResponse::BadRequest().json(response))
                        }
                    } else {
                        let response = Response {
                            msg: String::from("Query failed, no new Page CSS created."),
                            data: None,
                            success: false,
                        };
                         Ok(HttpResponse::BadRequest().json(response))
                    }
                } else {
                    let response = Response {
                        msg: String::from("Query failed, no new Page Permissions created."),
                        data: None,
                        success: false,
                    };
                     Ok(HttpResponse::BadRequest().json(response))
                }
            } else {
                let response = Response {
                    msg: String::from("Query failed, no new Page created."),
                    data: None,
                    success: false,
                };
                 Ok(HttpResponse::BadRequest().json(response))
            }
        } else {
            let response = Response {
                msg: String::from("User does not have permission to create Pages."),
                data: None,
                success: false,
            };
             Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, could not find Page creator User."),
            data: None,
            success: false,
        };
         Ok(HttpResponse::BadRequest().json(response))
    }
}
}


pub async fn update_page(
    data: web::Data<AppState>,
    session_user_id: i64,
    page_id: i64,
    published: bool,
    title: String,
    selected_css_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

    if (!get_user.is_none()) {
        let user_type_id = get_user.unwrap().user_type_id;
        if (user_type_id <= 2) {
            let update_page = sqlx::query!(
                "UPDATE pages SET published = $1, title = $2, selected_css_id = $3 WHERE id = $4",
                published,
                title,
                selected_css_id, 
                page_id
            )
            .execute(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;


            if (update_page.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("Success"),
                    data: None,
                    success: true,
                };
                 Ok(HttpResponse::Ok().json(response))
            }
            else {
                let response = Response {
                    msg: String::from("Query Failed, Page was not Updated."),
                    data: None,
                    success: true,
                };
                 Ok(HttpResponse::BadRequest().json(response))
            }

        }
        else if (user_type_id == 3) {
            let page_permission = sqlx::query!(
                "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id,
                page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

        if (!page_permission.is_none()) {
            if (page_permission.unwrap().permission_type_id <= 2) {
                let update_page = sqlx::query!(
                "UPDATE pages SET published = $1, title = $2, selected_css_id = $3 WHERE id = $4",
                published,
                title,
                selected_css_id, 
                page_id
            )
            .execute(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (update_page.rows_affected() > 0) {
                let response = Response {
                        msg: String::from("Success"),
                        data: None,
                        success: true,
                    };
                 Ok(HttpResponse::Ok().json(response))
            }
            else {
                let response = Response {
                    msg: String::from("Query Failed, Page was not"),
                    data: None,
                    success: true,
                };
                 Ok(HttpResponse::BadRequest().json(response))
            }
            }
            else {
                let response = Response {
                    msg: String::from("User does not have Permission to Update Page."),
                    data: None,
                    success: false,
                };
                 Ok(HttpResponse::BadRequest().json(response))
            }
        }
        else {
            let response = Response {
                msg: String::from("User does not have Permission to Update Page, No Permissions found."),
                data: None,
                success: false,
            };
             Ok(HttpResponse::BadRequest().json(response))
        }
        }
        else {
            let response = Response {
                msg: String::from("User is not allowed to Update Pages."),
                data: None,
                success: false,
            };
             Ok(HttpResponse::BadRequest().json(response))
        }

    } else {
        let response = Response {
            msg: String::from("Query failed, could not find Page creator User."),
            data: None,
            success: false,
        };
         Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn delete_page(
    data: web::Data<AppState>,
    session_user_id: i64, page_id: i64
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_user = sqlx::query!(
        "SELECT id, user_type_id FROM users WHERE id = $1",
        session_user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

    if (!get_user.is_none()) {
        let user_type_id = get_user.unwrap().user_type_id;
        if (user_type_id <= 2) {
            let mut tx = pool.begin().await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let delete_page_elements: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_elements WHERE page_id = $1").bind(page_id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let update_page_remove_css = sqlx::query!(
                            "UPDATE pages SET selected_css_id = $1 WHERE id = $2",
                            None::<i64>,
                            page_id
                        )
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            let delete_page_css: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_css WHERE page_id = $1").bind(page_id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(serde_json::json!({ "error": e.to_string() }).to_string())
})?;

            let delete_page_permissions: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_permissions WHERE page_id = $1").bind(page_id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let delete_page: sqlx::postgres::PgQueryResult =
                sqlx::query("DELETE FROM pages WHERE id = $1")
                    .bind(page_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

            tx.commit().await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            if (delete_page.rows_affected() > 0) {
                let response = Response {
                    msg: String::from("Success"),
                    data: None,
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response)) 
            }
            else {
                let response = Response {
                    msg: String::from("Query Failed, Page was not deleted"),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }

            }
        else if (user_type_id == 3) {
            let page_permission = sqlx::query!(
                "SELECT id, user_id, page_id, permission_type_id FROM page_permissions WHERE user_id = $1 AND page_id = $2",
                session_user_id,
                page_id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            if (!page_permission.is_none()) {
                if (page_permission.unwrap().permission_type_id <= 1) {

                    let mut tx = pool.begin().await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let delete_page_elements: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_elements WHERE page_id = $1").bind(page_id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let update_page_remove_css = sqlx::query!(
                            "UPDATE pages SET selected_css_id = $1 WHERE id = $2",
                            None::<i64>,
                            page_id
                        )
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

            let delete_page_css: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_css WHERE page_id = $1").bind(page_id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(serde_json::json!({ "error": e.to_string() }).to_string())
})?;

            let delete_page_permissions: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_permissions WHERE page_id = $1").bind(page_id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

            let delete_page: sqlx::postgres::PgQueryResult =
                sqlx::query("DELETE FROM pages WHERE id = $1")
                    .bind(page_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

            tx.commit().await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                    if (delete_page.rows_affected() > 0) {
                        let response = Response {
                            msg: String::from("Success"),
                            data: None,
                            success: true,
                        };

                        Ok(HttpResponse::Ok().json(response)) 
                    }
                    else {
                        let response = Response {
                            msg: String::from("Query Failed, Page was not Deleted"),
                            data: None,
                            success: false,
                        };

                        Ok(HttpResponse::BadRequest().json(response))
                    }
                }
                else {
                    let response = ResponseList {
                        msg: String::from("User is not Page Owner, so they cannot Delete the Page"),
                        data: None,
                        success: false,
                    };

                 Ok(HttpResponse::BadRequest().json(response))
                }
            }
            else {
                let response = ResponseList {
                    msg: String::from("User does not have any Page Permissions"),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::BadRequest().json(response))
            }
        } 
        else {
            let response = ResponseList {
                msg: String::from("User does not have Permission to Delete Pages"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response)) 
        }
    }
    else {
        let response = ResponseList {
            msg: String::from("Query Failed, User not found, it may not Exist."),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response)) 
    }
}