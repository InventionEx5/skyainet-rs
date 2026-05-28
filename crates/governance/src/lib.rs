// crates/governance/src/lib.rs
// =====================================================
// SkyAInet Governance Crate v5.0
// DAO + Conviction Voting + PoSI + Governance Engine
// =====================================================

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub mod dao;
pub mod posi;
pub mod conviction_voting;

// =====================================================
// RÉ-EXPORTS PUBLICS
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
// VERSION DU CRATE
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}

// =====================================================
// FONCTIONS D'USINE (Factory)
// =====================================================

/// Crée un système de gouvernance complet et prêt à l'emploi
pub fn create_full_governance() -> (Dao, PoSI, ConvictionVoting) {
    let dao = Dao::new();
    let posi = PoSI::new();
    let conviction = ConvictionVoting::new();

    info!("[Governance] Full governance system initialized successfully");
    (dao, posi, conviction)
}