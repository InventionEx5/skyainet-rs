// crates/secure/src/transport/mod.rs
// =====================================================
// Transport Module — SkyAInet Secure Transport
// Version 6.1
// =====================================================

pub mod libp2p;
pub mod r#trait;           // "trait" est un mot-clé réservé en Rust

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use r#trait::{
    Transport,
    HybridTransport,
    TransportLayer,
    CryptoSuite,
    TransportError,
    HybridMode,
};