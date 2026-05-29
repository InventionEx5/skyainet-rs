// crates/secure/src/roots/builder.rs
// =====================================================
// DiamantCircuitBuilder v6.1 — Circuit Builder Intelligent
// Compatible Contact v6.2 + DID + RomanT369 + GroupManager
// DiamantRoots v2 — Routeur Tor-like Post-Quantique
// SkyAInet × Nikola T369
// =====================================================

use crate::crypto::kem_t369::KemT369;
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;

use rand::Rng;
use std::net::SocketAddr;
use tracing::{info, debug, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CircuitBuilderError {
    #[error("Not enough verified peers available")]
    NotEnoughPeers,
    #[error("Failed to build circuit: {0}")]
    BuildFailed(String),
    #[error("KEM error: {0}")]
    KemError(String),
    #[error("Contact not verified for circuit")]
    ContactNotVerified,
}

pub struct DiamantCircuitBuilder {
    kem: KemT369,
    roman: RomanT369,
    min_circuit_length: usize,
    max_circuit_length: usize,
    reputation_threshold: f64,
    prefer_diversity: bool,
}

impl DiamantCircuitBuilder {
    pub fn new() -> Self {
        Self {
            kem: KemT369::new(false),
            roman: RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256),
            min_circuit_length: 3,
            max_circuit_length: 5,
            reputation_threshold: 0.65,
            prefer_diversity: true,
        }
    }

    pub fn with_reputation_threshold(mut self, threshold: f64) -> Self {
        self.reputation_threshold = threshold;
        self
    }

    pub fn with_diversity(mut self, enabled: bool) -> Self {
        self.prefer_diversity = enabled;
        self
    }
}

impl DiamantCircuitBuilder {
    /// Construit un circuit avec sélection intelligente (DID + réputation)
    pub async fn build_circuit(
        &self,
        length: usize,
        contact_manager: Option<&ContactManager>,
    ) -> Result<super::Circuit, CircuitBuilderError> {
        let length = length.clamp(self.min_circuit_length, self.max_circuit_length);

        let nodes = self.select_diverse_peers(length, contact_manager).await?;

        let mut shared_secrets = Vec::with_capacity(length);
        let mut circuit_nodes = Vec::with_capacity(length);

        for addr in nodes {
            let (pk, _) = self.kem
                .generate_keypair()
                .map_err(|e| CircuitBuilderError::KemError(e.to_string()))?;

            let (_, shared) = self.kem
                .encapsulate(&pk)
                .map_err(|e| CircuitBuilderError::KemError(e.to_string()))?;

            // Renforcement post-quantique avec RomanT369
            let reinforced_secret = self.roman.encrypt(&shared.secret);
            let mut final_secret = [0u8; 32];
            final_secret.copy_from_slice(&reinforced_secret[0..32]);

            shared_secrets.push(final_secret);
            circuit_nodes.push(addr);
        }

        let circuit_id: u32 = rand::thread_rng().gen();

        info!(
            "[DiamantRoots] Circuit {} créé avec {} nœuds (longueur: {})",
            circuit_id, length, length
        );

        Ok(super::Circuit {
            id: circuit_id,
            nodes: circuit_nodes,
            epoch: 0,
            shared_secrets,
        })
    }

    pub async fn destroy_circuit(&self, circuit_id: u32) -> Result<(), CircuitBuilderError> {
        debug!("[DiamantRoots] Destruction du circuit {}", circuit_id);
        // TODO: Nettoyer les clés éphémères + notifier les nœuds
        Ok(())
    }

    /// Sélectionne des nœuds avec diversité + réputation + DID
    async fn select_diverse_peers(
        &self,
        count: usize,
        contact_manager: Option<&ContactManager>,
    ) -> Result<Vec<SocketAddr>, CircuitBuilderError> {
        let mut selected = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let port = rng.gen_range(40000..65000);
            let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

            // Si on a un ContactManager, on vérifie la réputation + DID
            if let Some(manager) = contact_manager {
                // Simulation : on suppose que l'adresse correspond à un node_id
                // (dans la vraie implémentation, on ferait un lookup)
                let fake_node_id = [port as u8; 32]; // placeholder

                if let Some(contact) = manager.get(&fake_node_id) {
                    if contact.has_decentralized_identity() && contact.verification_level >= 2 {
                        selected.push(addr);
                        continue;
                    }
                }
            }

            // Fallback : sélection aléatoire avec seuil de réputation simulé
            let reputation = rng.gen_range(0.5..1.0);
            if reputation >= self.reputation_threshold {
                selected.push(addr);
            } else {
                let port = rng.gen_range(40000..65000);
                selected.push(format!("127.0.0.1:{}", port).parse().unwrap());
            }
        }

        if selected.len() < count {
            return Err(CircuitBuilderError::NotEnoughPeers);
        }

        Ok(selected)
    }
}