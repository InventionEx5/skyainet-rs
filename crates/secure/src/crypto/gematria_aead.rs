// crates/secure/src/crypto/gematria_aead.rs
// =====================================================
// Gematria AEAD v4.0 — Version Finale Production
// RomanT369 (Hyper256) + SHA-256 Auth Tag
// SkyAInet × Nikola T369
// Compatible avec derive_gematria_aead_keys (sha_fips)
// =====================================================

use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GematriaAeadError {
    #[error("Decryption failed: invalid tag or corrupted data")]
    DecryptionFailed,
    #[error("Invalid input length")]
    InvalidInputLength,
}

#[derive(Clone)]
pub struct GematriaAead {
    key: [u8; 32],
    nonce: [u8; 12],
}

impl GematriaAead {
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self { key, nonce }
    }

    /// Crée une instance à partir d'une root_key (utilise derive_gematria_aead_keys)
    pub fn from_root_key(root_key: &[u8; 32]) -> Self {
        let (key, nonce) = crate::crypto::sha_fips::derive_gematria_aead_keys(root_key);
        Self { key, nonce }
    }

    /// Chiffrement AEAD avec RomanT369 (Hyper256) + Tag d'authentification
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let roman = RomanT369::new(self.key, self.nonce, GematriaMode::Hyper256);
        let ciphertext = roman.encrypt(plaintext);

        // Tag d'authentification (16 premiers octets du SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(&self.key);
        hasher.update(&self.nonce);
        hasher.update(&ciphertext);
        let tag = hasher.finalize();

        let mut result = ciphertext;
        result.extend_from_slice(&tag[..16]);

        result
    }

    /// Déchiffrement + vérification du tag
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, GematriaAeadError> {
        if data.len() < 16 {
            return Err(GematriaAeadError::InvalidInputLength);
        }

        let (ciphertext, tag) = data.split_at(data.len() - 16);

        let roman = RomanT369::new(self.key, self.nonce, GematriaMode::Hyper256);
        let plaintext = roman.decrypt(ciphertext)
            .ok_or(GematriaAeadError::DecryptionFailed)?;

        // Vérification du tag
        let mut hasher = Sha256::new();
        hasher.update(&self.key);
        hasher.update(&self.nonce);
        hasher.update(ciphertext);
        let computed_tag = hasher.finalize();

        if &computed_tag[..16] != tag {
            return Err(GematriaAeadError::DecryptionFailed);
        }

        Ok(plaintext)
    }

    /// Chiffre et retourne le tag séparément (pour usage avancé)
    pub fn encrypt_with_tag(&self, plaintext: &[u8]) -> (Vec<u8>, [u8; 16]) {
        let roman = RomanT369::new(self.key, self.nonce, GematriaMode::Hyper256);
        let ciphertext = roman.encrypt(plaintext);

        let mut hasher = Sha256::new();
        hasher.update(&self.key);
        hasher.update(&self.nonce);
        hasher.update(&ciphertext);
        let tag = hasher.finalize();

        let mut tag_bytes = [0u8; 16];
        tag_bytes.copy_from_slice(&tag[..16]);

        (ciphertext, tag_bytes)
    }
}