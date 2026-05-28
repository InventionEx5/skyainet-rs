// crates/node/src/dream_scoring.rs
// =====================================================
// DreamScoring v4.0 — Système de Scoring Créatif & Collectif Avancé
// Intégré avec Thevie, Rewards, ZipMemory et Collective Wisdom
// =====================================================

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tracing::{info, debug, warn};
use std::collections::VecDeque;

use crate::pouw::ContributionProof;
use crate::rewards::UserRewards;
use skyainet_memory::zip_memory::ZipMemory;

/// Contribution Dream enrichie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamContribution {
    pub proof: ContributionProof,
    pub creativity_score: f64,
    pub originality_score: f64,
    pub ethical_impact: f64,
    pub collective_wisdom_delta: f64,
    pub timestamp: DateTime<Utc>,
}

/// Scoring Dream avancé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamScoring {
    pub dream_contributions: u64,
    pub creativity_score: f64,           // 0.0 → 1.0
    pub originality_score: f64,
    pub ethical_alignment: f64,
    pub collective_impact: f64,          // Impact réel sur la sagesse collective
    pub contribution_streak: u32,
    pub last_contribution: Option<DateTime<Utc>>,

    // Historique récent pour analyse
    pub recent_contributions: VecDeque<DreamContribution>,

    // Cache ZipMemory pour stocker les contributions lourdes
    pub dream_archive: Option<ZipMemory>,

    pub total_dream_impact: f64,
    pub decay_rate: f64,
}

impl DreamScoring {
    pub fn new() -> Self {
        Self {
            dream_contributions: 0,
            creativity_score: 0.45,
            originality_score: 0.52,
            ethical_alignment: 0.96,
            collective_impact: 0.0,
            contribution_streak: 0,
            last_contribution: None,
            recent_contributions: VecDeque::with_capacity(50),
            dream_archive: None,
            total_dream_impact: 0.0,
            decay_rate: 0.978,
        }
    }

    /// Enregistre une contribution Dream avec scoring multi-dimensionnel
    pub async fn record_dream_contribution(
        &mut self,
        proof: &ContributionProof,
        collective_wisdom_delta: f64,
        rewards: &mut UserRewards,
    ) {
        let now = Utc::now();

        // Calcul des scores
        let quality_bonus = if proof.score > 0.88 { 1.35 } else { 1.0 };
        let creativity = (self.creativity_score * 0.78) + (proof.score * 0.22 * quality_bonus);
        let originality = (self.originality_score * 0.85) + 
                         (if proof.contribution_type.contains("dream") || 
                          proof.contribution_type.contains("creative") { 0.42 } else { 0.18 });

        let ethical = if proof.score > 0.85 {
            (self.ethical_alignment * 0.92) + 0.08
        } else {
            self.ethical_alignment * 0.97
        };

        let wisdom_impact = collective_wisdom_delta.clamp(-0.15, 0.45);

        let contribution = DreamContribution {
            proof: proof.clone(),
            creativity_score: creativity,
            originality_score: originality,
            ethical_impact: ethical,
            collective_wisdom_delta: wisdom_impact,
            timestamp: now,
        };

        // Mise à jour des scores
        self.creativity_score = creativity.min(0.995);
        self.originality_score = originality.min(0.99);
        self.ethical_alignment = ethical.min(0.99);
        self.collective_impact = (self.collective_impact * 0.72) + (wisdom_impact * 0.28);
        self.total_dream_impact = self.calculate_total_impact();

        // Streak
        if let Some(last) = self.last_contribution {
            if (now - last).num_hours() < 72 {
                self.contribution_streak = (self.contribution_streak + 1).min(45);
            } else {
                self.contribution_streak = 1;
            }
        } else {
            self.contribution_streak = 1;
        }

        self.last_contribution = Some(now);
        self.dream_contributions += 1;
        self.recent_contributions.push_back(contribution);

        if self.recent_contributions.len() > 50 {
            self.recent_contributions.pop_front();
        }

        // Récompense utilisateur
        let reward_amount = (proof.score * 18.0 * (1.0 + self.contribution_streak as f64 * 0.012)) as u128;
        rewards.add_reward(crate::rewards::RewardReason::DreamContribution, reward_amount);

        debug!(
            "Dream Contribution | Creativity: {:.3} | Originality: {:.3} | Streak: {} | Impact: {:.3}",
            self.creativity_score, self.originality_score, self.contribution_streak, wisdom_impact
        );
    }

    pub fn calculate_total_impact(&self) -> f64 {
        let base = 
            (self.dream_contributions as f64 * 0.12) +
            (self.creativity_score * 0.38) +
            (self.originality_score * 0.28) +
            (self.ethical_alignment * 0.12) +
            (self.collective_impact * 0.10);

        let streak_bonus = 1.0 + (self.contribution_streak as f64 * 0.009).min(0.40);

        (base * streak_bonus).clamp(0.0, 100.0)
    }

    pub fn get_total_score(&self) -> f64 {
        self.calculate_total_impact()
    }

    /// Décroissance naturelle (appelée périodiquement par Thevie)
    pub fn apply_decay(&mut self) {
        self.creativity_score *= self.decay_rate;
        self.originality_score *= self.decay_rate * 0.99;
        self.collective_impact *= 0.965;

        if self.contribution_streak > 0 {
            self.contribution_streak = (self.contribution_streak as f32 * 0.65) as u32;
        }
    }

    /// Bonus exceptionnel accordé par Thevie
    pub fn apply_thevie_bonus(&mut self, wisdom_boost: f64) {
        if wisdom_boost > 0.04 {
            self.collective_impact += wisdom_boost * 0.75;
            self.ethical_alignment = (self.ethical_alignment * 0.88) + 0.12;
            info!("Thevie Special Bonus applied (+{:.3} collective impact)", wisdom_boost);
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Dreams: {} | Creativity: {:.3} | Originality: {:.3} | Ethical: {:.3} | Collective Impact: {:.3} | Streak: {}",
            self.dream_contributions,
            self.creativity_score,
            self.originality_score,
            self.ethical_alignment,
            self.collective_impact,
            self.contribution_streak
        )
    }
}

impl Default for DreamScoring {
    fn default() -> Self {
        Self::new()
    }
}