// crates/secure/src/identity/mod.rs
// =====================================================
// Identity Module — SkyAInet Secure Transport
// Version 6.1
// =====================================================

pub mod did;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use did::{
    Did,
    DidError,
    ServiceType,
    ServiceEndpoint,
};