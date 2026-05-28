// crates/secure/src/crypto/dilithium.rs
// =====================================================
// Dilithium5 (FIPS 204) — Signatures Post-Quantiques Niveau 5
// SkyAInet × Nikola T369 — Version Production Ready
// =====================================================

use pqcrypto_dilithium::dilithium5::{
    DetachedSignature, PublicKey, SecretKey,
    detached_sign, keypair, verify_detached_signature,
};
use thiserror::Error;

const DILITHIUM5_SIGNATURE_SIZE: usize = 3293;

#[derive(Error, Debug)]
pub enum DilithiumError {
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Invalid secret key bytes")]
    InvalidSecretKeyBytes,
}

/// Paire de clés Dilithium5 (niveau de sécurité maximal)
#[derive(Clone)]
pub struct Dilithium5KeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl Dilithium5KeyPair {
    /// Génère une nouvelle paire de clés (sécurisée)
    pub fn generate() -> Result<Self, DilithiumError> {
        let (pk, sk) = keypair();
        Ok(Self { public_key: pk, secret_key: sk })
    }

    /// Signe un message (retourne une signature détachée)
    pub fn sign(&self, message: &[u8]) -> DetachedSignature {
        detached_sign(message, &self.secret_key)
    }

    /// Vérifie une signature
    pub fn verify(
        public_key: &PublicKey,
        message: &[u8],
        signature: &DetachedSignature,
    ) -> Result<(), DilithiumError> {
        verify_detached_signature(signature, message, public_key)
            .map_err(|_| DilithiumError::VerificationFailed)
    }

    /// Exporte la clé publique en bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.as_bytes().to_vec()
    }

    /// Exporte la clé privée en bytes (à utiliser avec précaution)
    pub fn secret_key_bytes(&self) -> Vec<u8> {
        self.secret_key.as_bytes().to_vec()
    }
}

/// Signer optimisé avec cache (recommandé pour usage courant)
pub struct Dilithium5Signer {
    keypair: Dilithium5KeyPair,
    cached_public: Vec<u8>,
}

impl Dilithium5Signer {
    pub fn new() -> Result<Self, DilithiumError> {
        let keypair = Dilithium5KeyPair::generate()?;
        let cached_public = keypair.public_key_bytes();
        Ok(Self { keypair, cached_public })
    }

    /// Signe un message et retourne la signature en bytes
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).as_bytes().to_vec()
    }

    /// Vérifie une signature (API simplifiée)
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), DilithiumError> {
        if signature.len() != DILITHIUM5_SIGNATURE_SIZE {
            return Err(DilithiumError::InvalidSignature);
        }

        let sig = DetachedSignature::from_bytes(signature)
            .map_err(|_| DilithiumError::InvalidSignature)?;

        Dilithium5KeyPair::verify(&self.keypair.public_key, message, &sig)
    }

    /// Retourne la clé publique en bytes (pour partage)
    pub fn public_key_bytes(&self) -> &[u8] {
        &self.cached_public
    }

    /// Crée un signer à partir d’une clé privée existante
    pub fn from_secret_key(secret_key_bytes: &[u8]) -> Result<Self, DilithiumError> {
        let secret_key = SecretKey::from_bytes(secret_key_bytes)
            .map_err(|_| DilithiumError::InvalidSecretKeyBytes)?;

        // On régénère la clé publique à partir de la privée (Dilithium le permet)
        let public_key = pqcrypto_dilithium::dilithium5::public_key_from_secret_key(&secret_key);

        let keypair = Dilithium5KeyPair { public_key, secret_key };
        let cached_public = keypair.public_key_bytes();

        Ok(Self { keypair, cached_public })
    }
}

impl Default for Dilithium5Signer {
    fn default() -> Self {
        Self::new().expect("Failed to generate Dilithium5 keypair")
    }
}