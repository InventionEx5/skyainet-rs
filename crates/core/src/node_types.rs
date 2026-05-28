// crates/core/src/node_types.rs
// =====================================================
// Node Types v7.0 — Version Ultra Améliorée
// SkyAInet × Thevie — Architecture des Nœuds
// =====================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// =====================================================
/// TYPES DE NŒUDS (Architecture SkyAInet)
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Mini,           // Gratuit - Très léger
    Light,          // \~6€/mois - Équilibré
    Full,           // \~18€/mois - Hautes performances
    Validator,      // \~55€/mois + staking - Consensus
    Sentinel,       // Surveillance & Auto-healing
    DreamWeaver,    // Spécialisé créatif
}

impl NodeType {
    /// Retourne si le nœud est payant
    pub fn is_paid(&self) -> bool {
        matches!(self, NodeType::Full | NodeType::Validator | NodeType::DreamWeaver)
    }

    /// Prix mensuel en euros
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            NodeType::Mini => 0,
            NodeType::Light => 6,
            NodeType::Full => 18,
            NodeType::Validator => 55,
            NodeType::Sentinel => 32,
            NodeType::DreamWeaver => 45,
        }
    }

    /// Multiplicateur de puissance de calcul
    pub fn compute_multiplier(&self) -> f64 {
        match self {
            NodeType::Mini => 1.0,
            NodeType::Light => 2.5,
            NodeType::Full => 6.0,
            NodeType::Validator => 12.0,
            NodeType::Sentinel => 4.0,
            NodeType::DreamWeaver => 8.5,
        }
    }
}

/// =====================================================
/// RÔLE DU NŒUD DANS LE RÉSEAU
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeRole {
    Core,           // Serveurs principaux
    Edge,           // Mobile / Navigateur
    Validator,      // Consensus
    Sentinel,       // Surveillance
    DreamWeaver,    // Créatif
}

/// =====================================================
/// ÉTAT DU NŒUD
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeState {
    Active,
    Sleeping,
    Syncing,
    DreamMode,
    Evolving,
    Gateway,
    Maintenance,
}

impl NodeState {
    pub fn is_operational(&self) -> bool {
        matches!(self, NodeState::Active | NodeState::Gateway | NodeState::DreamMode)
    }
}

/// =====================================================
/// NIVEAU D'ABONNEMENT
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionLevel {
    Free,
    Pro,
    Validator,
}

impl SubscriptionLevel {
    pub fn is_paid(&self) -> bool {
        !matches!(self, SubscriptionLevel::Free)
    }
}

/// =====================================================
/// CAPACITÉS DU NŒUD
/// =====================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub storage_gb: u64,
    pub compute_power: f64,
    pub max_concurrent_tasks: u32,
    pub supports_gpu: bool,
    pub supports_flash_gematria: bool,
    pub zip_memory_enabled: bool,
    pub custom_features: HashMap<String, bool>,
}

impl NodeCapabilities {
    pub fn new(level: &SubscriptionLevel) -> Self {
        match level {
            SubscriptionLevel::Free => Self {
                storage_gb: 8,
                compute_power: 1.0,
                max_concurrent_tasks: 2,
                supports_gpu: false,
                supports_flash_gematria: true,
                zip_memory_enabled: true,
                custom_features: HashMap::new(),
            },
            SubscriptionLevel::Pro => Self {
                storage_gb: 128,
                compute_power: 6.0,
                max_concurrent_tasks: 16,
                supports_gpu: true,
                supports_flash_gematria: true,
                zip_memory_enabled: true,
                custom_features: HashMap::from([
                    ("dynamic_site_generation".to_string(), true),
                    ("api_gateway".to_string(), true),
                ]),
            },
            SubscriptionLevel::Validator => Self {
                storage_gb: 512,
                compute_power: 12.0,
                max_concurrent_tasks: 64,
                supports_gpu: true,
                supports_flash_gematria: true,
                zip_memory_enabled: true,
                custom_features: HashMap::from([
                    ("consensus_participation".to_string(), true),
                    ("governance_voting".to_string(), true),
                ]),
            },
        }
    }

    pub fn adjust_for_state(&mut self, state: &NodeState) {
        match state {
            NodeState::Sleeping => {
                self.compute_power *= 0.2;
                self.max_concurrent_tasks = 1;
            }
            NodeState::DreamMode => {
                self.compute_power *= 0.6;
            }
            _ => {}
        }
    }
}

/// =====================================================
/// NIVEAU DE RÉPUTATION
/// =====================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReputationTier {
    Newcomer,    // 0.0 - 0.50
    Reliable,    // 0.50 - 0.70
    Trusted,     // 0.70 - 0.85
    Sovereign,   // 0.85 - 0.95
    Legend,      // 0.95 - 1.0
}

impl ReputationTier {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.95 => ReputationTier::Legend,
            s if s >= 0.85 => ReputationTier::Sovereign,
            s if s >= 0.70 => ReputationTier::Trusted,
            s if s >= 0.50 => ReputationTier::Reliable,
            _ => ReputationTier::Newcomer,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ReputationTier::Newcomer => "Newcomer",
            ReputationTier::Reliable => "Reliable",
            ReputationTier::Trusted => "Trusted",
            ReputationTier::Sovereign => "Sovereign",
            ReputationTier::Legend => "Legend",
        }
    }
}

/// =====================================================
/// FONCTIONS UTILITAIRES
/// =====================================================

pub fn default_capabilities_for_type(node_type: &NodeType) -> NodeCapabilities {
    match node_type {
        NodeType::Mini => NodeCapabilities::new(&SubscriptionLevel::Free),
        NodeType::Light | NodeType::Full => NodeCapabilities::new(&SubscriptionLevel::Pro),
        NodeType::Validator | NodeType::Sentinel | NodeType::DreamWeaver => {
            NodeCapabilities::new(&SubscriptionLevel::Validator)
        }
    }
}

pub fn is_edge_node(role: &NodeRole) -> bool {
    matches!(role, NodeRole::Edge)
}

pub fn requires_paid_subscription(node_type: &NodeType) -> bool {
    node_type.is_paid()
}