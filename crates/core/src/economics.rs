// crates/core/src/economics.rs
// =====================================================
// NodeEconomics v6.6 — Gestion des Abonnements + Rewards
// Intégré avec le système UserRewards (Learn, Dream, Stacking)
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::info;
use chrono::{DateTime, Utc};

use crate::rewards::{UserRewards, AccountType, RewardReason};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEconomics {
    pub active_subscriptions: Vec<SubscriptionType>,
    pub is_rented_out: bool,
    pub rental_price_per_hour_sky: u64,

    pub total_earned_sky: u128,
    pub last_payout: Option<DateTime<Utc>>,

    // Lien avec le système de rewards
    pub user_rewards: UserRewards,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionType {
    Node(NodeTier),
    Gateway(GatewayPlan),
    ApiKeys(ApiKeysPlan),
    Storage(StoragePlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeTier {
    Mini, Light, Full, DreamWeaver, Validator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GatewayPlan { Basic, Pro, Sovereign }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiKeysPlan { Free, Developer, Enterprise }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StoragePlan { Basic, Pro, Enterprise }

impl NodeEconomics {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            active_subscriptions: vec![SubscriptionType::Node(NodeTier::Mini)],
            is_rented_out: false,
            rental_price_per_hour_sky: 0,
            total_earned_sky: 0,
            last_payout: None,
            user_rewards: UserRewards::new(account_type),
        }
    }

    // ==================== ABONNEMENTS ====================

    pub fn add_subscription(&mut self, sub: SubscriptionType) {
        if !self.active_subscriptions.contains(&sub) {
            self.active_subscriptions.push(sub);
            info!("[Economics] Abonnement ajouté : {:?}", sub);
        }
    }

    pub fn get_total_monthly_cost(&self) -> u64 {
        self.active_subscriptions.iter().map(|s| s.monthly_price_eur()).sum()
    }

    // ==================== REWARDS ====================

    pub fn record_learn_contribution(&mut self, quality: f64) {
        self.user_rewards.record_learn_contribution(quality);
    }

    pub fn record_dream_cycle(&mut self, quality: f64) {
        self.user_rewards.record_dream_cycle(quality);
    }

    pub fn record_high_quality_interaction(&mut self, quality: f64) {
        self.user_rewards.record_high_quality_interaction(quality);
    }

    pub fn claim_monthly_rewards(&mut self) -> u128 {
        let amount = self.user_rewards.claim_monthly_rewards();
        self.total_earned_sky += amount;
        amount
    }

    pub fn get_subscription_bonus(&self) -> u128 {
        self.user_rewards.get_subscription_bonus()
    }

    pub fn is_eligible_for_rewards(&self) -> bool {
        self.user_rewards.is_eligible_for_rewards()
    }

    pub fn summary(&self) -> String {
        format!(
            "Economics | Subs: {} | Monthly Cost: {}€ | Earned: {} SKY | Pending: {} SKY | Quality: {:.2}",
            self.active_subscriptions.len(),
            self.get_total_monthly_cost(),
            self.total_earned_sky,
            self.user_rewards.pending_rewards,
            self.user_rewards.conversation_quality_score
        )
    }
}

// ==================== IMPLÉMENTATIONS DES PLANS ====================

impl SubscriptionType {
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            SubscriptionType::Node(tier) => tier.monthly_price_eur(),
            SubscriptionType::Gateway(plan) => plan.monthly_price_eur(),
            SubscriptionType::ApiKeys(plan) => plan.monthly_price_eur(),
            SubscriptionType::Storage(plan) => plan.monthly_price_eur(),
        }
    }
}

impl NodeTier {
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            NodeTier::Mini        => 0,
            NodeTier::Light       => 6,
            NodeTier::Full        => 18,
            NodeTier::DreamWeaver => 32,
            NodeTier::Validator   => 55,
        }
    }
}

impl GatewayPlan {
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            GatewayPlan::Basic     => 9,
            GatewayPlan::Pro       => 19,
            GatewayPlan::Sovereign => 39,
        }
    }
}

impl ApiKeysPlan {
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            ApiKeysPlan::Free       => 0,
            ApiKeysPlan::Developer  => 12,
            ApiKeysPlan::Enterprise => 49,
        }
    }
}

impl StoragePlan {
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            StoragePlan::Basic      => 8,
            StoragePlan::Pro        => 25,
            StoragePlan::Enterprise => 79,
        }
    }
}