// crates/secure/src/crypto/mod.rs
// =====================================================
// Crypto Module — SkyAInet Secure Transport
// Version 5.0 — Strong Edition
// =====================================================

pub mod roman_t369;
pub mod gematria_aead;
pub mod kem_t369;
pub mod hybrid;
pub mod double_ratchet;
pub mod dilithium;
pub mod aes_fips;
pub mod sha_fips;
pub mod constant_time;
pub mod steganography;

// =====================================================
// Ré-exports principaux
// =====================================================
pub use roman_t369::{RomanT369, GematriaMode, RomanError};
pub use gematria_aead::GematriaAead;
pub use kem_t369::KemT369;
pub use hybrid::{HybridTransport, HybridMode};
pub use double_ratchet::DoubleRatchet;
pub use dilithium::Dilithium5Signer;
pub use aes_fips::Aes256GcmFips;
pub use sha_fips::{Sha256Hasher, hkdf_sha256};
pub use constant_time::{sample_uniform_mod, add_mod, sub_mod, constant_time_eq};
pub use steganography::MarkovSteganography;