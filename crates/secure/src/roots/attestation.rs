// crates/secure/src/roots/attestation.rs
// =====================================================
// Node Attestation v6.1 — Dilithium5 + Timestamp + DID
// Compatible Contact v6.2 + DID + RomanT369 + GroupManager
// DiamantRoots v2 — Preuve d’Identité Post-Quantique
// SkyAInet × Nikola T369
// =====================================================

use crate::crypto::dilithium::{Dilithium5Signer, DilithiumError};
use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;

use serde::{Serialize, Deserialize};
use thiserror::Error;
use tracing::{debug, warn, info};

#[derive(Error, Debug)]
pub enum AttestationError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Dilithium verification failed: {0}")]
    DilithiumError(String),
    #[error("Attestation expired")]
    Expired,
    #[error("Contact not verified (DID required)")]
    ContactNotVerified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAttestation {
    pub node_id: [u8; 32],
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub timestamp: u64,
    pub epoch: u64,
    pub did: Option<String>,           // ← Nouveau : DID du nœud
}

impl NodeAttestation {
    /// Vérifie l’attestation avec Dilithium5 + DID (si présent)
    pub fn verify(
        &self,
        signer: &Dilithium5Signer,
        contact_manager: Option<&ContactManager>,
    ) -> Result<(), AttestationError> {
        // Vérification de l’expiration (24h)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now - self.timestamp > 86400 {
            return Err(AttestationError::Expired);
        }

        // Vérification Dilithium5
        match signer.verify(&self.public_key, &self.signature) {
            Ok(()) => {
                debug!(
                    "[Attestation] Nœud {} attesté avec succès",
                    hex::encode(&self.node_id[0..8])
                );
            }
            Err(e) => {
                warn!(
                    "[Attestation] Échec de vérification pour le nœud {}",
                    hex::encode(&self.node_id[0..8])
                );
                return Err(AttestationError::DilithiumError(e.to_string()));
            }
        }

        // Vérification DID si un ContactManager est fourni
        if let (Some(manager), Some(did_str)) = (contact_manager, &self.did) {
            // On cherche le contact par node_id
            if let Some(contact) = manager.get(&self.node_id) {
                if !contact.has_decentralized_identity() || contact.verification_level < 2 {
                    return Err(AttestationError::ContactNotVerified);
                }
            }
        }

        Ok(())
    }

    /// Crée une nouvelle attestation (côté nœud)
    pub fn create(
        node_id: [u8; 32],
        public_key: Vec<u8>,
        signature: Vec<u8>,
        epoch: u64,
        contact: Option<&Contact>,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let did = contact.and_then(|c| c.get_did_string());

        Self {
            node_id,
            public_key,
            signature,
            timestamp,
            epoch,
            did,
        }
    }
}