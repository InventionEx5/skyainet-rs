// crates/secure/src/lib.rs
// =====================================================
// SkyAInet Secure Crate v6.0
// Post-Quantum Cryptography + Blockchain + Secure Transport
// =====================================================

#![warn(clippy::all, clippy::pedantic)]

pub mod blockchain;
pub mod crypto;           // À créer plus tard si besoin

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use blockchain::{BroadcastSession, EpochManager, EpochError, BroadcastError};

// =====================================================
// VERSION DU CRATE
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}

// =====================================================
// INITIALISATION (optionnelle)
// =====================================================

pub fn init() {
    tracing::info!("[Secure] SkyAInet Secure Layer initialized (v{})", VERSION);
}