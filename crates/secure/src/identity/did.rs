// crates/secure/src/identity/did.rs
// =====================================================
// DID (Decentralized Identifier) t369 v6.1
// SkyAInet × Nikola T369 — DID Core + Dilithium + Contact Integration
// =====================================================

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug, warn};
use thiserror::Error;

use crate::crypto::dilithium::{Dilithium5Signer, DilithiumError};

#[derive(Error, Debug)]
pub enum DidError {
    #[error("Invalid public key length")]
    InvalidPublicKey,
    #[error("Service endpoint not found")]
    ServiceNotFound,
    #[error("DID is revoked")]
    DidRevoked,
    #[error("Dilithium verification failed")]
    VerificationFailed,
    #[error("Dilithium error: {0}")]
    DilithiumError(#[from] DilithiumError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ServiceType {
    Messaging,
    Storage,
    Compute,
    Discovery,
    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub id: String,
    pub r#type: ServiceType,
    pub service_endpoint: String,
    pub priority: u8,
    pub created_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Did {
    pub id: String,                          // did:t369:...
    pub public_key: Vec<u8>,
    pub authentication: Vec<String>,
    pub service: Vec<ServiceEndpoint>,
    pub created_at: u64,
    pub updated_at: u64,
    pub revoked: bool,
    pub revocation_reason: Option<String>,
}

impl Did {
    /// Crée un nouveau DID t369 à partir d'une clé publique Dilithium
    pub fn new(public_key: Vec<u8>) -> Result<Self, DidError> {
        if public_key.len() != 32 {
            return Err(DidError::InvalidPublicKey);
        }

        let id = format!("did:t369:{}", hex::encode(&public_key[0..16]));
        let now = Self::current_timestamp();

        info!("[DID] Nouveau DID créé : {}", id);

        Ok(Self {
            id,
            public_key,
            authentication: vec!["Dilithium5VerificationKey2020".to_string()],
            service: vec![],
            created_at: now,
            updated_at: now,
            revoked: false,
            revocation_reason: None,
        })
    }

    /// Crée un DID directement depuis une clé Dilithium
    pub fn from_dilithium_key(dilithium_public_key: &[u8; 32]) -> Result<Self, DidError> {
        Self::new(dilithium_public_key.to_vec())
    }

    /// Ajoute un endpoint de service
    pub fn add_service(
        &mut self,
        id: String,
        service_type: ServiceType,
        endpoint: String,
        priority: u8,
    ) {
        let service = ServiceEndpoint {
            id,
            r#type: service_type,
            service_endpoint: endpoint,
            priority,
            created_at: Self::current_timestamp(),
        };

        self.service.push(service);
        self.updated_at = Self::current_timestamp();

        debug!("[DID] Service ajouté à {}", self.id);
    }

    /// Supprime un endpoint de service par son ID
    pub fn remove_service(&mut self, service_id: &str) -> Result<(), DidError> {
        let before = self.service.len();
        self.service.retain(|s| s.id != service_id);

        if self.service.len() == before {
            return Err(DidError::ServiceNotFound);
        }

        self.updated_at = Self::current_timestamp();
        debug!("[DID] Service {} supprimé de {}", service_id, self.id);
        Ok(())
    }

    /// Révoque le DID
    pub fn revoke(&mut self, reason: &str) {
        self.revoked = true;
        self.revocation_reason = Some(reason.to_string());
        self.updated_at = Self::current_timestamp();

        warn!("[DID] DID révoqué : {} (raison: {})", self.id, reason);
    }

    /// Vérifie une signature avec Dilithium (méthode principale du projet)
    pub fn verify_with_dilithium(
        &self,
        message: &[u8],
        signature: &[u8],
        dilithium_signer: &Dilithium5Signer,
    ) -> Result<bool, DidError> {
        if self.revoked {
            return Err(DidError::DidRevoked);
        }

        let is_valid = dilithium_signer
            .verify(message, signature)
            .map_err(DidError::from)?;

        if is_valid {
            debug!("[DID] Signature Dilithium vérifiée pour {}", self.id);
            Ok(true)
        } else {
            Err(DidError::VerificationFailed)
        }
    }

    /// Retourne le DID sous forme de document JSON (DID Document standard)
    pub fn to_did_document(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_default()
    }

    /// Retourne le DID sous forme courte (did:t369:...)
    pub fn to_short_string(&self) -> String {
        self.id.clone()
    }

    /// Met à jour la clé publique (rotation)
    pub fn rotate_public_key(&mut self, new_public_key: Vec<u8>) -> Result<(), DidError> {
        if new_public_key.len() != 32 {
            return Err(DidError::InvalidPublicKey);
        }

        self.public_key = new_public_key;
        self.updated_at = Self::current_timestamp();
        self.id = format!("did:t369:{}", hex::encode(&self.public_key[0..16]));

        info!("[DID] Rotation de clé effectuée pour {}", self.id);
        Ok(())
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Default for Did {
    fn default() -> Self {
        // DID de test (ne jamais utiliser en production)
        Self::new(vec![0u8; 32]).unwrap()
    }
}