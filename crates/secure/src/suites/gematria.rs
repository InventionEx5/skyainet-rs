// crates/secure/src/suites/gematria.rs
// =====================================================
// Gematria Suite v6.1 — Version Finale
// AES-256-GCM + RomanT369 (Hyper256)
// Compatible Contact v6.2 + DID + GroupManager
// SkyAInet × Nikola T369
// =====================================================

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use thiserror::Error;
use tracing::{debug, warn};

use crate::crypto::roman_t369::{RomanT369, GematriaMode};

#[derive(Error, Debug)]
pub enum GematriaError {
    #[error("Invalid key length (must be 32 bytes)")]
    InvalidKeyLength,
    #[error("Data too short for decryption")]
    DataTooShort,
    #[error("RomanT369 decryption failed")]
    RomanDecryptionFailed,
    #[error("AES-GCM error: {0}")]
    AesGcmError(String),
}

pub struct GematriaSuite {
    pub name: &'static str,
    pub version: &'static str,
}

impl GematriaSuite {
    pub fn new() -> Self {
        Self {
            name: "Gematria",
            version: "v6.1",
        }
    }

    /// Chiffre avec AES-256-GCM + RomanT369 (Hyper256)
    pub fn encrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, GematriaError> {
        if key.len() != 32 {
            return Err(GematriaError::InvalidKeyLength);
        }

        // 1. AES-256-GCM
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let nonce = Nonce::from_slice(&nonce);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| GematriaError::AesGcmError(e.to_string()))?;

        // 2. Couche RomanT369 (Hyper256)
        let roman = RomanT369::new(
            key.try_into().unwrap(),
            nonce.as_slice().try_into().unwrap(),
            GematriaMode::Hyper256,
        );
        let mixed = roman.encrypt(&ciphertext);

        // 3. Résultat final (nonce + données chiffrées)
        let mut result = nonce.to_vec();
        result.extend_from_slice(&mixed);

        debug!("[GematriaSuite] Données chiffrées ({} octets)", result.len());
        Ok(result)
    }

    /// Déchiffre avec RomanT369 + AES-256-GCM
    pub fn decrypt(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, GematriaError> {
        if data.len() < 12 {
            return Err(GematriaError::DataTooShort);
        }

        if key.len() != 32 {
            return Err(GematriaError::InvalidKeyLength);
        }

        let nonce = Nonce::from_slice(&data[..12]);
        let mixed = &data[12..];

        // 1. Inverse RomanT369
        let roman = RomanT369::new(
            key.try_into().unwrap(),
            nonce.as_slice().try_into().unwrap(),
            GematriaMode::Hyper256,
        );
        let unmixed = roman
            .decrypt(mixed)
            .ok_or(GematriaError::RomanDecryptionFailed)?;

        // 2. AES-256-GCM
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let plaintext = cipher
            .decrypt(nonce, unmixed.as_ref())
            .map_err(|e| GematriaError::AesGcmError(e.to_string()))?;

        debug!("[GematriaSuite] Données déchiffrées avec succès");
        Ok(plaintext)
    }
}