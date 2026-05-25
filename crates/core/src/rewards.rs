// crates/core/src/rewards.rs
// =====================================================
// User Rewards System v5.1 — 15% Burn + Limites Compétitives
// SkyAInet × Thevie
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    Free,
    NodeOwner,
    VeryEngaged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRewards {
    pub account_type: AccountType,
    pub daily_messages: u32,
    pub total_messages: u64,
    pub conversation_quality_score: f64,
    pub thevie_evolution_contribution: f64,
    pub total_sky_earned: u128,
    pub last_reward_date: Option<DateTime<Utc>>,
    pub total_burned: u128,
}

impl UserRewards {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            daily_messages: 0,
            total_messages: 0,
            conversation_quality_score: 0.5,
            thevie_evolution_contribution: 0.0,
            total_sky_earned: 0,
            last_reward_date: None,
            total_burned: 0,
        }
    }

    pub fn record_message(&mut self) {
        let today = Utc::now().date_naive();

        if let Some(last) = self.last_reward_date {
            if last.date_naive() != today {
                self.daily_messages = 0;
            }
        }

        self.daily_messages += 1;
        self.total_messages += 1;
        self.last_reward_date = Some(Utc::now());
    }

    pub fn update_quality_score(&mut self, message_length: usize, rating: Option<u8>) {
        let length_factor = (message_length as f64 / 450.0).min(1.0);
        let feedback_bonus = if let Some(r) = rating {
            match r {
                5 => 0.20,
                4 => 0.12,
                3 => 0.05,
                _ => 0.0,
            }
        } else { 0.0 };

        self.conversation_quality_score = 
            (self.conversation_quality_score * 0.82) + (length_factor * 0.12) + feedback_bonus;
        self.conversation_quality_score = self.conversation_quality_score.clamp(0.1, 1.0);
    }

    /// Calcule la récompense avec **15% Burn**
    pub fn calculate_daily_reward(&self) -> (u128, u128) {
        if self.daily_messages == 0 {
            return (0, 0);
        }

        // === LIMITES ANTI-FARMING COMPÉTITIVES ===
        let max_daily = match self.account_type {
            AccountType::Free => 50,           // 50 messages/jour (compétitif)
            AccountType::VeryEngaged => 90,
            AccountType::NodeOwner => 250,
        };

        if self.daily_messages > max_daily {
            return (0, 0);
        }

        let base_reward: u128 = match self.account_type {
            AccountType::Free => 7,
            AccountType::VeryEngaged => 20,
            AccountType::NodeOwner => 40,
        };

        let quality_multiplier = 1.0 + (self.conversation_quality_score * 1.4);
        let evolution_bonus = (self.thevie_evolution_contribution * 28.0) as u128;

        let gross_reward = ((base_reward as f64 * quality_multiplier) as u128 + evolution_bonus).min(350);

        // === 15% BURN (comme dans le TreasuryVault) ===
        let burn_amount = (gross_reward as f64 * 0.15) as u128;
        let net_reward = gross_reward - burn_amount;

        (net_reward, burn_amount)
    }

    pub fn claim_daily_reward(&mut self) -> (u128, u128) {
        let (net_reward, burn_amount) = self.calculate_daily_reward();

        if net_reward > 0 {
            self.total_sky_earned += net_reward;
            self.total_burned += burn_amount;

            if self.conversation_quality_score > 0.78 {
                self.thevie_evolution_contribution = (self.thevie_evolution_contribution + 0.03).min(1.0);
            }

            self.daily_messages = 0;
        }

        (net_reward, burn_amount)
    }

    pub fn rate_response(&mut self, rating: u8) {
        if rating > 5 { return; }
        self.update_quality_score(0, Some(rating));
    }
}