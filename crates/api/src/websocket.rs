// crates/api/src/websocket.rs
// =====================================================
// WebSocket API v6.7 — SkyAInet × Thevie
// Version Simplifiée (sans gestion d'erreurs de verrouillage)
// =====================================================

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, warn};

use skyainet_node::SkyAInetNode;
use skyainet_model::Thevie;
use crate::rewards::UserRewards;

pub type AppState = (
    Arc<Mutex<Thevie>>,
    Arc<Mutex<SkyAInetNode>>,
    Arc<Mutex<UserRewards>>,
);

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State((thevie, node, rewards)): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, thevie, node, rewards))
}

async fn handle_socket(
    mut socket: WebSocket,
    thevie: Arc<Mutex<Thevie>>,
    node: Arc<Mutex<SkyAInetNode>>,
    rewards: Arc<Mutex<UserRewards>>,
) {
    info!("[WebSocket] Nouvelle connexion établie");

    let _ = socket
        .send(Message::Text(
            r#"{"type":"welcome","message":"Connecté à SkyAInet WebSocket"}"#.to_string(),
        ))
        .await;

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                debug!("[WebSocket] Message reçu: {}", text);

                if let Ok(parsed) = serde_json::from_str::<IncomingMessage>(&text) {
                    match parsed.message_type.as_str() {
                        "chat" => {
                            let mut thevie = thevie.lock().await;
                            let response = thevie.process_query(parsed.content).await;

                            let reply = serde_json::json!({
                                "type": "thevie_response",
                                "content": response,
                                "timestamp": chrono::Utc::now().to_rfc3339()
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        "stats" => {
                            let rewards = rewards.lock().await;
                            let reply = serde_json::json!({
                                "type": "stats",
                                "pending_rewards": rewards.pending_rewards,
                                "total_earned": rewards.total_sky_earned,
                                "quality_score": rewards.conversation_quality_score
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        "claim_rewards" => {
                            let mut rewards = rewards.lock().await;
                            let amount = rewards.claim_monthly_rewards();

                            let reply = serde_json::json!({
                                "type": "claim_result",
                                "claimed": amount
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        "node" => {
                            let node = node.lock().await;
                            let reply = serde_json::json!({
                                "type": "node",
                                "tier": format!("{:?}", node.economics.tier),
                                "is_rented_out": node.economics.is_rented_out
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        _ => {
                            warn!("[WebSocket] Type inconnu: {}", parsed.message_type);
                        }
                    }
                }
            }

            Message::Close(_) => {
                info!("[WebSocket] Connexion fermée");
                break;
            }

            _ => {}
        }
    }

    info!("[WebSocket] Connexion terminée");
}

#[derive(Deserialize)]
struct IncomingMessage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(default)]
    content: String,
}

pub fn create_websocket_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state)
}