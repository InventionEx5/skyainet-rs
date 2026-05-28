// crates/node/src/main_server.rs
// =====================================================
// SkyNode HTTP Server v3.0 — API REST Décentralisée (Version Finale)
// Intégration complète avec Hub Central + Stockage Avancé + Routage IA
// =====================================================

use axum::{routing::{get, post}, Router, Json, extract::State};
use tokio::net::TcpListener;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::skynode::{SkyNode, AIRequest, AIResponse, FileMetadata};

pub type SharedSkyNode = Arc<Mutex<SkyNode>>;

pub async fn start_server(node: SharedSkyNode) {
    let app = Router::new()
        // === Routes existantes améliorées ===
        .route("/api/status", get(status))
        .route("/api/neural-mesh", get(neural_mesh))
        .route("/api/files", get(files))
        .route("/api/stats", get(stats))
        .route("/api/dream-cycle", get(trigger_dream_cycle))
        .route("/api/node", get(get_full_node_status))

        // === Nouvelles routes Hub Central & IA ===
        .route("/api/ai/generate", post(generate_with_ai))
        .route("/api/ai/message", post(send_ai_message))
        .route("/api/ai/list", get(get_registered_ais))
        .route("/api/ai/external", post(toggle_external_ai))

        // === Nouvelles routes Stockage Décentralisé Avancé ===
        .route("/api/storage/upload", post(upload_file))
        .route("/api/storage/list", get(list_files))
        .route("/api/storage/download", post(download_file))
        .route("/api/storage/delete", post(delete_file))

        .with_state(node);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("✅ SkyNode Server v3.0 started on http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}

// =====================================================
// ROUTES EXISTANTES (améliorées)
// =====================================================

async fn status(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    Json(json!({
        "status": if node.is_running { "active" } else { "stopped" },
        "node_id": node.id,
        "wisdom_score": node.wisdom_score,
        "total_requests": node.total_requests,
        "peers": node.peers.len(),
        "registered_ais": node.registered_ais.len(),
        "message_bus": node.message_bus.len()
    }))
}

async fn neural_mesh(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    Json(json!({
        "wisdom_level": node.wisdom_score,
        "evolution_cycles": node.evolution_cycles,
        "last_dream_cycle": node.last_dream_cycle
    }))
}

async fn files(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    let file_count = node.storage.list_files().map(|v| v.len()).unwrap_or(0);
    Json(json!({
        "total_files": file_count,
        "storage_type": "Decentralized + RomanT369 Encrypted"
    }))
}

async fn stats(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    Json(json!({
        "wisdom_score": node.wisdom_score,
        "total_requests": node.total_requests,
        "active_model": "T369Inference + LoraÉvo"
    }))
}

async fn trigger_dream_cycle(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let mut node = node.lock().await;
    match node.run_real_dream_cycle().await {
        Ok(msg) => Json(json!({ "success": true, "message": msg, "wisdom": node.wisdom_score })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}

async fn get_full_node_status(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    Json(json!({
        "id": node.id,
        "state": format!("{:?}", node.state),
        "is_running": node.is_running,
        "wisdom_score": node.wisdom_score,
        "total_requests": node.total_requests,
        "evolution_cycles": node.evolution_cycles,
        "peers_connected": node.peers.len(),
        "registered_ais": node.registered_ais.len(),
        "message_bus_size": node.message_bus.len()
    }))
}

// =====================================================
// NOUVELLES ROUTES - HUB CENTRAL & IA
// =====================================================

async fn generate_with_ai(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<AIRequest>,
) -> Json<serde_json::Value> {
    let mut node = node.lock().await;
    match node.generate_with_ai(payload).await {
        Ok(resp) => Json(json!({ "success": true, "response": resp })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}

async fn send_ai_message(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let mut node = node.lock().await;
    let from = payload["from"].as_str().unwrap_or("").to_string();
    let to = payload["to"].as_str().unwrap_or("").to_string();
    let content = payload["content"].as_str().unwrap_or("").to_string();

    match node.send_message(&from, &to, &content) {
        Ok(msg) => Json(json!({ "success": true, "message": msg })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}

async fn get_registered_ais(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    Json(json!({ "ais": node.registered_ais.keys().collect::<Vec<_>>() }))
}

async fn toggle_external_ai(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let mut node = node.lock().await;
    let enabled = payload["enabled"].as_bool().unwrap_or(false);
    node.enable_external_ai(enabled);
    Json(json!({ "success": true, "external_ai_enabled": enabled }))
}

// =====================================================
// NOUVELLES ROUTES - STOCKAGE DÉCENTRALISÉ AVANCÉ
// =====================================================

async fn upload_file(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let mut node = node.lock().await;
    let name = payload["name"].as_str().unwrap_or("unknown").to_string();
    let data = payload["data"].as_array().unwrap_or(&vec![]).iter()
        .filter_map(|v| v.as_u64().map(|n| n as u8))
        .collect::<Vec<u8>>();

    match node.upload_file(&name, &data) {
        Ok(id) => Json(json!({ "success": true, "file_id": id })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}

async fn list_files(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    match node.list_files() {
        Ok(files) => Json(json!({ "success": true, "files": files })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}

async fn download_file(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let node = node.lock().await;
    let id = payload["id"].as_str().unwrap_or("").to_string();

    match node.download_file(&id) {
        Ok(data) => Json(json!({ "success": true, "data": data })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}

async fn delete_file(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let mut node = node.lock().await;
    let id = payload["id"].as_str().unwrap_or("").to_string();

    match node.delete_file(&id) {
        Ok(success) => Json(json!({ "success": success })),
        Err(e) => Json(json!({ "success": false, "error": e })),
    }
}