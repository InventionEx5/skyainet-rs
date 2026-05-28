// crates/governance/src/conviction_voting.rs
// =====================================================
// Conviction Voting v5.0 — Vote par Conviction Avancé
// Plus tu maintiens ton vote, plus il pèse — Intégré avec DAO, PoSI & Rewards
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc};
use thiserror::Error;
use std::collections::HashMap;

use crate::dao::Proposal;
use crate::rewards::UserRewards;
use crate::posi::PoSI;

#[derive(Error, Debug)]
pub enum ConvictionError {
    #[error("Vote not found")]
    VoteNotFound,
    #[error("Voting period has ended")]
    VotingEnded,
    #[error("Conviction already locked")]
    AlreadyLocked,
    #[error("Invalid voting power")]
    InvalidPower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvictionVote {
    pub voter: [u8; 32],
    pub proposal_id: u64,
    pub base_weight: u128,
    pub start_time: DateTime<Utc>,
    pub conviction_multiplier: f64,
    pub final_weight: u128,
    pub is_locked: bool,
    pub direction: bool,           // true = For, false = Against
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvictionVoting {
    pub votes: HashMap<(u64, [u8; 32]), ConvictionVote>,
    pub conviction_period: u64,      // en secondes (ex: 7 jours = 604800)
    pub max_multiplier: f64,
    pub decay_rate: f64,
}

impl ConvictionVoting {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
            conviction_period: 604800,   // 7 jours par défaut
            max_multiplier: 4.0,         // Jusqu'à 4x le poids de base
            decay_rate: 0.0,
        }
    }

    /// Vote avec conviction (le poids augmente avec le temps)
    pub fn cast_vote(
        &mut self,
        proposal_id: u64,
        voter: [u8; 32],
        base_weight: u128,
        in_favor: bool,
    ) -> Result<(), ConvictionError> {
        if base_weight == 0 {
            return Err(ConvictionError::InvalidPower);
        }

        let key = (proposal_id, voter);

        if self.votes.contains_key(&key) {
            return Err(ConvictionError::AlreadyLocked);
        }

        let vote = ConvictionVote {
            voter,
            proposal_id,
            base_weight,
            start_time: Utc::now(),
            conviction_multiplier: 1.0,
            final_weight: base_weight,
            is_locked: true,
            direction: in_favor,
        };

        self.votes.insert(key, vote);

        info!(
            "[Conviction] Vote casted on proposal {} | Voter: {} | Base: {} | Direction: {}",
            proposal_id, hex::encode(&voter[0..8]), base_weight, if in_favor { "FOR" } else { "AGAINST" }
        );

        Ok(())
    }

    /// Calcule le poids actuel de conviction
    pub fn calculate_current_weight(&self, proposal_id: u64, voter: [u8; 32]) -> Result<u128, ConvictionError> {
        let key = (proposal_id, voter);
        let vote = self.votes.get(&key).ok_or(ConvictionError::VoteNotFound)?;

        let elapsed = (Utc::now() - vote.start_time).num_seconds() as u64;
        let time_ratio = (elapsed as f64 / self.conviction_period as f64).min(1.0);

        let multiplier = 1.0 + (time_ratio * (self.max_multiplier - 1.0));
        let current_weight = (vote.base_weight as f64 * multiplier) as u128;

        Ok(current_weight)
    }

    /// Met à jour tous les votes d’une proposition
    pub fn update_all_weights(&mut self, proposal_id: u64) {
        for ((pid, _), vote) in self.votes.iter_mut() {
            if *pid != proposal_id { continue; }

            let elapsed = (Utc::now() - vote.start_time).num_seconds() as u64;
            let time_ratio = (elapsed as f64 / self.conviction_period as f64).min(1.0);

            vote.conviction_multiplier = 1.0 + (time_ratio * (self.max_multiplier - 1.0));
            vote.final_weight = (vote.base_weight as f64 * vote.conviction_multiplier) as u128;
        }
    }

    /// Retourne les poids totaux (For / Against) d’une proposition
    pub fn get_proposal_weights(&self, proposal_id: u64) -> (u128, u128) {
        let mut for_weight = 0u128;
        let mut against_weight = 0u128;

        for ((pid, _), vote) in &self.votes {
            if *pid == proposal_id {
                if vote.direction {
                    for_weight += vote.final_weight;
                } else {
                    against_weight += vote.final_weight;
                }
            }
        }

        (for_weight, against_weight)
    }

    pub fn release_vote(&mut self, proposal_id: u64, voter: [u8; 32]) -> Result<(), ConvictionError> {
        let key = (proposal_id, voter);
        if self.votes.remove(&key).is_some() {
            debug!("[Conviction] Vote released for voter {:?}", voter);
            Ok(())
        } else {
            Err(ConvictionError::VoteNotFound)
        }
    }

    pub fn get_multiplier(&self, proposal_id: u64, voter: [u8; 32]) -> Result<f64, ConvictionError> {
        let key = (proposal_id, voter);
        let vote = self.votes.get(&key).ok_or(ConvictionError::VoteNotFound)?;
        Ok(vote.conviction_multiplier)
    }
}

impl Default for ConvictionVoting {
    fn default() -> Self {
        Self::new()
    }
}