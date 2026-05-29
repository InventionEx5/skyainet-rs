// crates/secure/src/group/mod.rs
// =====================================================
// Group Module — SkyAInet Secure Transport
// Version 5.2
// =====================================================

pub mod sender_keys;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use sender_keys::{
    GroupManager,
    Group,
    GroupError,
};