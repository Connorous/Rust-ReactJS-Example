use crate::extractors::{errors, user_type, RequireUserType};
use crate::state::{AppState, UserConnections};
use actix_rt::spawn;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures_util::StreamExt;
use tokio::sync::broadcast;

pub async fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
    data: web::Data<AppState>,
    claims: RequireUserType<{ user_type::VIEWER }, { errors::DEFAULT }>,
) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    let user_id = claims.0.user_id;
    let connections = data.connections.clone();

    // Create broadcast channel for this user
    let (tx, _rx) = broadcast::channel::<String>(100);
    connections.write().await.insert(user_id, tx.clone());

    // Task: forward broadcast messages to this client
    let mut send_session = session.clone();
    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    if send_session.text(msg).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Task: receive from this client, handle ping/close
    let connections_clone = data.connections.clone();
    actix_rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
        connections_clone.write().await.remove(&user_id);
        let _ = session.close(None).await;
    });

    Ok(response)
}
