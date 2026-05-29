// crates/secure/src/protocol/mod.rs
// =====================================================
// Protocol Module — SkyAInet Secure Transport
// Version 6.1
// =====================================================

pub mod handshake;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use handshake::{
    Handshake,
    HandshakeMessage,
    NodeRole,
    CryptoSuite,
};