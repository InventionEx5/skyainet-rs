// crates/secure/src/roots/mod.rs
// =====================================================
// Roots Module — DiamantRoots v2
// SkyAInet × Nikola T369
// Version 6.1 — Post-Quantique + DID + Contact Intégré
// =====================================================

pub mod attestation;
pub mod builder;
pub mod epoch_rekey;
pub mod pool;
pub mod reputation;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use attestation::{NodeAttestation, AttestationError};
pub use builder::{DiamantCircuitBuilder, CircuitBuilderError};
pub use epoch_rekey::{EpochRekeyManager, RekeyError};
pub use pool::{PeerPool, PeerInfo, PeerPoolError};
pub use reputation::{PeerReputation, ReputationTier, ReputationError};