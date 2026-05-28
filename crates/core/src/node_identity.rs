// crates/sentinel/src/node_identity.rs
// =====================================================
// NodeIdentity v4.0 — Identité Souveraine & Attestation Post-Quantique
// Dilithium5 + HybridTransport + Réputation Dynamique + Peer Trust
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use skyainet_secure_transport::crypto::dilithium::Dilithium5Signer;
use skyainet_secure_transport::crypto::hybrid::HybridTransport;
use crate::rewards::UserRewards;

/// Attestation cryptographique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub timestamp: u64,
    pub signature: Vec<u8>,
    pub valid: bool,
    pub issuer: String,
}

/// Identité d’un nœud
#[derive(Debug, Clone)]
pub struct NodeIdentity {
    pub node_id: [u8; 32],
    pub sovereign_alias: String,
    pub public_key: Vec<u8>,
    pub reputation: f64,
    pub attestations: Vec<Attestation>,
    pub registered_peers: HashMap<[u8; 32], f64>, // peer_id → reputation

    // Crypto
    pub signer: Dilithium5Signer,
    pub hybrid: HybridTransport,
}

impl NodeIdentity {
    pub fn new(sovereign_alias: &str) -> Self {
        let signer = Dilithium5Signer::new().expect("Dilithium key generation failed");
        let public_key = signer.public_key_bytes().to_vec();
        let mut node_id = [0u8; 32];
        rand::thread_rng().fill(&mut node_id[..]);

        Self {
            node_id,
            sovereign_alias: sovereign_alias.to_string(),
            public_key,
            reputation: 0.82,
            attestations: Vec::new(),
            registered_peers: HashMap::new(),
            signer,
            hybrid: HybridTransport::new(true),
        }
    }

    /// Génère une attestation cryptographique signée
    pub fn generate_attestation(&mut self) -> Attestation {
        let timestamp = crate::utils::now_millis();
        let message = format!("attest:{}:{}", self.sovereign_alias, timestamp);
        let signature = self.signer.sign(message.as_bytes());

        let attestation = Attestation {
            timestamp,
            signature,
            valid: true,
            issuer: self.sovereign_alias.clone(),
        };

        self.attestations.push(attestation.clone());
        attestation
    }

    /// Vérifie une attestation
    pub fn verify_attestation(&self, attestation: &Attestation) -> bool {
        let message = format!("attest:{}:{}", attestation.issuer, attestation.timestamp);
        self.signer.verify(message.as_bytes(), &attestation.signature).is_ok() &&
            attestation.valid &&
            (crate::utils::now_millis() - attestation.timestamp) < 300_000 // 5 minutes
    }

    pub fn attest(&self) -> bool {
        if self.attestations.is_empty() {
            return false;
        }
        let last = self.attestations.last().unwrap();
        self.verify_attestation(last) && self.reputation > 0.65
    }

    pub fn update_reputation(&mut self, delta: f64) {
        self.reputation = (self.reputation + delta).clamp(0.0, 1.0);
        info!("[NodeIdentity] Reputation updated: {:.3}", self.reputation);
    }

    pub fn register_peer(&mut self, peer_id: [u8; 32], initial_reputation: f64) {
        self.registered_peers.insert(peer_id, initial_reputation.clamp(0.0, 1.0));
    }

    pub fn update_peer_reputation(&mut self, peer_id: &[u8; 32], delta: f64) {
        if let Some(rep) = self.registered_peers.get_mut(peer_id) {
            *rep = (*rep + delta).clamp(0.0, 1.0);
        }
    }

    pub fn is_peer_trusted(&self, peer_id: &[u8; 32]) -> bool {
        self.registered_peers.get(peer_id).map(|&r| r > 0.68).unwrap_or(false)
    }

    pub fn trust_score(&self) -> f64 {
        let base = self.reputation;
        let attestation_bonus = if self.attest() { 0.12 } else { 0.0 };
        (base + attestation_bonus).min(1.0)
    }

    pub fn health_report(&self) -> String {
        format!(
            "NodeIdentity {} | Reputation: {:.3} | Attestations: {} | Trusted Peers: {}",
            self.sovereign_alias,
            self.reputation,
            self.attestations.len(),
            self.registered_peers.len()
        )
    }
}