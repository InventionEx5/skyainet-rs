// crates/secure/src/crypto/hybrid.rs
// =====================================================
// Hybrid Transport v7.0 — Stratégie Finale Production
// KemT369 (ML-KEM-768) + RomanT369 (Hyper256) + GematriaAead
// SkyAInet × Nikola T369
// =====================================================

use crate::crypto::kem_t369::{KemT369, KemPublicKey, KemCiphertext};
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::crypto::gematria_aead::GematriaAead;
use hkdf::Hkdf;
use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HybridError {
    #[error("KEM encapsulation failed")]
    EncapsulationFailed,
    #[error("KEM decapsulation failed")]
    DecapsulationFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Invalid mode")]
    InvalidMode,
}

/// Modes de chiffrement hybride (stratégie SkyAInet)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HybridMode {
    /// Mode principal du CŒUR (80% du trafic)
    /// KemT369 + RomanT369 Hyper256 — Post-quantique + rapide
    KemT369Core,

    /// Flash Gematria (20% du trafic cœur - métadonnées légères)
    FlashGematria,

    /// Mode complet pour les EXTRÉMITÉS (Mobile, Navigateur, WebRTC)
    FullGematria,
}

pub struct HybridTransport {
    kem: KemT369,
    current_mode: HybridMode,
    cached_secret: Option<[u8; 32]>,
}

impl HybridTransport {
    pub fn new(is_1024: bool) -> Self {
        Self {
            kem: KemT369::new(is_1024),
            current_mode: HybridMode::KemT369Core,
            cached_secret: None,
        }
    }

    pub fn set_mode(&mut self, mode: HybridMode) {
        self.current_mode = mode;
    }

    // =====================================================
    // CHIFFREMENT
    // =====================================================
    pub fn encrypt(
        &mut self,
        public_key: &KemPublicKey,
        plaintext: &[u8],
        mode: HybridMode,
    ) -> Result<(KemCiphertext, Vec<u8>), HybridError> {
        let (kem_ct, shared) = self.kem
            .encapsulate(public_key)
            .map_err(|_| HybridError::EncapsulationFailed)?;

        let (key, nonce) = derive_keys(&shared.secret);

        let ciphertext = match mode {
            HybridMode::KemT369Core | HybridMode::FullGematria => {
                // Mode principal : RomanT369 Hyper256
                let roman = RomanT369::new(key, nonce, GematriaMode::Hyper256);
                roman.encrypt(plaintext)
            }

            HybridMode::FlashGematria => {
                // Mode léger : GematriaAead sur métadonnées
                let aead = GematriaAead::new(key, nonce);
                aead.encrypt(plaintext)
            }
        };

        // Mise en cache pour les modes qui en ont besoin
        if mode == HybridMode::FullGematria {
            self.cached_secret = Some(shared.secret);
        }

        Ok((kem_ct, ciphertext))
    }

    // =====================================================
    // DÉCHIFFREMENT
    // =====================================================
    pub fn decrypt(
        &mut self,
        secret_key: &[u8],
        kem_ct: &KemCiphertext,
        ciphertext: &[u8],
        mode: HybridMode,
    ) -> Result<Vec<u8>, HybridError> {
        let shared = self.kem
            .decapsulate(secret_key, kem_ct)
            .map_err(|_| HybridError::DecapsulationFailed)?;

        let (key, nonce) = derive_keys(&shared.secret);

        let plaintext = match mode {
            HybridMode::KemT369Core | HybridMode::FullGematria => {
                let roman = RomanT369::new(key, nonce, GematriaMode::Hyper256);
                roman.decrypt(ciphertext).ok_or(HybridError::DecryptionFailed)?
            }

            HybridMode::FlashGematria => {
                let aead = GematriaAead::new(key, nonce);
                aead.decrypt(ciphertext).map_err(|_| HybridError::DecryptionFailed)?
            }
        };

        Ok(plaintext)
    }

    pub fn encrypt_with_current_mode(
        &mut self,
        public_key: &KemPublicKey,
        plaintext: &[u8],
    ) -> Result<(KemCiphertext, Vec<u8>), HybridError> {
        self.encrypt(public_key, plaintext, self.current_mode)
    }
}

// =====================================================
// Fonctions utilitaires
// =====================================================

fn derive_keys(shared_secret: &[u8; 32]) -> ([u8; 32], [u8; 12]) {
    let hk = Hkdf::<Sha256>::new(Some(b"SkyAInet-Hybrid-v7"), shared_secret);
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];

    hk.expand(b"gematria-key", &mut key).unwrap();
    hk.expand(b"gematria-nonce", &mut nonce).unwrap();

    (key, nonce)
}