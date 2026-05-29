// crates/secure/src/roots/reputation.rs
// =====================================================
// PeerReputation v6.1 — Système de Réputation Avancé
// Compatible Contact v6.2 + DID + RomanT369 + GroupManager
// DiamantRoots v2 — Évaluation Dynamique des Nœuds
// SkyAInet × Nikola T369
// =====================================================

use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use tracing::{debug, info, warn};
use thiserror::Error;

use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;

#[derive(Error, Debug)]
pub enum ReputationError {
    #[error("Invalid reputation score")]
    InvalidScore,
    #[error("Contact not verified")]
    ContactNotVerified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReputation {
    pub score: f64,                    // 0.0 → 1.0
    pub last_updated: u64,
    pub history: VecDeque<f64>,        // Historique des 10 dernières mises à jour
    pub successful_interactions: u32,
    pub failed_interactions: u32,
    pub contact_id: Option<[u8; 32]>,  // Lien vers le Contact (DID)
}

impl PeerReputation {
    pub fn new() -> Self {
        Self {
            score: 0.65,
            last_updated: Self::now(),
            history: VecDeque::with_capacity(10),
            successful_interactions: 0,
            failed_interactions: 0,
            contact_id: None,
        }
    }

    pub fn with_contact(mut self, contact_id: [u8; 32]) -> Self {
        self.contact_id = Some(contact_id);
        self
    }

    #[inline]
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Met à jour le score de réputation
    pub fn update(&mut self, delta: f64) -> Result<(), ReputationError> {
        let new_score = (self.score + delta).clamp(0.0, 1.0);

        if (new_score - self.score).abs() < 0.001 {
            return Ok(());
        }

        self.history.push_back(self.score);
        if self.history.len() > 10 {
            self.history.pop_front();
        }

        self.score = new_score;
        self.last_updated = Self::now();

        debug!(
            "[Reputation] Score mis à jour : {:.3} → {:.3} (delta: {:+.3})",
            self.score - delta, self.score, delta
        );

        Ok(())
    }

    /// Enregistre une interaction réussie (renforcée si le contact a un DID)
    pub fn record_success(&mut self, impact: f64, contact: Option<&Contact>) {
        self.successful_interactions += 1;

        let mut final_impact = impact.max(0.01);

        // Bonus si le contact a un DID vérifié
        if let Some(c) = contact {
            if c.has_decentralized_identity() && c.verification_level >= 2 {
                final_impact *= 1.15; // +15% de bonus
            }
        }

        let _ = self.update(final_impact);
    }

    /// Enregistre une interaction échouée
    pub fn record_failure(&mut self, impact: f64) {
        self.failed_interactions += 1;
        let _ = self.update(-impact.abs().max(0.01));
    }

    /// Applique une décroissance naturelle (anti-inactivité)
    pub fn apply_decay(&mut self, decay_rate: f64) {
        if self.score > 0.3 {
            self.score *= decay_rate;
            self.score = self.score.clamp(0.0, 1.0);
        }
    }

    /// Retourne le niveau de réputation
    pub fn tier(&self) -> ReputationTier {
        ReputationTier::from_score(self.score)
    }

    /// Score pondéré (historique + actuel)
    pub fn weighted_score(&self) -> f64 {
        if self.history.is_empty() {
            return self.score;
        }

        let avg_history: f64 = self.history.iter().sum::<f64>() / self.history.len() as f64;
        (self.score * 0.7) + (avg_history * 0.3)
    }

    /// Vérifie si le nœud est considéré comme fiable
    pub fn is_trusted(&self, contact_manager: Option<&ContactManager>) -> bool {
        let base_trust = self.score >= 0.75 && self.successful_interactions > 5;

        if let Some(manager) = contact_manager {
            if let Some(id) = self.contact_id {
                if let Some(contact) = manager.get(&id) {
                    return base_trust && contact.has_decentralized_identity();
                }
            }
        }

        base_trust
    }
}

impl Default for PeerReputation {
    fn default() -> Self {
        Self::new()
    }
}

/// Niveaux de réputation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReputationTier {
    Newcomer,
    Reliable,
    Trusted,
    Elite,
    Legendary,
}

impl ReputationTier {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 0.92 => ReputationTier::Legendary,
            s if s >= 0.82 => ReputationTier::Elite,
            s if s >= 0.70 => ReputationTier::Trusted,
            s if s >= 0.55 => ReputationTier::Reliable,
            _ => ReputationTier::Newcomer,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ReputationTier::Newcomer => "Newcomer",
            ReputationTier::Reliable => "Reliable",
            ReputationTier::Trusted => "Trusted",
            ReputationTier::Elite => "Elite",
            ReputationTier::Legendary => "Legendary",
        }
    }
}