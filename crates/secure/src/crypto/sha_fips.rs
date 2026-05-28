// crates/secure/src/crypto/sha_fips.rs
// =====================================================
// SHA-256 + HKDF-SHA256 (FIPS 140-3)
// Version Finale — SkyAInet × Nikola T369
// GematriaAead (post-quantique ready)
// =====================================================

use sha2::{Sha256, Digest};
use hkdf::Hkdf;
use thiserror::Error;
use subtle::ConstantTimeEq;

#[derive(Error, Debug)]
pub enum ShaError {
    #[error("HKDF expansion failed")]
    HkdfExpandFailed,
    #[error("Invalid output length")]
    InvalidOutputLength,
    #[error("Invalid input")]
    InvalidInput,
}

/// Wrapper SHA-256 (FIPS 140-3 compliant)
pub struct Sha256Hasher {
    hasher: Sha256,
}

impl Sha256Hasher {
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn finalize(self) -> [u8; 32] {
        self.hasher.finalize().into()
    }

    /// Version one-shot (plus pratique)
    pub fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self::new()
    }
}

/// HKDF-SHA256 (recommandé pour dérivation de clés)
pub fn hkdf_sha256(ikm: &[u8], salt: Option<&[u8]>, info: &[u8], okm: &mut [u8]) -> Result<(), ShaError> {
    if okm.len() > 255 * 32 {
        return Err(ShaError::InvalidOutputLength);
    }

    let hk = Hkdf::<Sha256>::new(salt, ikm);
    hk.expand(info, okm).map_err(|_| ShaError::HkdfExpandFailed)
}

/// Version simplifiée (panic en cas d'erreur — pour usage interne contrôlé)
pub fn hkdf_sha256_unchecked(ikm: &[u8], salt: Option<&[u8]>, info: &[u8], okm: &mut [u8]) {
    let hk = Hkdf::<Sha256>::new(salt, ikm);
    hk.expand(info, okm).expect("HKDF-SHA256 expand failed");
}

/// Dérivation de clés pour GematriaAead (remplace l'ancienne ChaCha20)
pub fn derive_gematria_aead_keys(root_key: &[u8; 32]) -> ([u8; 32], [u8; 12]) {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];

    // Clé principale pour GematriaAead
    hkdf_sha256_unchecked(root_key, Some(b"T369-GEMATRIA"), b"GEMATRIA-KEY", &mut key);

    // Nonce (12 octets)
    let mut full_nonce = [0u8; 32];
    hkdf_sha256_unchecked(root_key, Some(b"T369-GEMATRIA"), b"GEMATRIA-NONCE", &mut full_nonce);
    nonce.copy_from_slice(&full_nonce[..12]);

    (key, nonce)
}

/// Dérivation de clé AES-256 (compatibilité)
pub fn derive_aes_key(root_key: &[u8; 32], context: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    hkdf_sha256_unchecked(root_key, Some(b"T369-AES"), context, &mut key);
    key
}

/// Comparaison constante (résistance aux attaques temporelles)
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}

/// HMAC-SHA256 (utile pour signatures et authenticité)
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key error");
    mac.update(data);
    mac.finalize().into_bytes().into()
}