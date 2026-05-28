// crates/secure/src/crypto/aes_fips.rs
// =====================================================
// AES-256-GCM (FIPS 140-3) — Version Production
// SkyAInet × Nikola T369 — Enterprise & Internal Use
// =====================================================

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, Payload};
use thiserror::Error;
use hkdf::Hkdf;
use sha2::Sha256;

#[derive(Error, Debug)]
pub enum AesError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid nonce length")]
    InvalidNonceLength,
}

/// AES-256-GCM conforme FIPS 140-3
/// Recommandé pour les communications internes du cœur du réseau
pub struct Aes256GcmFips {
    cipher: Aes256Gcm,
}

impl Aes256GcmFips {
    /// Crée une nouvelle instance à partir d'une clé de 32 octets
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    /// Chiffrement AEAD avec Additional Authenticated Data (AAD)
    pub fn encrypt(
        &self,
        nonce: &[u8; 12],
        plaintext: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, AesError> {
        let nonce = Nonce::from_slice(nonce);
        let payload = Payload { msg: plaintext, aad };

        self.cipher
            .encrypt(nonce, payload)
            .map_err(|_| AesError::EncryptionFailed)
    }

    /// Déchiffrement AEAD
    pub fn decrypt(
        &self,
        nonce: &[u8; 12],
        ciphertext: &[u8],
        aad: &[u8],
    ) -> Result<Vec<u8>, AesError> {
        let nonce = Nonce::from_slice(nonce);
        let payload = Payload { msg: ciphertext, aad };

        self.cipher
            .decrypt(nonce, payload)
            .map_err(|_| AesError::DecryptionFailed)
    }

    /// Génère une clé AES-256 dérivée via HKDF (recommandé)
    pub fn derive_key(root_key: &[u8; 32], info: &[u8]) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(Some(b"T369-AES-KEY"), root_key);
        let mut aes_key = [0u8; 32];
        hk.expand(info, &mut aes_key)
            .expect("HKDF expand failed for AES key");
        aes_key
    }
}

/// Méthodes simplifiées (sans AAD)
impl Aes256GcmFips {
    #[must_use]
    pub fn encrypt_simple(&self, nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
        self.encrypt(nonce, plaintext, &[])
    }

    #[must_use]
    pub fn decrypt_simple(&self, nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
        self.decrypt(nonce, ciphertext, &[])
    }
}