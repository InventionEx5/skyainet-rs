// crates/secure/src/suites/post_quantum.rs
// =====================================================
// Post-Quantum Suite v6.1
// ML-KEM-768/1024 + RomanT369 (Hyper256)
// Compatible Contact v6.2 + DID + GroupManager
// SkyAInet × Nikola T369
// =====================================================

use crate::crypto::kem_t369::{KemT369, KemPublicKey, KemCiphertext};
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::contacts::contact::Contact;

use thiserror::Error;
use tracing::{debug, warn};

#[derive(Error, Debug)]
pub enum PostQuantumError {
    #[error("KEM encapsulation failed: {0}")]
    KemEncapsulationFailed(String),
    #[error("KEM decapsulation failed: {0}")]
    KemDecapsulationFailed(String),
    #[error("RomanT369 decryption failed")]
    RomanDecryptionFailed,
}

pub struct PostQuantumSuite {
    pub name: &'static str,
    pub version: &'static str,
    kem: KemT369,
}

impl PostQuantumSuite {
    pub fn new(is_1024: bool) -> Self {
        Self {
            name: "PostQuantum",
            version: "v6.1",
            kem: KemT369::new(is_1024),
        }
    }

    /// Génère une paire de clés post-quantique
    pub fn generate_keypair(&self) -> (KemPublicKey, Vec<u8>) {
        self.kem.generate_keypair()
    }

    /// Chiffrement hybride post-quantique (KEM + RomanT369)
    pub fn encrypt(
        &self,
        public_key: &KemPublicKey,
        plaintext: &[u8],
        contact: Option<&Contact>,
    ) -> Result<(KemCiphertext, Vec<u8>), PostQuantumError> {
        let (kem_ct, shared) = self.kem
            .encapsulate(public_key)
            .map_err(|e| PostQuantumError::KemEncapsulationFailed(e.to_string()))?;

        let roman = RomanT369::new(shared.secret, [0u8; 12], GematriaMode::Hyper256);
        let ciphertext = roman.encrypt(plaintext);

        debug!(
            "[PostQuantumSuite] Message chiffré (KEM + RomanT369) — Contact: {:?}",
            contact.map(|c| hex::encode(&c.node_id[0..4]))
        );

        Ok((kem_ct, ciphertext))
    }

    /// Déchiffrement hybride post-quantique
    pub fn decrypt(
        &self,
        secret_key: &[u8],
        kem_ct: &KemCiphertext,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, PostQuantumError> {
        let shared = self.kem
            .decapsulate(secret_key, kem_ct)
            .map_err(|e| PostQuantumError::KemDecapsulationFailed(e.to_string()))?;

        let roman = RomanT369::new(shared.secret, [0u8; 12], GematriaMode::Hyper256);
        let plaintext = roman
            .decrypt(ciphertext)
            .ok_or(PostQuantumError::RomanDecryptionFailed)?;

        debug!("[PostQuantumSuite] Message déchiffré avec succès");
        Ok(plaintext)
    }

    pub fn is_1024(&self) -> bool {
        self.kem.is_1024
    }
}