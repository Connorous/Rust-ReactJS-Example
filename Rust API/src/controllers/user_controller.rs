use crate::auth::{generate_token, hash_password, verify_password};
use crate::state::AppState;
use crate::state::User;
use actix_web::{error, web, HttpResponse};
use serde::Serialize;
use sqlx::postgres::PgQueryResult;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct QueryUsername {
    username: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct QueryEmail {
    email: String,
}

#[derive(Debug, Clone, Serialize)]
struct LoggedInUser {
    id: i64,
    username: String,
    name: String,
    email: String,
    user_type_id: i64,
}

#[derive(Debug, Clone, Serialize)]
struct LoginResponse {
    msg: String,
    token: Option<String>,
    user: Option<LoggedInUser>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct Response {
    msg: String,
    data: Option<User>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseList {
    msg: String,
    data: Option<Vec<User>>,
    success: bool,
}

#[derive(Debug, Clone, Serialize)]
struct UserType {
    id: i64,
    r#type: String,
}

#[derive(Debug, Clone, Serialize)]
struct ResponseTypeList {
    msg: String,
    data: Option<Vec<UserType>>,
    success: bool,
}

pub async fn register_user(
    data: web::Data<AppState>,
    username: String,
    email: String,
    name: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if username.trim() == "" || password.trim() == "" || email.trim() == "" || name.trim() == "" {
        let response = LoginResponse {
            msg: String::from("Username, Email, Name or Password must not be empty!"),
            token: None,
            user: None,
            success: false,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        if (password.to_lowercase().contains("connor")) {
            let response = LoginResponse {
                msg: String::from("Registration Failed, Password cannot contain 'Connor'."),
                token: None,
                user: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        } else {
            let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();
            let username_rows: Vec<QueryUsername> =
                sqlx::query_as("SELECT username FROM users WHERE username = $1")
                    .bind(&username)
                    .fetch_all(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;
            let email_rows: Vec<QueryEmail> =
                sqlx::query_as("SELECT email FROM users WHERE email = $1")
                    .bind(&email)
                    .fetch_all(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

            if username_rows.len() == 0 && email_rows.len() == 0 {
                let hashed_password = hash_password(&password);
                let result = sqlx::query(
                "INSERT INTO users (username, email, name, password, user_type_id) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(username)
            .bind(email)
            .bind(name)
            .bind(&hashed_password)
            .bind(4)
            .execute(&pool)
            .await
            .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;

                let response = LoginResponse {
                    msg: String::from("Success"),
                    token: None,
                    user: None,
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                let response = LoginResponse {
                    msg: String::from(
                        "Registration Failed, Username or Email may Already be in use.",
                    ),
                    token: None,
                    user: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    }
}

pub async fn login_user(
    data: web::Data<AppState>,
    username: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if username.trim() == "" || password.trim() == "" {
        Ok(HttpResponse::BadRequest().body("Username or Password must not be empty!"))
    } else {
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();
        let rows: Vec<User> =
            sqlx::query_as("SELECT id, username, email, name, password, date_created, user_type_id FROM users WHERE username = $1")
                .bind(&username)
                .fetch_all(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),)})?;
        if rows.len() == 0 {
            let response = LoginResponse {
                msg: String::from(
                    "Login Failed, Username may be Incorrect, as User does not Exist.",
                ),
                token: None,
                user: None,
                success: false,
            };
            Ok(HttpResponse::BadRequest().json(response))
        } else {
            match verify_password(&password, &rows[0].password) {
                Ok(_) => {
                    let token = generate_token(rows[0].username.clone());
                    let user = rows[0].clone();

                    let logged_in_user = LoggedInUser {
                        id: user.id,
                        username: user.username,
                        name: user.name,
                        email: user.email,
                        user_type_id: user.user_type_id,
                    };
                    let response = LoginResponse {
                        msg: String::from("Success"),
                        token: Some(token),
                        user: Some(logged_in_user),
                        success: true,
                    };
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(_) => {
                    let response = LoginResponse {
                        msg: String::from("Password is Uncorrect!"),
                        token: None,
                        user: None,
                        success: false,
                    };
                    Ok(HttpResponse::Unauthorized().json(response))
                }
            }
        }
    }
}

pub async fn list_users(
    data: web::Data<AppState>,
    id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_admin_user = sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;
    if (!get_admin_user.is_none()) {
        let actual = get_admin_user.unwrap();
        if (actual.user_type_id > 2) {
            let response = Response {
                msg: String::from("User is Unauthorized to Manage Users"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Unauthorized().json(response))
        } else {
            let users: Vec<User> = sqlx::query_as!(
        User,
        "SELECT id, date_created, username, email, password, name, user_type_id FROM users ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

            if (users.is_empty()) {
                let response = Response {
                    msg: String::from("Query failed, no Users."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseList {
                    msg: String::from("Success"),
                    data: Some(users),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            }
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Admin User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn list_user_types(
    data: web::Data<AppState>,
    id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_admin_user = sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;
    if (!get_admin_user.is_none()) {
        let actual = get_admin_user.unwrap();
        if (actual.user_type_id > 2) {
            let response = Response {
                msg: String::from("User is Unauthorized to Manage Users"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::Unauthorized().json(response))
        } else {
            let user_types: Vec<UserType> =
                sqlx::query_as!(UserType, "SELECT id, type FROM user_types ORDER BY id")
                    .fetch_all(&pool)
                    .await
                    .map_err(|e| {
                        error::ErrorBadRequest(
                            serde_json::to_string(&e.to_string())
                                .unwrap_or_else(|_| "{}".to_string()),
                        )
                    })?;

            if (user_types.is_empty()) {
                let response = Response {
                    msg: String::from("Query failed, no Users."),
                    data: None,
                    success: false,
                };
                Ok(HttpResponse::BadRequest().json(response))
            } else {
                let response = ResponseTypeList {
                    msg: String::from("Success"),
                    data: Some(user_types),
                    success: true,
                };

                Ok(HttpResponse::Ok().json(response))
            }
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Admin User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}

/*pub async fn get_user(
    data: web::Data<AppState>,
    id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let result = sqlx::query!(
        "SELECT id, date_created, username, email, name, user_type_id FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;

    if (!result.is_none()) {
        let actual = result.unwrap();

        let user = User {
            id: actual.id,
            date_created: actual.date_created,
            username: actual.username,
            email: actual.email,
            password: String::from(""),
            name: actual.name,
            user_type_id: actual.user_type_id,
        };

        let response = Response {
            msg: String::from("Success"),
            data: Some(user),
            success: true,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            msg: String::from("Query failed, User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}*/

pub async fn new_user(
    data: web::Data<AppState>,
    admin_id: i64,
    username: String,
    email: String,
    name: String,
    password: String,
    user_type_id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    if username.trim() == "" || name.trim() == "" || email.trim() == "" || password.trim() == "" {
        let response = Response {
            msg: String::from("Username, Email, Name or Password must not be empty!"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

        let get_admin_user =
            sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", admin_id)
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;
        if (!get_admin_user.is_none()) {
            let actual_admin_user = get_admin_user.unwrap();
            if (actual_admin_user.user_type_id > 2 || actual_admin_user.user_type_id > user_type_id)
            {
                let response = Response {
                    msg: String::from("User is Unauthorized to Create this User."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Unauthorized().json(response))
            } else {
                let result: sqlx::postgres::PgQueryResult = sqlx::query(
        "INSERT INTO users (username, email, name, password, user_type_id) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(username)
    .bind(email)
    .bind(name)
    .bind(password)
    .bind(user_type_id)
    .execute(&pool)
    .await
    .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                if (result.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, no new User created."),
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
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, Admin User not found"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

pub async fn update_user(
    data: web::Data<AppState>,
    admin_id: i64,
    id: i64,
    username: String,
    email: String,
    name: String,
    user_type_id: i64,
    original_user_type: i64,
) -> Result<HttpResponse, actix_web::Error> {
    if username.trim() == "" || name.trim() == "" || email.trim() == "" {
        let response = Response {
            msg: String::from("Username, Email or Name  must not be empty!"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();
        let get_admin_user =
            sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", admin_id)
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;
        if (!get_admin_user.is_none()) {
            let actual_admin_user = get_admin_user.unwrap();
            if (actual_admin_user.user_type_id > 2) {
                let response = Response {
                    msg: String::from("User is Unauthorized to Edit this User."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Unauthorized().json(response))
            } else if (actual_admin_user.user_type_id > user_type_id) {
                let response = Response {
                    msg: String::from("User is Not Authorized to Update an Existing User to an Admin Type higher than their own."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Unauthorized().json(response))
            } else if (actual_admin_user.user_type_id > original_user_type) {
                let response = Response {
                    msg: String::from("User is Not Authorized to Update a User with an Admin Type Greater than their own."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Unauthorized().json(response))
            } else {
                let result: PgQueryResult = sqlx::query(
        "UPDATE users SET username = $1, email = $2, name = $3, user_type_id = $4 WHERE id = $5",
    )
    .bind(username)
    .bind(email)
    .bind(name)
    .bind(user_type_id)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        error::ErrorBadRequest(
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
        )
    })?;
                if (result.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, User was not Updated, they may not Exist"),
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
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, Admin User not found"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

pub async fn reset_user_password(
    data: web::Data<AppState>,
    username: String,
    email: String,
    password: String,
) -> Result<HttpResponse, actix_web::Error> {
    if email.trim() == "" || username.trim() == "" || password.trim() == "" {
        let response = Response {
            msg: String::from("Email, Username or Password must not be empty!"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    } else {
        let hashed_password = hash_password(&password);
        let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();
        let get_user = sqlx::query!(
            "SELECT id, email, username FROM users WHERE email = $1 AND username = $2",
            email,
            username
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;
        if (!get_user.is_none()) {
            let actual_user = get_user.unwrap();

            let result: PgQueryResult = sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
                .bind(hashed_password)
                .bind(actual_user.id)
                .execute(&pool)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;
            if (result.rows_affected() == 0) {
                let response = Response {
                    msg: String::from("Query failed, Password was not Updated, User may not Exist"),
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
                msg: String::from("Query failed, User not found, Check Email and Username"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

pub async fn delete_user(
    data: web::Data<AppState>,
    admin_id: i64,
    id: i64,
) -> Result<HttpResponse, actix_web::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = data.db.to_owned();

    let get_admin_user = sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", admin_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            error::ErrorBadRequest(
                serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
            )
        })?;

    if (!get_admin_user.is_none()) {
        let get_user = sqlx::query!("SELECT id, user_type_id FROM users WHERE id = $1", id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;
        if (!get_user.is_none()) {
            let actual_admin_user = get_admin_user.unwrap();
            let actual_user = get_user.unwrap();

            if (actual_admin_user.user_type_id == 1) {
                let mut tx = pool.begin().await.map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                let delete_users_page_elements: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_elements WHERE page_id IN (SELECT id FROM pages WHERE created_by_id = $1)").bind(id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                let update_page_remove_css = sqlx::query!(
                    "UPDATE pages SET selected_css_id = $1 WHERE created_by_id = $2",
                    None::<i64>,
                    id
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                let delete_users_page_css: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_css WHERE page_id IN (SELECT id FROM pages WHERE created_by_id = $1)").bind(id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                let delete_users_page_permissions: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_permissions WHERE page_id IN (SELECT id FROM pages WHERE created_by_id = $1)").bind(id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                let delete_users_pages: sqlx::postgres::PgQueryResult =
                    sqlx::query("DELETE FROM pages WHERE created_by_id = $1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| {
                            error::ErrorBadRequest(
                                serde_json::to_string(&e.to_string())
                                    .unwrap_or_else(|_| "{}".to_string()),
                            )
                        })?;

                let delete_user: sqlx::postgres::PgQueryResult =
                    sqlx::query("DELETE FROM users WHERE id = $1")
                        .bind(id)
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

                if (delete_user.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, no User Deleted, they may not Exist."),
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
            } else if (actual_admin_user.user_type_id == 2
                && actual_admin_user.id == actual_user.id)
            {
                let response = Response {
                    msg: String::from("User Cannot Delete Themselves, Unless of Course they are a Supereme Administrator, which they are Not, so Stop asking."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Unauthorized().json(response))
            } else if (actual_admin_user.user_type_id > 2
                || actual_admin_user.user_type_id > actual_user.user_type_id)
            {
                let response = Response {
                    msg: String::from("User is Unauthorized to Delete this User."),
                    data: None,
                    success: false,
                };

                Ok(HttpResponse::Unauthorized().json(response))
            } else {
                let mut tx = pool.begin().await.map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                let delete_users_page_elements: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_elements WHERE page_id IN (SELECT id FROM pages WHERE created_by_id = $1)").bind(id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                let update_page_remove_css = sqlx::query!(
                    "UPDATE pages SET selected_css_id = $1 WHERE created_by_id = $2",
                    None::<i64>,
                    id
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    error::ErrorBadRequest(
                        serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                    )
                })?;

                let delete_users_page_css: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_css WHERE page_id IN (SELECT id FROM pages WHERE created_by_id = $1)").bind(id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                let delete_users_page_permissions: sqlx::postgres::PgQueryResult = sqlx::query("DELETE FROM page_permissions WHERE page_id IN (SELECT id FROM pages WHERE created_by_id = $1)").bind(id).execute(&mut *tx).await.map_err(|e| {
                error::ErrorBadRequest(
                    serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "{}".to_string()),
                )
            })?;

                let delete_users_pages: sqlx::postgres::PgQueryResult =
                    sqlx::query("DELETE FROM pages WHERE created_by_id = $1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| {
                            error::ErrorBadRequest(
                                serde_json::to_string(&e.to_string())
                                    .unwrap_or_else(|_| "{}".to_string()),
                            )
                        })?;

                let delete_user: sqlx::postgres::PgQueryResult =
                    sqlx::query("DELETE FROM users WHERE id = $1")
                        .bind(id)
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

                if (delete_user.rows_affected() == 0) {
                    let response = Response {
                        msg: String::from("Query failed, no User Deleted, they may not Exist."),
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
            }
        } else {
            let response = Response {
                msg: String::from("Query failed, User to Delete not found"),
                data: None,
                success: false,
            };

            Ok(HttpResponse::BadRequest().json(response))
        }
    } else {
        let response = Response {
            msg: String::from("Query failed, Admin User not found"),
            data: None,
            success: false,
        };

        Ok(HttpResponse::BadRequest().json(response))
    }
}
