// crates/node/src/gateway.rs
// =====================================================
// SkyNode Sovereign Gateway v2.0 — Version Extrême
// Post-Quantique • IA Dynamique • Décentralisé • Sécurisé
// =====================================================

use axum::{
    extract::{Path, State, Json},
    routing::{get, post},
    Router, http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;
use tracing::{info, warn};

use crate::skynode::SkyNode;
use crate::crypto::hybrid::{HybridTransport, HybridMode};
use crate::crypto::gematria_aead::GematriaAead;
use crate::crypto::dilithium::Dilithium5Signer;

pub type SharedSkyNode = Arc<Mutex<SkyNode>>;

pub struct SovereignGateway {
    pub node: SharedSkyNode,
    pub hybrid: HybridTransport,
    pub signer: Dilithium5Signer,
    pub port: u16,
}

impl SovereignGateway {
    pub fn new(node: SharedSkyNode) -> Self {
        Self {
            node,
            hybrid: HybridTransport::new(false), // ML-KEM-768
            signer: Dilithium5Signer::new().expect("Failed to create Dilithium signer"),
            port: 8080,
        }
    }

    pub async fn start(&self) {
        let app = Router::new()
            .route("/api/status", get(self_status))
            .route("/api/sites/:site_id", get(serve_sovereign_site))
            .route("/api/generate", post(generate_dynamic_site))
            .route("/api/public/:file_id", get(serve_public_file))
            .route("/api/keys", post(create_api_key))
            .with_state(self.node.clone());

        let addr = format!("0.0.0.0:{}", self.port);
        info!("🌐 Sovereign Gateway v2.0 démarré sur http://{}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

// ==================== HANDLERS AVANCÉS ====================

async fn self_status(State(node): State<SharedSkyNode>) -> Json<serde_json::Value> {
    let node = node.lock().await;
    Json(json!({
        "status": "sovereign",
        "node_id": node.id,
        "wisdom_score": node.wisdom_score,
        "peers": node.peers.len(),
        "crypto": "KemT369 + RomanT369 Hyper256 + Dilithium5",
        "version": "2.0"
    }))
}

async fn serve_sovereign_site(
    State(node): State<SharedSkyNode>,
    Path(site_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let node = node.lock().await;

    match node.get_site(&site_id).await {
        Some(site) => {
            // Déchiffrement avec GematriaAead
            let aead = GematriaAead::new(site.encryption_key, site.nonce);
            let content = aead.decrypt(&site.encrypted_content)
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            // Signature Dilithium5
            let signature = node.dilithium_signer.sign(&content);

            Ok(Json(json!({
                "site_id": site_id,
                "content": String::from_utf8_lossy(&content),
                "signature": signature,
                "encrypted": true,
                "ai_generated": site.is_ai_generated
            })))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn generate_dynamic_site(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let prompt = payload.get("prompt").and_then(|v| v.as_str()).unwrap_or("");
    let node = node.lock().await;

    // Génération via Thevie / LoraÉvo (via T369Inference)
    match node.generate_with_ai(prompt).await {
        Ok(response) => {
            // Chiffrement du contenu généré
            let (key, nonce) = generate_encryption_keys();
            let aead = GematriaAead::new(key, nonce);
            let encrypted = aead.encrypt(response.as_bytes());

            let site_id = uuid::Uuid::new_v4().to_string();

            // Stockage dans le nœud
            node.store_ai_generated_site(&site_id, encrypted, key, nonce).await;

            Json(json!({
                "success": true,
                "site_id": site_id,
                "url": format!("/api/sites/{}", site_id),
                "message": "Site généré et chiffré avec RomanT369"
            }))
        }
        Err(e) => Json(json!({
            "success": false,
            "error": e
        }))
    }
}

async fn serve_public_file(
    State(node): State<SharedSkyNode>,
    Path(file_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let node = node.lock().await;

    match node.get_public_file(&file_id).await {
        Some(data) => Ok(Json(json!({
            "file_id": file_id,
            "data": data,
            "encrypted": true,
            "crypto": "GematriaAead + RomanT369"
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_api_key(
    State(node): State<SharedSkyNode>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("default");
    let node = node.lock().await;

    let api_key = node.generate_api_key(name).await;

    Json(json!({
        "success": true,
        "api_key": api_key,
        "message": "Clé API créée avec succès (stockée chiffrée)"
    }))
}

// ==================== UTILITAIRES ====================

fn generate_encryption_keys() -> ([u8; 32], [u8; 12]) {
    // À remplacer par une vraie génération sécurisée via HybridTransport
    ([0u8; 32], [0u8; 12])
}