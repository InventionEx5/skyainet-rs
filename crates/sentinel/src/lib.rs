// crates/sentinel/src/lib.rs
// =====================================================
// Sentinel Crate v4.0 — Auto-Healing, Anti-Fork & Node Defense System
// Protection intelligente, détection proactive et auto-guérison du réseau SkyAInet
// =====================================================

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod auto_healing;
pub mod anti_fork;
pub mod node_identity;

// =====================================================
// RÉ-EXPORTS PUBLICS
// =====================================================

pub use auto_healing::Sentinel;
pub use anti_fork::AntiFork;
pub use node_identity::NodeIdentity;

// =====================================================
// TYPES COMMUNS RÉ-EXPORTÉS
// =====================================================

pub use auto_healing::{HealingAction, DetectedIssue, IssueSeverity};
pub use anti_fork::{ForkEvent, ForkSeverity};

// =====================================================
// VERSION DU MODULE
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}

// =====================================================
// FONCTION D'INITIALISATION GLOBALE
// =====================================================

/// Crée un système Sentinel complet et prêt à l'emploi
pub fn create_full_sentinel() -> (Sentinel, AntiFork, NodeIdentity) {
    let sentinel = Sentinel::new();
    let anti_fork = AntiFork::new();
    let node_identity = NodeIdentity::new("sentinel-core");

    info!("[Sentinel] Full defense system initialized successfully");
    (sentinel, anti_fork, node_identity)
}