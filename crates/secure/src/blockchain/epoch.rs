// crates/secure/src/blockchain/epoch.rs
// =====================================================
// Epoch Manager v6.0 — Gestion des Époques + Rekeying Sécurisé
// SkyAInet × Nikola T369 — Intégration Dilithium5 + Hybrid Crypto
// Version Ultra Améliorée (Production Ready)
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, error};
use thiserror::Error;

use crate::crypto::dilithium::Dilithium5Signer;
use crate::blockchain::node_identity::NodeIdentity;

#[derive(Error, Debug)]
pub enum EpochError {
    #[error("Epoch advance failed: {0}")]
    AdvanceFailed(String),
    #[error("Rekeying consensus not reached")]
    ConsensusNotReached,
    #[error("Invalid epoch state")]
    InvalidState,
    #[error("Key rotation failed: {0}")]
    KeyRotationFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EpochStatus {
    Active,
    Rekeying,
    Finalizing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochManager {
    pub current_epoch: u64,
    pub epoch_duration: u64,           // en secondes
    pub last_epoch_start: DateTime<Utc>,
    pub status: EpochStatus,
    pub rekey_threshold: f64,          // % de nœuds nécessaires pour consensus (ex: 0.66)
    pub total_rekey_votes: u64,
    pub min_participants: usize,       // Nombre minimum de participants pour valider le consensus
}

impl EpochManager {
    pub fn new(duration_seconds: u64, rekey_threshold: f64) -> Self {
        Self {
            current_epoch: 0,
            epoch_duration: duration_seconds,
            last_epoch_start: Utc::now(),
            status: EpochStatus::Active,
            rekey_threshold,
            total_rekey_votes: 0,
            min_participants: 3, // Minimum 3 nœuds pour consensus
        }
    }

    /// Vérifie si on doit passer à l'epoch suivant
    pub fn should_advance_epoch(&self) -> bool {
        let now = Utc::now();
        let elapsed = (now - self.last_epoch_start).num_seconds() as u64;
        elapsed >= self.epoch_duration
    }

    /// Avance à l'epoch suivant (avec rekeying si nécessaire)
    pub fn advance_epoch(&mut self) -> Result<(), EpochError> {
        if self.status != EpochStatus::Active {
            return Err(EpochError::InvalidState);
        }

        self.current_epoch += 1;
        self.last_epoch_start = Utc::now();
        self.status = EpochStatus::Rekeying;
        self.total_rekey_votes = 0;

        info!("[EpochManager] Nouvel epoch: {} → Status: Rekeying", self.current_epoch);
        Ok(())
    }

    /// Enregistre un vote pour le rekeying (consensus-aware)
    pub fn vote_for_rekey(&mut self, node_id: [u8; 32], total_nodes: usize) -> bool {
        if total_nodes < self.min_participants {
            warn!("[EpochManager] Pas assez de participants pour consensus");
            return false;
        }

        self.total_rekey_votes += 1;

        let consensus_ratio = self.total_rekey_votes as f64 / total_nodes as f64;

        if consensus_ratio >= self.rekey_threshold {
            self.status = EpochStatus::Finalizing;
            info!(
                "[EpochManager] Consensus rekeying atteint ({}/{} → {:.1}%) → Finalizing epoch {}",
                self.total_rekey_votes, total_nodes, consensus_ratio * 100.0, self.current_epoch
            );
            true
        } else {
            debug!(
                "[EpochManager] Vote rekeying: {}/{} ({:.1}%)",
                self.total_rekey_votes, total_nodes, consensus_ratio * 100.0
            );
            false
        }
    }

    /// Finalise l'epoch et effectue le rekeying global (rotation réelle des clés)
    pub fn finalize_epoch(&mut self, identities: &mut [NodeIdentity]) -> Result<(), EpochError> {
        if self.status != EpochStatus::Finalizing {
            return Err(EpochError::InvalidState);
        }

        let mut successful_rotations = 0;

        for identity in identities.iter_mut() {
            match identity.rotate_public_key() {
                Ok(_) => {
                    successful_rotations += 1;
                    debug!("[EpochManager] Clé rotée avec succès pour {:?}", identity.node_id);
                }
                Err(e) => {
                    error!("[EpochManager] Échec rotation clé pour {:?}: {}", identity.node_id, e);
                }
            }
        }

        if successful_rotations == 0 {
            return Err(EpochError::KeyRotationFailed("Aucune clé n'a pu être rotée".to_string()));
        }

        self.status = EpochStatus::Active;
        self.total_rekey_votes = 0;

        info!(
            "[EpochManager] Epoch {} finalisé → Rekeying terminé ({} nœuds sur {})",
            self.current_epoch,
            successful_rotations,
            identities.len()
        );

        Ok(())
    }

    /// Retourne le temps restant avant le prochain epoch (en secondes)
    pub fn time_until_next_epoch(&self) -> i64 {
        let now = Utc::now();
        let elapsed = (now - self.last_epoch_start).num_seconds();
        (self.epoch_duration as i64 - elapsed).max(0)
    }

    /// Vérifie si on est en période de rekeying
    pub fn is_rekeying_period(&self) -> bool {
        matches!(self.status, EpochStatus::Rekeying | EpochStatus::Finalizing)
    }
}

impl Default for EpochManager {
    fn default() -> Self {
        Self::new(3600, 0.66) // 1 heure par défaut, 66% de consensus
    }
}