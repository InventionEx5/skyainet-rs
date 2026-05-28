// crates/governance/src/dao.rs
// =====================================================
// DAO v5.0 — Système de Gouvernance Décentralisée Avancé
// Propositions, Votes pondérés, Quorum dynamique, Exécution sécurisée
// Intégré avec PoSI, Rewards, NodeIdentity & Thevie
// =====================================================

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use thiserror::Error;

use crate::posi::PoSI;
use crate::rewards::UserRewards;
use skyainet_node::SkyAInetNode;

#[derive(Error, Debug)]
pub enum DaoError {
    #[error("Proposal not found: {0}")]
    ProposalNotFound(u64),
    #[error("Voting period has ended")]
    VotingEnded,
    #[error("Proposal already executed")]
    AlreadyExecuted,
    #[error("Insufficient quorum")]
    InsufficientQuorum,
    #[error("Proposal rejected by vote")]
    ProposalRejected,
    #[error("Invalid voter reputation")]
    InvalidReputation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: [u8; 32],
    pub votes_for: u128,
    pub votes_against: u128,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub executed: bool,
    pub status: ProposalStatus,
    pub quorum: u128,
    pub threshold: f64,           // Pourcentage minimum "pour"
    pub category: String,         // "governance", "treasury", "protocol", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dao {
    pub proposals: HashMap<u64, Proposal>,
    pub next_proposal_id: u64,
    pub default_quorum: u128,
    pub default_threshold: f64,
    pub active_voters: HashMap<[u8; 32], u128>, // voter → voting power
}

impl Dao {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            next_proposal_id: 1,
            default_quorum: 250,
            default_threshold: 0.62,
            active_voters: HashMap::new(),
        }
    }

    /// Crée une nouvelle proposition
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        proposer: [u8; 32],
        duration_days: u64,
        category: String,
    ) -> u64 {
        let id = self.next_proposal_id;
        let now = Utc::now();

        let proposal = Proposal {
            id,
            title,
            description,
            proposer,
            votes_for: 0,
            votes_against: 0,
            start_time: now,
            end_time: now + chrono::Duration::days(duration_days as i64),
            executed: false,
            status: ProposalStatus::Active,
            quorum: self.default_quorum,
            threshold: self.default_threshold,
            category,
        };

        self.proposals.insert(id, proposal);
        self.next_proposal_id += 1;

        info!("[DAO] Nouvelle proposition créée : {} (ID: {})", title, id);
        id
    }

    /// Vote pondéré par réputation et PoSI
    pub fn vote(
        &mut self,
        proposal_id: u64,
        voter: [u8; 32],
        in_favor: bool,
        voting_power: u128,
    ) -> Result<(), DaoError> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or(DaoError::ProposalNotFound(proposal_id))?;

        if Utc::now() > proposal.end_time {
            return Err(DaoError::VotingEnded);
        }

        if proposal.executed {
            return Err(DaoError::AlreadyExecuted);
        }

        if in_favor {
            proposal.votes_for += voting_power;
        } else {
            proposal.votes_against += voting_power;
        }

        debug!(
            "[DAO] Vote enregistré sur proposition {} | Voter: {} | Power: {} | Favor: {}",
            proposal_id, hex::encode(&voter[0..8]), voting_power, in_favor
        );

        Ok(())
    }

    /// Vérifie si une proposition peut être exécutée
    pub fn can_execute(&self, proposal_id: u64) -> Result<bool, DaoError> {
        let proposal = self.proposals.get(&proposal_id)
            .ok_or(DaoError::ProposalNotFound(proposal_id))?;

        if Utc::now() < proposal.end_time {
            return Ok(false);
        }

        if proposal.executed {
            return Err(DaoError::AlreadyExecuted);
        }

        let total_votes = proposal.votes_for + proposal.votes_against;
        if total_votes < proposal.quorum {
            return Err(DaoError::InsufficientQuorum);
        }

        let approval_rate = proposal.votes_for as f64 / total_votes as f64;
        if approval_rate < proposal.threshold {
            return Err(DaoError::ProposalRejected);
        }

        Ok(true)
    }

    /// Exécute une proposition validée
    pub async fn execute_proposal(
        &mut self,
        proposal_id: u64,
        node: &mut SkyAInetNode,
        rewards: &mut UserRewards,
    ) -> Result<(), DaoError> {
        if !self.can_execute(proposal_id)? {
            return Err(DaoError::ProposalRejected);
        }

        let proposal = self.proposals.get_mut(&proposal_id).unwrap();
        proposal.executed = true;
        proposal.status = ProposalStatus::Executed;

        // Récompense pour le proposeur
        rewards.add_reward(crate::rewards::RewardReason::GovernanceContribution, 120);

        info!(
            "[DAO] Proposition {} exécutée avec succès ({} pour / {} contre)",
            proposal_id, proposal.votes_for, proposal.votes_against
        );

        // TODO: Exécuter l'action réelle (mise à jour paramètres, treasury, etc.)
        Ok(())
    }

    pub fn get_active_proposals(&self) -> Vec<&Proposal> {
        self.proposals
            .values()
            .filter(|p| p.status == ProposalStatus::Active && Utc::now() < p.end_time)
            .collect()
    }

    pub fn get_proposal(&self, id: u64) -> Option<&Proposal> {
        self.proposals.get(&id)
    }
}

impl Default for Dao {
    fn default() -> Self {
        Self::new()
    }
}