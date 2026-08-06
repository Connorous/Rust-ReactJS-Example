use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, ResponseError};
use log::info;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
mod auth;
mod controllers;
mod extractors;
mod listeners;
mod routes;
mod state;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use state::UserConnections;
use std::env;

// ApiError covers all error cases the API can return
#[derive(Debug)]
enum ApiError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

// Display formats the error for logging
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

// ResponseError converts ApiError into an HTTP response
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({"error": msg}))
            }
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
            }
            ApiError::Internal(msg) => {
                log::error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({"error": "An internal error occurred"}))
            }
        }
    }
}

// Configure CORS for the application
fn configure_cors() -> Cors {
    let frontend_url = env::var("FRONTEND_URL").unwrap();

    Cors::default()
        .allowed_origin(&frontend_url)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec!["Content-Type", "Authorization"])
        .supports_credentials() // required for httpOnly cookies
        .max_age(3600)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger from the RUST_LOG environment variable
    env_logger::init();
    // Create shared application state
    dotenv().ok();

    let pg_user = env::var("PgUser").expect("PgUser must be set");
    let pg_password = env::var("PgPassword").expect("PgPassword must be set");
    let pg_ip = env::var("PgIp").expect("PgIp must be set");
    let pg_port = env::var("PgPort").expect("PgPort must be set");
    let pg_database = env::var("PgDatabase").expect("PgDatabase must be set");

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        pg_user, pg_password, pg_ip, pg_port, pg_database
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let connections: UserConnections = Arc::new(RwLock::new(HashMap::new()));

    let data = web::Data::new(state::AppState {
        db: pool,
        connections,
    });

    // Spawn PG LISTEN/NOTIFY listener as background task
    let listener_pool = pool.clone();
    let listener_connections = connections.clone();
    tokio::spawn(async move {
        listeners::pg_listener(listener_pool, listener_connections).await;
    });

    info!("Starting server on 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .wrap(configure_cors())
            .configure(routes::configure_login_routes)
            .configure(routes::configure_user_routes)
            .configure(routes::configure_chat_group_routes)
            .configure(routes::configure_direct_message_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
