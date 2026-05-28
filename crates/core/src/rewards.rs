// crates/core/src/rewards.rs
// =====================================================
// UserRewards System v6.6 — Version Légère Anti-Farming
// Limites journalières + Qualité minimale + Claim mensuel
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardReason {
    LearnContribution,
    DreamCycleParticipation,
    HighQualityInteraction,
    SubscriptionBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRewards {
    pub account_type: AccountType,

    pub daily_messages: u32,
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
            daily_messages: 0,
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

    // ==================== ANTI-FARMING LÉGER ====================

    pub fn get_max_daily_messages(&self) -> u32 {
        match self.account_type {
            AccountType::Free      => 80,
            AccountType::Pro       => 200,
            AccountType::NodeOwner => 450,
        }
    }

    fn can_earn_reward_today(&self) -> bool {
        self.daily_messages < self.get_max_daily_messages()
    }

    // ==================== ENREGISTREMENT ====================

    pub fn record_learn_contribution(&mut self, quality: f64) {
        if !self.can_earn_reward_today() || quality < 0.70 {
            return;
        }

        self.daily_messages += 1;
        self.total_learn_contributions += 1;
        self.conversation_quality_score = (self.conversation_quality_score * 0.84) + (quality * 0.16);
        self.conversation_quality_score = self.conversation_quality_score.clamp(0.1, 1.0);
        self.last_reward_date = Some(Utc::now());

        self.add_pending_reward(RewardReason::LearnContribution, quality);
    }

    pub fn record_dream_cycle(&mut self, quality: f64) {
        if !self.can_earn_reward_today() || quality < 0.70 {
            return;
        }

        self.daily_messages += 1;
        self.total_dream_cycles += 1;
        self.thevie_evolution_contribution = (self.thevie_evolution_contribution + quality * 0.11).min(1.0);
        self.last_reward_date = Some(Utc::now());

        self.add_pending_reward(RewardReason::DreamCycleParticipation, quality);
    }

    pub fn record_high_quality_interaction(&mut self, quality: f64) {
        if !self.can_earn_reward_today() || quality < 0.70 {
            return;
        }

        self.daily_messages += 1;
        self.high_quality_interactions += 1;
        self.conversation_quality_score = (self.conversation_quality_score * 0.82) + (quality * 0.18);
        self.last_reward_date = Some(Utc::now());

        self.add_pending_reward(RewardReason::HighQualityInteraction, quality);
    }

    fn add_pending_reward(&mut self, reason: RewardReason, quality: f64) {
        let base = match reason {
            RewardReason::LearnContribution => 12,
            RewardReason::DreamCycleParticipation => 22,
            RewardReason::HighQualityInteraction => 8,
            RewardReason::SubscriptionBonus => 50,
        };

        let amount = (base as f64 * quality) as u128;
        self.pending_rewards += amount;
    }

    // ==================== CLAIM MENSUEL ====================

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

// =====================================================
// TESTS UNITAIRES
// =====================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_account_learn_contribution() {
        let mut rewards = UserRewards::new(AccountType::Free);
        
        rewards.record_learn_contribution(0.85);
        
        assert_eq!(rewards.total_learn_contributions, 1);
        assert!(rewards.pending_rewards > 0);
        assert!(rewards.conversation_quality_score > 0.65);
    }

    #[test]
    fn test_quality_threshold() {
        let mut rewards = UserRewards::new(AccountType::Free);
        
        rewards.record_learn_contribution(0.65); // En dessous du seuil
        
        assert_eq!(rewards.total_learn_contributions, 0);
        assert_eq!(rewards.pending_rewards, 0);
    }

    #[test]
    fn test_daily_limit_free() {
        let mut rewards = UserRewards::new(AccountType::Free);
        
        for _ in 0..85 {
            rewards.record_learn_contribution(0.85);
        }
        
        assert_eq!(rewards.daily_messages, 80); // Limite atteinte
    }

    #[test]
    fn test_pro_account_higher_rewards() {
        let mut free = UserRewards::new(AccountType::Free);
        let mut pro = UserRewards::new(AccountType::Pro);

        free.record_learn_contribution(0.90);
        pro.record_learn_contribution(0.90);

        assert!(pro.pending_rewards > free.pending_rewards);
    }

    #[test]
    fn test_claim_monthly_rewards() {
        let mut rewards = UserRewards::new(AccountType::Pro);
        
        rewards.record_dream_cycle(0.88);
        rewards.record_learn_contribution(0.82);
        
        let claimed = rewards.claim_monthly_rewards();
        
        assert!(claimed > 0);
        assert_eq!(rewards.pending_rewards, 0);
        assert_eq!(rewards.total_sky_earned, claimed);
    }

    #[test]
    fn test_subscription_bonus() {
        let pro = UserRewards::new(AccountType::Pro);
        let node = UserRewards::new(AccountType::NodeOwner);
        let free = UserRewards::new(AccountType::Free);

        assert_eq!(pro.get_subscription_bonus(), 35);
        assert_eq!(node.get_subscription_bonus(), 75);
        assert_eq!(free.get_subscription_bonus(), 0);
    }

    #[test]
    fn test_dream_cycle_increases_evolution() {
        let mut rewards = UserRewards::new(AccountType::Free);
        
        let initial = rewards.thevie_evolution_contribution;
        rewards.record_dream_cycle(0.90);
        
        assert!(rewards.thevie_evolution_contribution > initial);
    }
}