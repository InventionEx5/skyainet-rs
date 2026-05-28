// crates/api/src/rest.rs
// =====================================================
// REST API v6.7 — SkyAInet × Thevie
// Ultra optimisé + Intégration complète Rewards
// =====================================================

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use skyainet_model::Thevie;
use skyainet_node::SkyAInetNode;
use crate::rewards::{UserRewards, RewardReason};

// =====================================================
// SHARED STATE
// =====================================================

pub type AppState = (
    Arc<Mutex<Thevie>>,
    Arc<Mutex<SkyAInetNode>>,
    Arc<Mutex<UserRewards>>,
);

// =====================================================
// REQUEST / RESPONSE STRUCTS
// =====================================================

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub response: String,
    pub quality: f32,
    pub expert_used: String,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct RewardsResponse {
    pub pending_rewards: u128,
    pub total_earned: u128,
    pub quality_score: f64,
    pub learn_contributions: u64,
    pub dream_cycles: u64,
}

// =====================================================
// HANDLERS
// =====================================================

async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "0.4.2".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

async fn version() -> impl IntoResponse {
    Json(serde_json::json!({
        "version": "0.4.2",
        "build": "production",
        "rust_version": env!("CARGO_PKG_VERSION")
    }))
}

async fn get_system_stats(
    State((thevie, _, _)): State<AppState>,
) -> impl IntoResponse {
    let thevie = thevie.lock().await;
    let stats = thevie.get_system_stats().await;
    Json(stats)
}

async fn get_node_dashboard(
    State((_, node, rewards)): State<AppState>,
) -> impl IntoResponse {
    let node = node.lock().await;
    let rewards = rewards.lock().await;

    Json(serde_json::json!({
        "tier": format!("{:?}", node.economics.tier),
        "monthly_cost_eur": node.economics.get_total_monthly_cost(),
        "is_rented_out": node.economics.is_rented_out,
        "estimated_monthly_earnings": node.get_estimated_earnings(),
        "total_earned_sky": node.economics.total_earned_sky,
        "pending_rewards": rewards.pending_rewards,
        "quality_score": rewards.conversation_quality_score,
        "learn_contributions": rewards.total_learn_contributions,
        "dream_cycles": rewards.total_dream_cycles,
    }))
}

async fn send_message(
    State((thevie, _, rewards)): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let mut thevie = thevie.lock().await;
    let response = thevie.process_query(payload.message.clone()).await;

    // Enregistrement comme interaction de qualité (optionnel)
    let mut rewards = rewards.lock().await;
    rewards.record_high_quality_interaction(0.85);

    Json(MessageResponse {
        response,
        quality: 0.91,
        expert_used: "orchestrator".to_string(),
    })
}

async fn trigger_dream_cycle(
    State((thevie, _, _)): State<AppState>,
) -> impl IntoResponse {
    let mut thevie = thevie.lock().await;
    thevie.trigger_dream_cycle().await;

    (StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "message": "Dream Cycle déclenché avec succès"
    })))
}

// === NOUVEAUX ENDPOINTS REWARDS ===

async fn get_rewards_stats(
    State((_, _, rewards)): State<AppState>,
) -> impl IntoResponse {
    let rewards = rewards.lock().await;

    Json(RewardsResponse {
        pending_rewards: rewards.pending_rewards,
        total_earned: rewards.total_sky_earned,
        quality_score: rewards.conversation_quality_score,
        learn_contributions: rewards.total_learn_contributions,
        dream_cycles: rewards.total_dream_cycles,
    })
}

async fn claim_rewards(
    State((_, _, rewards)): State<AppState>,
) -> impl IntoResponse {
    let mut rewards = rewards.lock().await;
    let amount = rewards.claim_monthly_rewards();

    (StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "claimed": amount,
        "new_total": rewards.total_sky_earned
    })))
}

// =====================================================
// ROUTER
// =====================================================

pub fn create_rest_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/stats", get(get_system_stats))
        .route("/node", get(get_node_dashboard))
        .route("/thevie/message", post(send_message))
        .route("/dream/trigger", post(trigger_dream_cycle))
        // === Rewards Endpoints ===
        .route("/rewards", get(get_rewards_stats))
        .route("/rewards/claim", post(claim_rewards))
        .with_state(state)
}