// crates/core/src/node_types.rs
// =====================================================
// Node Types — Définitions Centrales du Système SkyAInet
// Gematria Flash Core + Zip Memory + Modèle Économique
// =====================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// =====================================================
/// TYPES DE NŒUDS (Architecture SkyAInet)
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeType {
    /// Mini Node (gratuit) — Très léger, Zip Memory activé
    Mini,
    /// Light Node (gratuit ou Pro) — Équilibre performance / stockage
    Light,
    /// Full Node (payant) — Stockage complet + haute disponibilité
    Full,
    /// Validator Node (payant + staking) — Consensus et gouvernance
    Validator,
    /// Sentinel Node — Surveillance + Auto-healing
    Sentinel,
    /// DreamWeaver Node — Spécialisé dans le traitement créatif
    DreamWeaver,
}

/// =====================================================
/// RÔLE DU NŒUD DANS LE RÉSEAU
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeRole {
    /// Nœud cœur (serveurs) — Priorité performance + Flash Gematria
    Core,
    /// Nœud extrémité (mobile, navigateur) — Priorité discrétion (Full Gematria)
    Edge,
    /// Nœud de validation — Haute sécurité + consensus
    Validator,
}

/// =====================================================
/// ÉTAT DU NŒUD
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeState {
    Initializing,
    Active,
    Sleeping,       // Zip Memory activé, consommation réduite
    Migrating,
    Syncing,
    Degraded,       // Problème détecté
    Offline,
}

/// =====================================================
/// NIVEAU D'ABONNEMENT (Modèle Économique Mixte)
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscriptionLevel {
    Free,       // Mini Node (5 Go, fonctionnalités limitées)
    Pro,        // Light Node (100 Go, Flash Gematria avancé)
    Validator,  // Full Node + staking (2 To, gouvernance)
}

impl SubscriptionLevel {
    pub fn storage_limit_gb(&self) -> u64 {
        match self {
            SubscriptionLevel::Free => 5,
            SubscriptionLevel::Pro => 100,
            SubscriptionLevel::Validator => 2000,
        }
    }

    pub fn monthly_price_eur(&self) -> Option<f64> {
        match self {
            SubscriptionLevel::Free => None,
            SubscriptionLevel::Pro => Some(4.99),
            SubscriptionLevel::Validator => Some(19.99),
        }
    }

    pub fn is_paid(&self) -> bool {
        *self != SubscriptionLevel::Free
    }
}

/// =====================================================
/// CAPACITÉS DU NŒUD (Dynamiques selon abonnement)
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub storage_gb: u64,
    pub compute_power: f64,          // 0.0 → 1.0
    pub bandwidth_mbps: u32,
    pub supports_flash_gematria: bool,
    pub supports_zip_memory: bool,
    pub max_concurrent_connections: u32,
    pub custom_tags: Vec<String>,
}

impl NodeCapabilities {
    pub fn new(subscription: &SubscriptionLevel) -> Self {
        match subscription {
            SubscriptionLevel::Free => Self {
                storage_gb: 5,
                compute_power: 0.3,
                bandwidth_mbps: 50,
                supports_flash_gematria: false,
                supports_zip_memory: true,
                max_concurrent_connections: 8,
                custom_tags: vec!["mini".to_string()],
            },
            SubscriptionLevel::Pro => Self {
                storage_gb: 100,
                compute_power: 0.7,
                bandwidth_mbps: 200,
                supports_flash_gematria: true,
                supports_zip_memory: true,
                max_concurrent_connections: 32,
                custom_tags: vec!["pro".to_string(), "flash".to_string()],
            },
            SubscriptionLevel::Validator => Self {
                storage_gb: 2000,
                compute_power: 1.0,
                bandwidth_mbps: 1000,
                supports_flash_gematria: true,
                supports_zip_memory: true,
                max_concurrent_connections: 128,
                custom_tags: vec!["validator".to_string(), "governance".to_string()],
            },
        }
    }

    /// Met à jour dynamiquement les capacités selon l’état du nœud
    pub fn adjust_for_state(&mut self, state: &NodeState) {
        match state {
            NodeState::Sleeping => {
                self.compute_power *= 0.3;
                self.bandwidth_mbps = (self.bandwidth_mbps as f32 * 0.4) as u32;
            }
            NodeState::Active => {
                // Rien à faire
            }
            _ => {}
        }
    }
}

/// =====================================================
/// RÉPUTATION ET NIVEAU DE CONFIANCE
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationTier {
    pub level: u8,           // 1 → 5
    pub name: String,
    pub min_score: f64,
    pub benefits: Vec<String>,
}

impl ReputationTier {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.95 => Self {
                level: 5,
                name: "Legendary".to_string(),
                min_score: 0.95,
                benefits: vec!["Priorité réseau", "Récompenses x3", "Accès gouvernance".to_string()],
            },
            s if s >= 0.85 => Self {
                level: 4,
                name: "Elite".to_string(),
                min_score: 0.85,
                benefits: vec!["Récompenses x2", "Accès anticipé".to_string()],
            },
            s if s >= 0.70 => Self {
                level: 3,
                name: "Trusted".to_string(),
                min_score: 0.70,
                benefits: vec!["Récompenses x1.5".to_string()],
            },
            s if s >= 0.50 => Self {
                level: 2,
                name: "Reliable".to_string(),
                min_score: 0.50,
                benefits: vec!["Accès standard".to_string()],
            },
            _ => Self {
                level: 1,
                name: "Newcomer".to_string(),
                min_score: 0.0,
                benefits: vec!["Accès limité".to_string()],
            },
        }
    }
}

/// =====================================================
/// FONCTIONS UTILITAIRES GLOBALES
/// =====================================================
pub fn default_capabilities_for_type(node_type: &NodeType) -> NodeCapabilities {
    match node_type {
        NodeType::Mini => NodeCapabilities::new(&SubscriptionLevel::Free),
        NodeType::Light => NodeCapabilities::new(&SubscriptionLevel::Pro),
        NodeType::Full | NodeType::Validator => NodeCapabilities::new(&SubscriptionLevel::Validator),
        _ => NodeCapabilities::new(&SubscriptionLevel::Pro),
    }
}

pub fn is_edge_node(role: &NodeRole) -> bool {
    *role == NodeRole::Edge
}

pub fn requires_paid_subscription(node_type: &NodeType) -> bool {
    matches!(node_type, NodeType::Full | NodeType::Validator)
}