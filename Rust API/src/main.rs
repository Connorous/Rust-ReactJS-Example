use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, ResponseError};
use log::info;
use std::fmt;
mod routes;
mod state;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod auth;
mod controllers;

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
    Cors::default()
        // Allow requests from your frontend origin
        .allowed_origin("https://page-creator-frontend.fly.dev")
        // Allow common HTTP methods
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        // Allow JSON content type header
        .allowed_headers(vec!["Content-Type", "Authorization"])
        // Cache preflight responses for one hour
        .max_age(3600)
}

#[allow(non_snake_case)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger from the RUST_LOG environment variable
    env_logger::init();
    // Create shared application state

    dotenv::dotenv().ok();

    let PgUser = env::var("PgUser").unwrap();
    let PgPassword = std::env::var("PgPassword").unwrap();
    let PgIp = std::env::var("PgIp").unwrap();
    let PgPort = std::env::var("PgPort").unwrap();
    let PgDatabase = std::env::var("PgDatabase").unwrap();

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        PgUser, PgPassword, PgIp, PgPort, PgDatabase
    );
    //let database_connection_url: String = (database_url).expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let data = web::Data::new(state::AppState { db: pool });

    info!("Starting server on 0.0.0.0");
    // Build and start the HTTP server
    HttpServer::new(move || {
        App::new()
            // Attach shared state to the application
            // Add logging middleware for every request
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            //configure routes
            .configure(routes::configure_login_user_routes)
            .configure(routes::configure_user_routes)
            .configure(routes::configure_page_routes)
            .configure(routes::configure_page_element_routes)
            .configure(routes::configure_page_permission_routes)
            .configure(routes::configure_page_css_routes)
            .wrap(configure_cors())
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
