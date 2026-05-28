// crates/core/src/node_types.rs
// =====================================================
// Node Types v5.0 — Définitions Centrales & Architecture SkyAInet
// Types de nœuds, rôles, états, capacités & réputation
// =====================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::info;

/// =====================================================
/// TYPES DE NŒUDS
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Mini,           // Gratuit - Très léger
    Light,          // Gratuit/Pro - Équilibré
    Full,           // Payant - Haute performance
    Validator,      // Payant + Staking - Gouvernance
    Sentinel,       // Surveillance & Auto-Healing
    DreamWeaver,    // Spécialisé IA créative & Dream Cycle
    Mixed,          // Hybride multi-rôles
}

/// =====================================================
/// RÔLES DU NŒUD DANS LE RÉSEAU
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeRole {
    Core,           // Serveurs principaux - Haute disponibilité
    Edge,           // Nœuds légers (mobile, desktop, navigateur)
    Validator,      // Validation & Consensus
    Storage,        // Stockage décentralisé
    Compute,        // Calcul haute performance
    Sentinel,       // Surveillance du réseau
}

/// =====================================================
/// ÉTATS DU NŒUD
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    Initializing,
    Active,
    Sleeping,       // ZipMemory optimisé + faible consommation
    Syncing,
    Migrating,
    Degraded,       // Problème détecté (Sentinel actif)
    Offline,
    Quarantined,    // Mis en quarantaine par AntiFork
}

/// =====================================================
/// NIVEAUX D'ABONNEMENT
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionLevel {
    Free,
    Pro,
    Validator,
}

impl SubscriptionLevel {
    pub fn storage_limit_gb(&self) -> u64 {
        match self {
            SubscriptionLevel::Free => 8,
            SubscriptionLevel::Pro => 120,
            SubscriptionLevel::Validator => 2048,
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
        !matches!(self, SubscriptionLevel::Free)
    }
}

/// =====================================================
/// CAPACITÉS DU NŒUD (Dynamiques)
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub storage_gb: u64,
    pub compute_power: f64,           // 0.0 → 1.0
    pub bandwidth_mbps: u32,
    pub supports_flash_gematria: bool,
    pub supports_zip_memory: bool,
    pub max_concurrent_connections: u32,
    pub max_tasks: u32,
    pub custom_tags: Vec<String>,
}

impl NodeCapabilities {
    pub fn new(subscription: &SubscriptionLevel) -> Self {
        match subscription {
            SubscriptionLevel::Free => Self {
                storage_gb: 8,
                compute_power: 0.35,
                bandwidth_mbps: 60,
                supports_flash_gematria: false,
                supports_zip_memory: true,
                max_concurrent_connections: 12,
                max_tasks: 6,
                custom_tags: vec!["mini".to_string(), "light".to_string()],
            },
            SubscriptionLevel::Pro => Self {
                storage_gb: 120,
                compute_power: 0.78,
                bandwidth_mbps: 350,
                supports_flash_gematria: true,
                supports_zip_memory: true,
                max_concurrent_connections: 48,
                max_tasks: 24,
                custom_tags: vec!["pro".to_string(), "flash".to_string()],
            },
            SubscriptionLevel::Validator => Self {
                storage_gb: 2048,
                compute_power: 1.0,
                bandwidth_mbps: 1200,
                supports_flash_gematria: true,
                supports_zip_memory: true,
                max_concurrent_connections: 256,
                max_tasks: 120,
                custom_tags: vec!["validator".to_string(), "governance".to_string(), "high-performance".to_string()],
            },
        }
    }

    pub fn adjust_for_state(&mut self, state: &NodeState) {
        match state {
            NodeState::Sleeping | NodeState::Degraded => {
                self.compute_power *= 0.35;
                self.bandwidth_mbps = (self.bandwidth_mbps as f32 * 0.4) as u32;
            }
            NodeState::Active => {
                // Restauration des capacités nominales (optionnel)
            }
            _ => {}
        }
    }
}

/// =====================================================
/// TIERS DE RÉPUTATION
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationTier {
    pub level: u8,
    pub name: String,
    pub min_score: f64,
    pub benefits: Vec<String>,
}

impl ReputationTier {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.96 => Self { level: 5, name: "Legendary".to_string(), min_score: 0.96, benefits: vec!["×3 Rewards".to_string(), "Governance Priority".to_string(), "Exclusive Access".to_string()] },
            s if s >= 0.88 => Self { level: 4, name: "Elite".to_string(), min_score: 0.88, benefits: vec!["×2 Rewards".to_string(), "Early Features".to_string()] },
            s if s >= 0.75 => Self { level: 3, name: "Trusted".to_string(), min_score: 0.75, benefits: vec!["×1.5 Rewards".to_string()] },
            s if s >= 0.55 => Self { level: 2, name: "Reliable".to_string(), min_score: 0.55, benefits: vec!["Standard Access".to_string()] },
            _ => Self { level: 1, name: "Newcomer".to_string(), min_score: 0.0, benefits: vec!["Limited Access".to_string()] },
        }
    }
}

/// =====================================================
/// FONCTIONS UTILITAIRES GLOBALES
/// =====================================================
pub fn default_capabilities_for_type(node_type: &NodeType) -> NodeCapabilities {
    match node_type {
        NodeType::Mini | NodeType::Light => NodeCapabilities::new(&SubscriptionLevel::Free),
        NodeType::Full | NodeType::DreamWeaver => NodeCapabilities::new(&SubscriptionLevel::Pro),
        NodeType::Validator | NodeType::Sentinel => NodeCapabilities::new(&SubscriptionLevel::Validator),
        NodeType::Mixed => NodeCapabilities::new(&SubscriptionLevel::Pro),
    }
}

pub fn is_edge_node(role: &NodeRole) -> bool {
    matches!(role, NodeRole::Edge)
}

pub fn requires_paid_subscription(node_type: &NodeType) -> bool {
    matches!(node_type, NodeType::Full | NodeType::Validator | NodeType::DreamWeaver)
}

pub fn is_governance_eligible(node_type: &NodeType) -> bool {
    matches!(node_type, NodeType::Validator)
}