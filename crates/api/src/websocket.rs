// crates/api/src/websocket.rs
// =====================================================
// WebSocket API v6.7 — SkyAInet × Thevie
// Real-time Communication + Rewards Integration
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
use crate::rewards::{UserRewards, RewardReason};

// =====================================================
// SHARED STATE
// =====================================================

pub type AppState = (
    Arc<Mutex<Thevie>>,
    Arc<Mutex<SkyAInetNode>>,
    Arc<Mutex<UserRewards>>,   // ← Ajouté
);

// =====================================================
// WEBSOCKET HANDLER
// =====================================================

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

    // Message de bienvenue
    let _ = socket
        .send(Message::Text(
            serde_json::json!({
                "type": "welcome",
                "message": "Connecté à SkyAInet WebSocket v6.7"
            })
            .to_string(),
        ))
        .await;

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                debug!("[WebSocket] Message reçu: {}", text);

                if let Ok(parsed) = serde_json::from_str::<IncomingMessage>(&text) {
                    match parsed.message_type.as_str() {
                        // === CHAT AVEC THEVIE ===
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

                        // === DEMANDE DE STATISTIQUES ===
                        "stats" => {
                            let rewards = rewards.lock().await;
                            let reply = serde_json::json!({
                                "type": "stats",
                                "pending_rewards": rewards.pending_rewards,
                                "total_earned": rewards.total_sky_earned,
                                "quality_score": rewards.conversation_quality_score,
                                "learn_contributions": rewards.total_learn_contributions,
                                "dream_cycles": rewards.total_dream_cycles
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        // === CLAIM MENSUEL DES REWARDS ===
                        "claim_rewards" => {
                            let mut rewards = rewards.lock().await;
                            let amount = rewards.claim_monthly_rewards();

                            let reply = serde_json::json!({
                                "type": "claim_result",
                                "claimed": amount,
                                "new_total": rewards.total_sky_earned
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        // === INFORMATIONS NOEUD ===
                        "node" => {
                            let node = node.lock().await;
                            let reply = serde_json::json!({
                                "type": "node",
                                "tier": format!("{:?}", node.economics.tier),
                                "is_rented_out": node.economics.is_rented_out,
                                "monthly_cost": node.economics.get_total_monthly_cost()
                            });
                            let _ = socket.send(Message::Text(reply.to_string())).await;
                        }

                        _ => {
                            warn!("[WebSocket] Type de message inconnu: {}", parsed.message_type);
                        }
                    }
                }
            }

            Message::Close(_) => {
                info!("[WebSocket] Connexion fermée par le client");
                break;
            }

            _ => {}
        }
    }

    info!("[WebSocket] Connexion terminée");
}

// =====================================================
// STRUCTURES
// =====================================================

#[derive(Deserialize)]
struct IncomingMessage {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(default)]
    content: String,
}

// =====================================================
// ROUTER
// =====================================================

pub fn create_websocket_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state)
}