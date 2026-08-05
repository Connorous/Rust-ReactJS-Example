use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

// Each user gets a broadcast sender — multiple tabs/devices can subscribe
pub type UserConnections = Arc<RwLock<HashMap<i64, broadcast::Sender<String>>>>;

pub struct AppState {
    pub db: PgPool,
    pub connections: UserConnections,
}

// Full user struct matching new schema - never sent to client directly
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub bio_info: Option<String>,
    pub user_type_id: i64,
    pub account_status_id: i64,
    pub status_id: Option<i64>,
    pub is_online: bool,
    pub theme_id: Option<i64>,
    pub theme_dark_mode: bool,

    // Light mode colours
    pub light_theme_primary_colour: String,
    pub light_theme_secondary_colour: String,
    pub light_theme_accent_colour: String,
    pub light_theme_sent_colour: String,
    pub light_theme_received_colour: String,
    pub light_theme_dark_text_colour: String,
    pub light_theme_light_text_colour: String,

    // Dark mode colours
    pub dark_theme_primary_colour: String,
    pub dark_theme_secondary_colour: String,
    pub dark_theme_accent_colour: String,
    pub dark_theme_sent_colour: String,
    pub dark_theme_received_colour: String,
    pub dark_theme_dark_text_colour: String,
    pub dark_theme_light_text_colour: String,

    // Refresh token
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token_created_at: Option<DateTime<Utc>>,
    pub refresh_token_updated_at: Option<DateTime<Utc>>,

    // Audit
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// WebSocket push helpers
pub async fn push_to_user(
    connections: &UserConnections,
    user_id: i64,
    event: &str,
    data: serde_json::Value,
) {
    let msg = serde_json::json!({
        "event": event,
        "data": data
    })
    .to_string();

    let conns = connections.read().await;
    if let Some(tx) = conns.get(&user_id) {
        let _ = tx.send(msg);
    }
}

pub async fn push_to_users(
    connections: &UserConnections,
    user_ids: Vec<i64>,
    event: &str,
    data: serde_json::Value,
) {
    let msg = serde_json::json!({
        "event": event,
        "data": data
    })
    .to_string();

    let conns = connections.read().await;
    for user_id in user_ids {
        if let Some(tx) = conns.get(&user_id) {
            let _ = tx.send(msg.clone());
        }
    }
}
