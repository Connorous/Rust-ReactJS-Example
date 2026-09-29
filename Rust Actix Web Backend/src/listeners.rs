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

async fn handle_user_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            match data["user_id"].as_i64() {
                Some(user_id) => {
                    // Always notify the user themselves
                    push_to_user(connections, user_id, "user_updated", data.clone()).await;

                    // Notify friends
                    let friend_ids = get_friend_ids(&data);
                    if (!friend_ids.is_empty()) {
                        push_to_users(connections, friend_ids, "user_updated", data.clone()).await;
                    }

                    // Only notify admins if admin_ids is not empty
                    // trigger only includes admin_ids when type or status changed
                    let admin_ids = get_admin_ids(&data);
                    if (!admin_ids.is_empty()) {
                        push_to_users(connections, admin_ids, "users_list_refresh", data).await;
                    }
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

async fn handle_user_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            match data["user_id"].as_i64() {
                Some(user_id) => {
                    // Force logout deleted user first
                    push_to_user(
                        connections,
                        user_id,
                        "force_logout",
                        serde_json::json!({ "reason": "account_deleted" }),
                    )
                    .await;

                    // Notify admins to refresh users list
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
            }
        }
        Err(e) => {
            log::error!("Failed to parse user_deleted payload: {}", e);
        }
    }
}

// --- GROUPS ---

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

async fn handle_group_permission_added(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            match data["user_id"].as_i64() {
                Some(user_id) => {
                    // Notify added user to refresh their groups list
                    push_to_user(connections, user_id, "groups_refresh", data.clone()).await;

                    // Notify existing members to refresh permissions list
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
            }
        }
        Err(e) => {
            log::error!("Failed to parse group_permission_added payload: {}", e);
        }
    }
}

async fn handle_group_permission_updated(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            match data["user_id"].as_i64() {
                Some(user_id) => {
                    // Notify affected user of their permission change
                    push_to_user(
                        connections,
                        user_id,
                        "group_permission_updated",
                        data.clone(),
                    )
                    .await;

                    // Notify existing members to refresh permissions list
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
            }
        }
        Err(e) => {
            log::error!("Failed to parse group_permission_updated payload: {}", e);
        }
    }
}

async fn handle_group_permission_deleted(payload: &str, connections: &UserConnections) {
    match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(data) => {
            match data["user_id"].as_i64() {
                Some(user_id) => {
                    // Notify removed user to refresh their groups list
                    push_to_user(
                        connections,
                        user_id,
                        "group_permission_deleted",
                        data.clone(),
                    )
                    .await;

                    // Notify existing members to refresh permissions list
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
            }
        }
        Err(e) => {
            log::error!("Failed to parse group_permission_deleted payload: {}", e);
        }
    }
}

// --- RELATIONSHIPS ---

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

fn get_member_ids(data: &serde_json::Value) -> Vec<i64> {
    match data["member_ids"].as_array() {
        Some(ids) => ids.iter().filter_map(|id| id.as_i64()).collect(),
        None => vec![],
    }
}

fn get_friend_ids(data: &serde_json::Value) -> Vec<i64> {
    match data["friend_ids"].as_array() {
        Some(ids) => ids.iter().filter_map(|id| id.as_i64()).collect(),
        None => vec![],
    }
}
