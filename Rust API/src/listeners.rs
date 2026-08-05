use crate::state::{push_to_user, push_to_users, UserConnections};
use sqlx::PgPool;

pub async fn pg_listener(pool: PgPool, connections: UserConnections) {
    let mut listener = sqlx::postgres::PgListener::connect_with(&pool)
        .await
        .expect("Failed to create PG listener");

    listener
        .listen_all(vec![
            "user_updated",
            "user_suspended",
            "message_created",
            "message_updated",
            "message_deleted",
            "group_permission_changed",
            "group_updated",
        ])
        .await
        .expect("Failed to subscribe to channels");

    log::info!("PG listener started");

    loop {
        match listener.recv().await {
            Ok(notification) => {
                let channel = notification.channel();
                let payload = notification.payload();

                log::info!("PG notify: channel={} payload={}", channel, payload);

                match channel {
                    "user_updated" => handle_user_updated(payload, &connections).await,
                    "user_suspended" => handle_user_suspended(payload, &connections).await,
                    "message_created" => handle_message_created(payload, &connections).await,
                    "message_updated" => handle_message_updated(payload, &connections).await,
                    "message_deleted" => handle_message_deleted(payload, &connections).await,
                    "group_permission_changed" => {
                        handle_group_permission_changed(payload, &connections).await
                    }
                    "group_updated" => handle_group_updated(payload, &connections).await,
                    _ => {}
                }
            }
            Err(e) => {
                log::error!("PG listener error: {}", e);
                // Small delay before reconnect attempt
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}

async fn handle_user_updated(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(user_id) = data["user_id"].as_i64() {
            push_to_user(connections, user_id, "user_updated", data).await;
        }
    }
}

async fn handle_user_suspended(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(user_id) = data["user_id"].as_i64() {
            // Force logout by pushing a special event
            push_to_user(
                connections,
                user_id,
                "force_logout",
                serde_json::json!({
                    "reason": "account_suspended"
                }),
            )
            .await;
        }
    }
}

async fn handle_message_created(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        // Could be DM or group message — payload includes recipient user_ids
        if let Some(user_ids) = data["recipient_ids"].as_array() {
            let ids: Vec<i64> = user_ids.iter().filter_map(|id| id.as_i64()).collect();
            push_to_users(connections, ids, "message_created", data).await;
        }
    }
}

async fn handle_message_updated(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(user_ids) = data["recipient_ids"].as_array() {
            let ids: Vec<i64> = user_ids.iter().filter_map(|id| id.as_i64()).collect();
            push_to_users(connections, ids, "message_updated", data).await;
        }
    }
}

async fn handle_message_deleted(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(user_ids) = data["recipient_ids"].as_array() {
            let ids: Vec<i64> = user_ids.iter().filter_map(|id| id.as_i64()).collect();
            push_to_users(connections, ids, "message_deleted", data).await;
        }
    }
}

async fn handle_group_permission_changed(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(user_id) = data["user_id"].as_i64() {
            push_to_user(connections, user_id, "group_permission_changed", data).await;
        }
    }
}

async fn handle_group_updated(payload: &str, connections: &UserConnections) {
    if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
        if let Some(user_ids) = data["member_ids"].as_array() {
            let ids: Vec<i64> = user_ids.iter().filter_map(|id| id.as_i64()).collect();
            push_to_users(connections, ids, "group_updated", data).await;
        }
    }
}
