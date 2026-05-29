// crates/secure/src/crypto/mod.rs
// =====================================================
// SkyAInet Secure Crypto Module
// =====================================================

pub mod gematria;
pub mod hybrid;
pub mod kem_t369;
pub mod double_ratchet;
pub mod dilithium;
pub mod sha_fips;
pub mod aes_fips;
pub mod steganography;
pub mod constant_time;
pub mod roman_t369;           // ← Pour accès direct si besoin

// =====================================================
// RÉ-EXPORTS PRINCIPAUX (pour un usage facile)
// =====================================================

pub use gematria::roman_t369::{RomanT369, GematriaMode, RomanError};
pub use hybrid::{HybridTransport, HybridMode, HybridError};
pub use kem_t369::{KemT369, KemPublicKey, KemCiphertext, KemSharedSecret, KemError};
pub use double_ratchet::{DoubleRatchet, DoubleRatchetError};
pub use dilithium::{Dilithium5Signer, Dilithium5KeyPair, DilithiumError};
pub use sha_fips::{Sha256Hasher, hkdf_sha256, derive_gematria_aead_keys};
pub use aes_fips::{Aes256GcmFips, AesError};
pub use steganography::{MarkovSteganography, StegoError};
pub use constant_time::{
    sample_uniform_mod,
    add_mod,
    sub_mod,
    constant_time_eq,
    constant_time_eq_fixed,
};