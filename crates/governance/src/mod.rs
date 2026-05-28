// crates/governance/src/mod.rs
// =====================================================
// Governance Module — Déclaration et ré-exports
// DAO, PoSI, Conviction Voting & Gouvernance Décentralisée
// =====================================================

pub mod dao;
pub mod posi;
pub mod conviction_voting;

// =====================================================
// RÉ-EXPORTS PUBLICS (pour simplicité d'utilisation)
// =====================================================

pub use dao::Dao;
pub use posi::PoSI;
pub use conviction_voting::ConvictionVoting;

// =====================================================
// TYPES COMMUNS RÉ-EXPORTÉS
// =====================================================

pub use dao::{Proposal, ProposalStatus, DaoError};
pub use posi::{PoSIScore, PoSIError};
pub use conviction_voting::{ConvictionVote, ConvictionError};

// =====================================================
// VERSION DU MODULE
// =====================================================

pub const MODULE_VERSION: &str = "5.0.0";