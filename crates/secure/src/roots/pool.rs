// crates/secure/src/roots/pool.rs
// =====================================================
// PeerPool v6.1 — Gestion Intelligente des Pairs
// Compatible Contact v6.2 + DID + RomanT369 + GroupManager
// DiamantRoots v2 — Sélection par Réputation + Diversité
// SkyAInet × Nikola T369
// =====================================================

use std::collections::HashMap;
use std::net::SocketAddr;
use rand::seq::SliceRandom;
use tracing::{info, debug, warn};
use thiserror::Error;

use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;
use crate::roots::reputation::{PeerReputation, ReputationTier};

#[derive(Error, Debug)]
pub enum PeerPoolError {
    #[error("Peer not found")]
    PeerNotFound,
    #[error("Not enough peers available (requested: {requested}, available: {available})")]
    NotEnoughPeers { requested: usize, available: usize },
    #[error("Peer reputation too low")]
    ReputationTooLow,
    #[error("Contact not verified (DID required)")]
    ContactNotVerified,
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub addr: SocketAddr,
    pub reputation: PeerReputation,
    pub last_seen: u64,
    pub connection_count: u32,
    pub contact_id: Option<[u8; 32]>,   // Lien vers Contact + DID
}

pub struct PeerPool {
    peers: HashMap<[u8; 32], PeerInfo>,
    min_reputation_threshold: f64,
}

impl PeerPool {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
            min_reputation_threshold: 0.60,
        }
    }

    pub fn with_min_reputation(mut self, threshold: f64) -> Self {
        self.min_reputation_threshold = threshold;
        self
    }

    // =====================================================
    // GESTION DES PAIRS
    // =====================================================

    /// Ajoute ou met à jour un pair (version simple)
    pub fn add_peer(&mut self, node_id: [u8; 32], addr: SocketAddr) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.peers.insert(
            node_id,
            PeerInfo {
                addr,
                reputation: PeerReputation::new(),
                last_seen: now,
                connection_count: 0,
                contact_id: None,
            },
        );

        debug!(
            "[PeerPool] Pair ajouté : {} ({})",
            hex::encode(&node_id[0..8]),
            addr
        );
    }

    /// Ajoute un pair avec lien vers un Contact (DID)
    pub fn add_peer_with_contact(
        &mut self,
        node_id: [u8; 32],
        addr: SocketAddr,
        contact: Option<&Contact>,
    ) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let contact_id = contact.map(|c| c.node_id);

        self.peers.insert(
            node_id,
            PeerInfo {
                addr,
                reputation: PeerReputation::new().with_contact(node_id),
                last_seen: now,
                connection_count: 0,
                contact_id,
            },
        );

        debug!(
            "[PeerPool] Pair ajouté avec Contact : {} ({})",
            hex::encode(&node_id[0..8]),
            addr
        );
    }

    pub fn remove_peer(&mut self, node_id: &[u8; 32]) -> bool {
        if self.peers.remove(node_id).is_some() {
            debug!("[PeerPool] Pair supprimé : {}", hex::encode(&node_id[0..8]));
            true
        } else {
            false
        }
    }

    pub fn get_peer(&self, node_id: &[u8; 32]) -> Option<SocketAddr> {
        self.peers.get(node_id).map(|info| info.addr)
    }

    pub fn contains(&self, node_id: &[u8; 32]) -> bool {
        self.peers.contains_key(node_id)
    }

    pub fn len(&self) -> usize {
        self.peers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    // =====================================================
    // SÉLECTION INTELLIGENTE (avec DID)
    // =====================================================

    pub fn get_random_peers(&self, count: usize) -> Result<Vec<SocketAddr>, PeerPoolError> {
        if self.peers.is_empty() {
            return Err(PeerPoolError::NotEnoughPeers { requested: count, available: 0 });
        }

        let mut peers: Vec<_> = self.peers.values().map(|info| info.addr).collect();
        let mut rng = rand::thread_rng();
        peers.shuffle(&mut rng);

        Ok(peers.into_iter().take(count).collect())
    }

    pub fn get_high_reputation_peers(
        &self,
        count: usize,
        min_reputation: Option<f64>,
        contact_manager: Option<&ContactManager>,
    ) -> Result<Vec<SocketAddr>, PeerPoolError> {
        let threshold = min_reputation.unwrap_or(self.min_reputation_threshold);

        let filtered: Vec<_> = self
            .peers
            .values()
            .filter(|info| {
                let base_ok = info.reputation.score >= threshold;

                if let (Some(manager), Some(cid)) = (contact_manager, info.contact_id) {
                    if let Some(contact) = manager.get(&cid) {
                        return base_ok && contact.has_decentralized_identity();
                    }
                }
                base_ok
            })
            .map(|info| info.addr)
            .collect();

        if filtered.len() < count {
            return Err(PeerPoolError::NotEnoughPeers {
                requested: count,
                available: filtered.len(),
            });
        }

        let mut rng = rand::thread_rng();
        let mut selected = filtered;
        selected.shuffle(&mut rng);

        Ok(selected.into_iter().take(count).collect())
    }

    pub fn get_diverse_peers(&self, count: usize) -> Result<Vec<SocketAddr>, PeerPoolError> {
        self.get_high_reputation_peers(count, None, None)
    }

    pub fn get_trusted_peers(&self, count: usize, contact_manager: &ContactManager) -> Result<Vec<SocketAddr>, PeerPoolError> {
        let filtered: Vec<_> = self
            .peers
            .values()
            .filter(|info| {
                if let Some(cid) = info.contact_id {
                    if let Some(contact) = contact_manager.get(&cid) {
                        return contact.has_decentralized_identity() && contact.verification_level >= 2;
                    }
                }
                false
            })
            .map(|info| info.addr)
            .collect();

        if filtered.len() < count {
            return Err(PeerPoolError::NotEnoughPeers {
                requested: count,
                available: filtered.len(),
            });
        }

        let mut rng = rand::thread_rng();
        let mut selected = filtered;
        selected.shuffle(&mut rng);

        Ok(selected.into_iter().take(count).collect())
    }

    // =====================================================
    // MISE À JOUR DE LA RÉPUTATION
    // =====================================================

    pub fn update_reputation(&mut self, node_id: &[u8; 32], new_score: f64) -> Result<(), PeerPoolError> {
        if let Some(info) = self.peers.get_mut(node_id) {
            info.reputation.score = new_score.clamp(0.0, 1.0);
            debug!(
                "[PeerPool] Réputation mise à jour pour {} : {:.2}",
                hex::encode(&node_id[0..8]),
                new_score
            );
            Ok(())
        } else {
            Err(PeerPoolError::PeerNotFound)
        }
    }

    pub fn get_peers_by_reputation(&self) -> Vec<(SocketAddr, f64)> {
        let mut list: Vec<_> = self
            .peers
            .values()
            .map(|info| (info.addr, info.reputation.score))
            .collect();

        list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        list
    }

    pub fn increment_connection(&mut self, node_id: &[u8; 32]) -> Result<(), PeerPoolError> {
        if let Some(info) = self.peers.get_mut(node_id) {
            info.connection_count += 1;
            info.last_seen = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Ok(())
        } else {
            Err(PeerPoolError::PeerNotFound)
        }
    }
}