// crates/node/src/validator.rs
// =====================================================
// ValidatorNode v4.0 — Nœud de Validation Souverain
// Staking + Consensus Avancé + PoUW + Réputation Dynamique
// Intégré avec Rewards, Dream Cycle, ZipMemory & Governance
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};

use skyainet_core::node_types::{NodeCapabilities, NodeState, SubscriptionLevel, NodeType};
use crate::pouw::ContributionProof;
use crate::rewards::UserRewards;
use skyainet_memory::zip_memory::ZipMemory;

/// Nœud Validateur Avancé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorNode {
    pub node_id: String,
    pub sovereign_alias: String,
    pub capabilities: NodeCapabilities,
    pub current_state: NodeState,
    
    // === Économie & Staking ===
    pub stake_amount: u64,
    pub locked_until: Option<DateTime<Utc>>,
    pub reputation: f32,                    // 0.0 → 1.0
    pub validated_contributions: u64,
    pub failed_validations: u64,
    pub last_validation: u64,
    
    // === Performance & Historique ===
    pub total_validation_score: f64,
    pub consecutive_success: u32,
    pub slash_count: u32,
    
    // === Intégration ZipMemory ===
    #[serde(skip)]
    pub validation_cache: Option<ZipMemory>,
}

impl ValidatorNode {
    pub fn new(sovereign_alias: &str, initial_stake: u64) -> Self {
        let mut capabilities = NodeCapabilities::new(&SubscriptionLevel::Validator);
        capabilities.compute_power = 0.97;
        capabilities.validation_power = 1.0;
        capabilities.bandwidth = 0.92;

        Self {
            node_id: format!("val-{}", uuid::Uuid::new_v4().simple()),
            sovereign_alias: sovereign_alias.to_string(),
            capabilities,
            current_state: NodeState::Active,
            stake_amount: initial_stake,
            locked_until: None,
            reputation: 0.78,
            validated_contributions: 0,
            failed_validations: 0,
            last_validation: crate::utils::now_millis(),
            total_validation_score: 0.0,
            consecutive_success: 0,
            slash_count: 0,
            validation_cache: Some(ZipMemory::new(&format!("./data/validator/{}_cache", sovereign_alias))),
        }
    }

    /// Validation avancée avec scoring intelligent + ZipMemory
    pub async fn validate_contribution(&mut self, proof: &ContributionProof, rewards: &mut UserRewards) -> Result<bool, String> {
        if self.stake_amount < 8000 {
            return Err("Stake insuffisant pour valider (minimum 8000 SKY)".to_string());
        }

        let base_score = proof.score;
        let reputation_multiplier = self.reputation.clamp(0.4, 1.2);
        let final_score = base_score * reputation_multiplier;

        let is_valid = final_score >= 0.82 && self.reputation > 0.62;

        if is_valid {
            self.validated_contributions += 1;
            self.consecutive_success += 1;
            self.reputation = (self.reputation + 0.018 * (self.consecutive_success as f32 / 10.0)).min(0.995);
            self.total_validation_score += final_score as f64;
            self.last_validation = crate::utils::now_millis();

            // Récompense légère
            rewards.add_reward(crate::rewards::RewardReason::Validation, 8);
            
            if let Some(cache) = &mut self.validation_cache {
                let _ = cache.save_compressed(&proof.proof_id, &proof.data);
            }

            debug!("[Validator] Contribution validée avec score {:.3}", final_score);
        } else {
            self.failed_validations += 1;
            self.consecutive_success = 0;
            self.reputation = (self.reputation - 0.035).max(0.25);
            self.slash(0.08);
            warn!("[Validator] Contribution rejetée (score {:.3})", final_score);
        }

        Ok(is_valid)
    }

    /// Participation au consensus avec poids dynamique
    pub fn participate_in_consensus(&self, proposal_weight: f64) -> f64 {
        let stake_factor = (self.stake_amount as f64 / 10000.0).min(8.0);
        let rep_factor = self.reputation as f64;
        (proposal_weight * stake_factor * rep_factor * 0.75).min(120.0)
    }

    /// Slashing intelligent avec impact sur la gouvernance
    pub fn slash(&mut self, percentage: f64) {
        let slash_amount = (self.stake_amount as f64 * percentage).min(self.stake_amount as f64 * 0.25) as u64;
        self.stake_amount = self.stake_amount.saturating_sub(slash_amount);
        self.reputation = (self.reputation - 0.12).max(0.18);
        self.slash_count += 1;

        if self.slash_count >= 5 {
            self.current_state = NodeState::Sleeping;
            warn!("[Validator] Nœud mis en veille après multiples slashes");
        }
    }

    pub fn is_eligible_for_governance(&self) -> bool {
        self.stake_amount >= 15000 && self.reputation >= 0.85 && self.consecutive_success >= 12
    }

    pub fn get_node_type(&self) -> NodeType {
        NodeType::Validator
    }

    /// Rapport complet pour monitoring
    pub fn health_report(&self) -> String {
        format!(
            "Validator {} | Stake: {} SKY | Rep: {:.3} | Validated: {} | Failed: {} | Consecutive: {} | State: {:?}",
            self.sovereign_alias, self.stake_amount, self.reputation,
            self.validated_contributions, self.failed_validations,
            self.consecutive_success, self.current_state
        )
    }
}