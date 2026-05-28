// crates/node/src/lib.rs
// =====================================================
// SkyAInet Node Crate
// Point d’entrée central pour tous les types de nœuds souverains
// Compute • Storage • Validator • Mixed • Communication
// =====================================================

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod skyainet_node;
pub mod node_types;
pub mod node_communication;
pub mod pouw;
pub mod dream_scoring;
pub mod storage;
pub mod mixed;
pub mod validator;
pub mod compute;
pub mod marketplace;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX (pour simplicité d'utilisation)
// =====================================================

pub use skyainet_node::SkyAInetNode;
pub use node_types::{
    NodeType, 
    NodeRole, 
    NodeState, 
    SubscriptionLevel, 
    NodeCapabilities,
    NodeTier,
    ReputationTier,
};
pub use pouw::{PoUWEngine, ContributionProof};
pub use dream_scoring::DreamScoring;
pub use storage::StorageNode;
pub use mixed::MixedNode;
pub use validator::ValidatorNode;
pub use compute::ComputeNode;
pub use marketplace::ComputeMarketplace;
pub use node_communication::NodeCommunication;

// =====================================================
// FONCTIONS D'USINE (Factory) — Crée facilement un nœud
// =====================================================

/// Crée un nœud selon son type principal
pub fn create_node(
    node_type: NodeType,
    alias: &str,
    subscription: SubscriptionLevel,
) -> SkyAInetNode {
    let capabilities = NodeCapabilities::new(&subscription);

    SkyAInetNode::new(
        node_type,
        match node_type {
            NodeType::Validator => NodeRole::Validator,
            NodeType::Storage => NodeRole::Storage,
            NodeType::Compute => NodeRole::Compute,
            _ => NodeRole::Full,
        },
        subscription,
        capabilities,
    )
}

/// Crée un nœud hybride (recommandé pour la plupart des usages)
pub fn create_mixed_node(alias: &str, subscription: SubscriptionLevel) -> MixedNode {
    MixedNode::new(alias, subscription)
}

// =====================================================
// VERSION & INFORMATIONS DU CRATE
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}