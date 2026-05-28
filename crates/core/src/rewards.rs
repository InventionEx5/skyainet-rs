// crates/core/src/rewards.rs
// =====================================================
// UserRewards System v6.3 — Version Simplifiée & Équilibrée
// Rewards sur Learn + Dream + High Quality + Bonus Abonnement
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardReason {
    HighQualityInteraction,
    LearnContribution,
    DreamCycleParticipation,
    SubscriptionBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardEntry {
    pub reason: RewardReason,
    pub amount: u128,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRewards {
    pub account_type: AccountType,

    pub total_learn_contributions: u64,
    pub total_dream_cycles: u64,
    pub high_quality_interactions: u64,

    pub conversation_quality_score: f64,
    pub thevie_evolution_contribution: f64,

    pub total_sky_earned: u128,
    pub pending_rewards: u128,
    pub last_reward_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountType {
    Free,
    Pro,
    NodeOwner,
}

impl UserRewards {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            total_learn_contributions: 0,
            total_dream_cycles: 0,
            high_quality_interactions: 0,
            conversation_quality_score: 0.65,
            thevie_evolution_contribution: 0.25,
            total_sky_earned: 0,
            pending_rewards: 0,
            last_reward_date: None,
        }
    }

    pub fn record_learn_contribution(&mut self, quality: f64) {
        self.total_learn_contributions += 1;
        self.conversation_quality_score = (self.conversation_quality_score * 0.84) + (quality * 0.16);
        self.conversation_quality_score = self.conversation_quality_score.clamp(0.1, 1.0);
        self.last_reward_date = Some(Utc::now());

        let amount = (12.0 * quality) as u128;
        self.pending_rewards += amount;
    }

    pub fn record_dream_cycle(&mut self, quality: f64) {
        self.total_dream_cycles += 1;
        self.thevie_evolution_contribution = (self.thevie_evolution_contribution + quality * 0.11).min(1.0);
        self.last_reward_date = Some(Utc::now());

        let amount = (22.0 * quality) as u128;
        self.pending_rewards += amount;
    }

    pub fn record_high_quality_interaction(&mut self, quality: f64) {
        self.high_quality_interactions += 1;
        self.conversation_quality_score = (self.conversation_quality_score * 0.82) + (quality * 0.18);
        self.last_reward_date = Some(Utc::now());

        let amount = (8.0 * quality) as u128;
        self.pending_rewards += amount;
    }

    pub fn claim_monthly_rewards(&mut self) -> u128 {
        if self.pending_rewards == 0 {
            return 0;
        }

        let amount = self.pending_rewards;
        self.total_sky_earned += amount;
        self.pending_rewards = 0;
        self.last_reward_date = Some(Utc::now());

        info!("[Rewards] Monthly claim: {} SKY", amount);
        amount
    }

    pub fn get_subscription_bonus(&self) -> u128 {
        match self.account_type {
            AccountType::Pro       => 35,
            AccountType::NodeOwner => 75,
            _ => 0,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Rewards | Type: {:?} | Pending: {} SKY | Quality: {:.2} | Learn: {} | Dream: {}",
            self.account_type,
            self.pending_rewards,
            self.conversation_quality_score,
            self.total_learn_contributions,
            self.total_dream_cycles
        )
    }
}