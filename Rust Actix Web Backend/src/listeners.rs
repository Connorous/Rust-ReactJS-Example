use crate::state::{push_to_user, push_to_users, UserConnections};
use sqlx::PgPool;

pub async fn pg_listener(pool: PgPool, connections: UserConnections) {
    let mut listener = sqlx::postgres::PgListener::connect_with(&pool)
        .await
        .expect("Failed to create PG listener");

    listener
        .listen_all(vec![
            "message_created",
            "message_updated",
            "message_deleted",
            "user_created",
            "user_updated",
            "user_suspended",
            "user_deleted",
            "group_updated",
            "group_deleted",
            "group_permission_added",
            "group_permission_updated",
            "group_permission_deleted",
            "relationship_added",
            "relationship_updated",
            "relationship_deleted",
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
                    "message_created" => handle_message_created(payload, &connections).await,
                    "message_updated" => handle_message_updated(payload, &connections).await,
                    "message_deleted" => handle_message_deleted(payload, &connections).await,
                    "user_created" => handle_user_created(payload, &connections).await,
                    "user_updated" => handle_user_updated(payload, &connections).await,
                    "user_suspended" => handle_user_suspended(payload, &connections).await,
                    "user_deleted" => handle_user_deleted(payload, &connections).await,
                    "group_updated" => handle_group_updated(payload, &connections).await,
                    "group_deleted" => handle_group_deleted(payload, &connections).await,
                    "group_permission_added" => {
                        handle_group_permission_added(payload, &connections).await
                    }
                    "group_permission_updated" => {
                        handle_group_permission_updated(payload, &connections).await
                    }
                    "group_permission_deleted" => {
                        handle_group_permission_deleted(payload, &connections).await
                    }
                    "relationship_added" => handle_relationship_added(payload, &connections).await,
                    "relationship_updated" => {
                        handle_relationship_updated(payload, &connections).await
                    }
                    "relationship_deleted" => {
                        handle_relationship_deleted(payload, &connections).await
                    }
                    _ => {}
                }
            }
            Err(e) => {
                log::error!("PG listener error: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}

// --- MESSAGES ---

// Push full message object → frontend appends to list
async fn handle_message_created(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["recipient_ids"].as_array() {
            Some(ids) => {
                let recipient_ids: Vec<i64> = ids.iter().filter_map(|id| id.as_i64()).collect();
                push_to_users(connections, recipient_ids, "message_created", data).await;
            }
            None => {
                log::error!("message_created payload missing recipient_ids");
            }
        },
        Err(e) => {
            log::error!("Failed to parse message_created payload: {}", e);
        }
    }
}

// Push refresh signal → frontend re-fetches conversation
async fn handle_message_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["recipient_ids"].as_array() {
            Some(ids) => {
                let recipient_ids: Vec<i64> = ids.iter().filter_map(|id| id.as_i64()).collect();
                push_to_users(connections, recipient_ids, "messages_refresh", data).await;
            }
            None => {
                log::error!("message_updated payload missing recipient_ids");
            }
        },
        Err(e) => {
            log::error!("Failed to parse message_updated payload: {}", e);
        }
    }
}

// Push refresh signal → frontend re-fetches conversation
async fn handle_message_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["recipient_ids"].as_array() {
            Some(ids) => {
                let recipient_ids: Vec<i64> = ids.iter().filter_map(|id| id.as_i64()).collect();
                push_to_users(connections, recipient_ids, "messages_refresh", data).await;
            }
            None => {
                log::error!("message_deleted payload missing recipient_ids");
            }
        },
        Err(e) => {
            log::error!("Failed to parse message_deleted payload: {}", e);
        }
    }
}

// --- USERS ---

// Push signal to admins → re-fetch users list
async fn handle_user_created(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            push_to_users(
                connections,
                get_admin_ids(&data),
                "users_list_refresh",
                data,
            )
            .await;
        }
        Err(e) => {
            log::error!("Failed to parse user_created payload: {}", e);
        }
    }
}

// Push signal to admins → re-fetch users list
// Push signal to that user → re-fetch their own record
async fn handle_user_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            match data["user_id"].as_i64() {
                Some(user_id) => {
                    // Notify the updated user to re-fetch their own record
                    push_to_user(connections, user_id, "user_refresh", data.clone()).await;
                    // Notify admins to re-fetch users list
                    push_to_users(
                        connections,
                        get_admin_ids(&data),
                        "users_list_refresh",
                        data,
                    )
                    .await;
                }
                None => {
                    log::error!("user_updated payload missing user_id");
                }
            }
        }
        Err(e) => {
            log::error!("Failed to parse user_updated payload: {}", e);
        }
    }
}

// Force logout the suspended user
// Push signal to admins → re-fetch users list
async fn handle_user_suspended(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["user_id"].as_i64() {
            Some(user_id) => {
                push_to_user(
                    connections,
                    user_id,
                    "force_logout",
                    serde_json::json!({ "reason": "account_suspended" }),
                )
                .await;
                push_to_users(
                    connections,
                    get_admin_ids(&data),
                    "users_list_refresh",
                    data,
                )
                .await;
            }
            None => {
                log::error!("user_suspended payload missing user_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse user_suspended payload: {}", e);
        }
    }
}

// Force logout the deleted user
// Push signal to admins → re-fetch users list
async fn handle_user_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["user_id"].as_i64() {
            Some(user_id) => {
                push_to_user(
                    connections,
                    user_id,
                    "force_logout",
                    serde_json::json!({ "reason": "account_deleted" }),
                )
                .await;
                push_to_users(
                    connections,
                    get_admin_ids(&data),
                    "users_list_refresh",
                    data,
                )
                .await;
            }
            None => {
                log::error!("user_deleted payload missing user_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse user_deleted payload: {}", e);
        }
    }
}

// --- GROUPS ---

// Push signal to all members → re-fetch groups list + group details if open
async fn handle_group_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["member_ids"].as_array() {
            Some(ids) => {
                let member_ids: Vec<i64> = ids.iter().filter_map(|id| id.as_i64()).collect();
                push_to_users(connections, member_ids, "group_updated", data).await;
            }
            None => {
                log::error!("group_updated payload missing member_ids");
            }
        },
        Err(e) => {
            log::error!("Failed to parse group_updated payload: {}", e);
        }
    }
}

// Push signal to all members → remove from groups list, redirect if open
async fn handle_group_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["member_ids"].as_array() {
            Some(ids) => {
                let member_ids: Vec<i64> = ids.iter().filter_map(|id| id.as_i64()).collect();
                push_to_users(connections, member_ids, "group_deleted", data).await;
            }
            None => {
                log::error!("group_deleted payload missing member_ids");
            }
        },
        Err(e) => {
            log::error!("Failed to parse group_deleted payload: {}", e);
        }
    }
}

// --- GROUP PERMISSIONS ---

// Push signal to added user → re-fetch their groups list
// Push signal to group members → re-fetch group permissions list
async fn handle_group_permission_added(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["user_id"].as_i64() {
            Some(user_id) => {
                push_to_user(connections, user_id, "groups_refresh", data.clone()).await;
                push_to_users(
                    connections,
                    get_member_ids(&data),
                    "group_permissions_refresh",
                    data,
                )
                .await;
            }
            None => {
                log::error!("group_permission_added payload missing user_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse group_permission_added payload: {}", e);
        }
    }
}

// Push signal to affected user → re-fetch their groups list
// Push signal to group members → re-fetch group permissions list
// If new permission is blocked → frontend handles redirect
async fn handle_group_permission_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["user_id"].as_i64() {
            Some(user_id) => {
                push_to_user(
                    connections,
                    user_id,
                    "group_permission_updated",
                    data.clone(),
                )
                .await;
                push_to_users(
                    connections,
                    get_member_ids(&data),
                    "group_permissions_refresh",
                    data,
                )
                .await;
            }
            None => {
                log::error!("group_permission_updated payload missing user_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse group_permission_updated payload: {}", e);
        }
    }
}

// Push signal to removed user → re-fetch their groups list, redirect if open
// Push signal to group members → re-fetch group permissions list
async fn handle_group_permission_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match data["user_id"].as_i64() {
            Some(user_id) => {
                push_to_user(
                    connections,
                    user_id,
                    "group_permission_deleted",
                    data.clone(),
                )
                .await;
                push_to_users(
                    connections,
                    get_member_ids(&data),
                    "group_permissions_refresh",
                    data,
                )
                .await;
            }
            None => {
                log::error!("group_permission_deleted payload missing user_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse group_permission_deleted payload: {}", e);
        }
    }
}

// --- RELATIONSHIPS ---

// Push signal to both users → re-fetch relationships list
async fn handle_relationship_added(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match (data["requester_id"].as_i64(), data["receiver_id"].as_i64()) {
            (Some(requester_id), Some(receiver_id)) => {
                push_to_users(
                    connections,
                    vec![requester_id, receiver_id],
                    "relationships_refresh",
                    data,
                )
                .await;
            }
            _ => {
                log::error!("relationship_added payload missing requester_id or receiver_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse relationship_added payload: {}", e);
        }
    }
}

// Push signal to both users → re-fetch relationships list
// Frontend handles redirect if blocked and DM is open
async fn handle_relationship_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match (data["requester_id"].as_i64(), data["receiver_id"].as_i64()) {
            (Some(requester_id), Some(receiver_id)) => {
                push_to_users(
                    connections,
                    vec![requester_id, receiver_id],
                    "relationship_updated",
                    data,
                )
                .await;
            }
            _ => {
                log::error!("relationship_updated payload missing requester_id or receiver_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse relationship_updated payload: {}", e);
        }
    }
}

// Push signal to both users → re-fetch relationships list
async fn handle_relationship_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => match (data["requester_id"].as_i64(), data["receiver_id"].as_i64()) {
            (Some(requester_id), Some(receiver_id)) => {
                push_to_users(
                    connections,
                    vec![requester_id, receiver_id],
                    "relationships_refresh",
                    data,
                )
                .await;
            }
            _ => {
                log::error!("relationship_deleted payload missing requester_id or receiver_id");
            }
        },
        Err(e) => {
            log::error!("Failed to parse relationship_deleted payload: {}", e);
        }
    }
}

// --- HELPERS ---

fn get_admin_ids(data: &serde_json::Value) -> Vec<i64> {
    match data["admin_ids"].as_array() {
        Some(ids) => ids.iter().filter_map(|id| id.as_i64()).collect(),
        None => vec![],
    }
}

// Extracts member_ids from group permission payloads
fn get_member_ids(data: &serde_json::Value) -> Vec<i64> {
    match data["member_ids"].as_array() {
        Some(ids) => ids.iter().filter_map(|id| id.as_i64()).collect(),
        None => vec![],
    }
}
