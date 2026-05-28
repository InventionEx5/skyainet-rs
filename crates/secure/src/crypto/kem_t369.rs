// crates/secure/src/crypto/kem_t369.rs
// =====================================================
// KemT369 v3.0 — Pure Post-Quantum KEM (Production Ready)
// ML-KEM-768 + RomanT369 (Hyper256) + Secure Key Derivation
// SkyAInet × Nikola T369
// =====================================================

use pqcrypto_mlkem::{ml_kem_768, ml_kem_1024};
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use hkdf::Hkdf;
use sha2::Sha256;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KemError {
    #[error("Invalid ML-KEM public key")]
    InvalidPublicKey,
    #[error("Invalid ML-KEM secret key")]
    InvalidSecretKey,
    #[error("Invalid ML-KEM ciphertext")]
    InvalidCiphertext,
    #[error("Key derivation failed")]
    DerivationFailed,
}

#[derive(Clone, Debug)]
pub struct KemT369 {
    pub is_1024: bool, // false = ML-KEM-768 (recommandé), true = ML-KEM-1024
}

#[derive(Clone, Debug)]
pub struct KemPublicKey {
    pub ml_kem_public: Vec<u8>,
    pub is_1024: bool,
}

#[derive(Clone, Debug)]
pub struct KemCiphertext {
    pub ml_kem_ciphertext: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct KemSharedSecret {
    pub secret: [u8; 32],
}

impl KemT369 {
    pub fn new(is_1024: bool) -> Self {
        Self { is_1024 }
    }

    /// Génère une paire de clés ML-KEM (768 ou 1024)
    pub fn generate_keypair(&self) -> (KemPublicKey, Vec<u8>) {
        let (pk, sk) = if self.is_1024 {
            let (pk, sk) = ml_kem_1024::keypair();
            (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
        } else {
            let (pk, sk) = ml_kem_768::keypair();
            (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
        };

        let public_key = KemPublicKey {
            ml_kem_public: pk,
            is_1024: self.is_1024,
        };

        (public_key, sk)
    }

    /// Encapsulation (côté émetteur)
    pub fn encapsulate(&self, public_key: &KemPublicKey) -> Result<(KemCiphertext, KemSharedSecret), KemError> {
        let (ml_ciphertext, ml_shared) = if public_key.is_1024 {
            let pk = ml_kem_1024::PublicKey::from_bytes(&public_key.ml_kem_public)
                .map_err(|_| KemError::InvalidPublicKey)?;
            ml_kem_1024::encapsulate(&pk)
        } else {
            let pk = ml_kem_768::PublicKey::from_bytes(&public_key.ml_kem_public)
                .map_err(|_| KemError::InvalidPublicKey)?;
            ml_kem_768::encapsulate(&pk)
        };

        let final_secret = self.derive_final_key(&ml_shared)?;

        let ciphertext = KemCiphertext {
            ml_kem_ciphertext: ml_ciphertext.as_bytes().to_vec(),
        };

        Ok((ciphertext, final_secret))
    }

    /// Décapsulation (côté récepteur)
    pub fn decapsulate(
        &self,
        secret_key: &[u8],
        ciphertext: &KemCiphertext,
    ) -> Result<KemSharedSecret, KemError> {
        let ml_shared = if self.is_1024 {
            let sk = ml_kem_1024::SecretKey::from_bytes(secret_key)
                .map_err(|_| KemError::InvalidSecretKey)?;
            let ct = ml_kem_1024::Ciphertext::from_bytes(&ciphertext.ml_kem_ciphertext)
                .map_err(|_| KemError::InvalidCiphertext)?;
            ml_kem_1024::decapsulate(&sk, &ct)
        } else {
            let sk = ml_kem_768::SecretKey::from_bytes(secret_key)
                .map_err(|_| KemError::InvalidSecretKey)?;
            let ct = ml_kem_768::Ciphertext::from_bytes(&ciphertext.ml_kem_ciphertext)
                .map_err(|_| KemError::InvalidCiphertext)?;
            ml_kem_768::decapsulate(&sk, &ct)
        };

        let final_secret = self.derive_final_key(&ml_shared)?;
        Ok(final_secret)
    }

    /// Dérivation sécurisée de la clé finale (HKDF + RomanT369 Hyper256)
    fn derive_final_key(&self, ml_shared: &[u8]) -> Result<KemSharedSecret, KemError> {
        // Étape 1 : HKDF-SHA256
        let hk = Hkdf::<Sha256>::new(Some(b"KemT369-v3"), ml_shared);
        let mut secret = [0u8; 32];
        hk.expand(b"final-secret", &mut secret)
            .map_err(|_| KemError::DerivationFailed)?;

        // Étape 2 : Passe finale avec RomanT369 (Hyper256) pour diffusion
        let roman = RomanT369::new(secret, [0u8; 12], GematriaMode::Hyper256);
        let final_key = roman.encrypt(&secret);

        let mut result = [0u8; 32];
        result.copy_from_slice(&final_key[..32]);

        Ok(KemSharedSecret { secret: result })
    }
}