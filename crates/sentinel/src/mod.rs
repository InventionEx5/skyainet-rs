// crates/sentinel/src/mod.rs
// =====================================================
// Sentinel Module — Déclaration et ré-exports
// Auto-Healing, Anti-Fork, Node Identity & Defense System
// =====================================================

pub mod auto_healing;
pub mod anti_fork;
pub mod node_identity;

// =====================================================
// RÉ-EXPORTS PUBLICS (pour simplicité d'utilisation)
// =====================================================

pub use auto_healing::Sentinel;
pub use anti_fork::AntiFork;
pub use node_identity::NodeIdentity;

// =====================================================
// TYPES COMMUNS
// =====================================================

pub use auto_healing::{HealingAction, DetectedIssue, IssueSeverity};
pub use anti_fork::{ForkEvent, ForkSeverity};

// =====================================================
// VERSION
// =====================================================

pub const MODULE_VERSION: &str = "4.0.0";