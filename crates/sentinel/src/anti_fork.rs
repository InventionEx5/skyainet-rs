// crates/sentinel/src/anti_fork.rs
// =====================================================
// AntiFork v4.0 — Système Anti-Fork & Auto-Défense Avancé
// Détection par hauteur, hash, réputation + Actions automatiques (Slash, Quarantine)
// Intégré avec Rewards, NodeIdentity et Reputation System
// =====================================================

use tracing::{info, warn, debug, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::node_identity::NodeIdentity;
use crate::rewards::UserRewards;
use skyainet_secure_transport::crypto::dilithium::Dilithium5Signer;

/// Niveau de gravité d’un fork
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForkSeverity {
    Warning,
    Minor,
    Major,
    Critical,
}

/// Événement de fork détecté
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkEvent {
    pub timestamp: DateTime<Utc>,
    pub severity: ForkSeverity,
    pub description: String,
    pub affected_nodes: Vec<String>,
    pub evidence: String,
}

pub struct AntiFork {
    pub fork_threshold_height: u64,
    pub fork_threshold_hash: u32,
    pub events: Vec<ForkEvent>,
    pub quarantined_nodes: HashMap<String, DateTime<Utc>>,
    pub signer: Dilithium5Signer,
}

impl AntiFork {
    pub fn new() -> Self {
        Self {
            fork_threshold_height: 5,
            fork_threshold_hash: 2,
            events: Vec::new(),
            quarantined_nodes: HashMap::new(),
            signer: Dilithium5Signer::new().expect("Dilithium signer failed"),
        }
    }

    /// Détection avancée de fork (hauteur + hash + réputation)
    pub fn detect_fork(
        &mut self,
        local_height: u64,
        local_hash: &str,
        peers: &[(String, u64, String, f64)], // (peer_id, height, hash, reputation)
        node: &mut crate::skyainet_node::SkyAInetNode,
        rewards: &mut UserRewards,
    ) -> Vec<ForkEvent> {
        let mut detected = Vec::new();

        for (peer_id, peer_height, peer_hash, reputation) in peers {
            // 1. Détection par écart de hauteur
            let height_diff = (peer_height.saturating_sub(local_height)) as i64;
            if height_diff > self.fork_threshold_height as i64 {
                let event = ForkEvent {
                    timestamp: Utc::now(),
                    severity: ForkSeverity::Major,
                    description: format!("Fork par hauteur détecté avec peer {}", peer_id),
                    affected_nodes: vec![peer_id.clone()],
                    evidence: format!("Height diff: {}", height_diff),
                };
                detected.push(event);
                self.quarantine_node(peer_id, node);
            }

            // 2. Détection par mismatch de hash
            if peer_hash != local_hash && *reputation > 0.6 {
                let event = ForkEvent {
                    timestamp: Utc::now(),
                    severity: ForkSeverity::Critical,
                    description: format!("Hash mismatch avec peer {}", peer_id),
                    affected_nodes: vec![peer_id.clone()],
                    evidence: format!("Local: {} | Peer: {}", local_hash, peer_hash),
                };
                detected.push(event);
                self.quarantine_node(peer_id, node);
            }
        }

        if !detected.is_empty() {
            self.events.extend(detected.clone());
            warn!("[AntiFork] {} forks détectés", detected.len());
            
            // Récompense pour détection
            rewards.add_reward(crate::rewards::RewardReason::SecurityContribution, 35);
        }

        detected
    }

    /// Met un nœud en quarantaine
    pub fn quarantine_node(&mut self, peer_id: &str, node: &mut crate::skyainet_node::SkyAInetNode) {
        self.quarantined_nodes.insert(peer_id.to_string(), Utc::now());
        node.metadata.reputation_score = (node.metadata.reputation_score - 0.25).max(0.1);
        warn!("[AntiFork] Node {} mis en quarantaine", peer_id);
    }

    /// Vérifie si un pair est en quarantaine
    pub fn is_quarantined(&self, peer_id: &str) -> bool {
        self.quarantined_nodes.contains_key(peer_id)
    }

    pub fn get_events(&self) -> &[ForkEvent] {
        &self.events
    }

    pub fn summary(&self) -> String {
        format!(
            "AntiFork | Events: {} | Quarantined: {} | Threshold Height: {}",
            self.events.len(),
            self.quarantined_nodes.len(),
            self.fork_threshold_height
        )
    }
}