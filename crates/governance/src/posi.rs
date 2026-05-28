// crates/governance/src/posi.rs
// =====================================================
// PoSI v5.0 — Proof of Sovereign Indexing
// Score de Souveraineté Décentralisée Avancé
// Intégré avec Rewards, DreamScoring, Reputation & Governance
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, debug};
use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::rewards::UserRewards;

#[derive(Error, Debug)]
pub enum PoSIError {
    #[error("Invalid contribution value")]
    InvalidContribution,
    #[error("Reputation out of bounds")]
    InvalidReputation,
    #[error("Dream score out of bounds")]
    InvalidDreamScore,
}

/// Score détaillé PoSI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoSIScore {
    pub total: f64,
    pub contribution_weight: f64,
    pub reputation_weight: f64,
    pub dream_weight: f64,
    pub last_updated: DateTime<Utc>,
}

/// Système de scoring souverain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoSI {
    pub sovereignty_score: f64,
    pub last_calculation: DateTime<Utc>,
    pub total_calculations: u64,
    pub min_score: f64,
    pub max_score: f64,

    // Historique pour analyse et traçabilité
    pub score_history: Vec<f64>,
}

impl PoSI {
    pub fn new() -> Self {
        Self {
            sovereignty_score: 0.78,
            last_calculation: Utc::now(),
            total_calculations: 0,
            min_score: 0.0,
            max_score: 1.0,
            score_history: Vec::new(),
        }
    }

    /// Calcul complet du score de souveraineté
    pub fn calculate_score(
        &mut self,
        contributions: u64,
        reputation: f64,
        dream_score: f64,
        rewards: Option<&mut UserRewards>,
    ) -> Result<PoSIScore, PoSIError> {
        if contributions == 0 {
            return Err(PoSIError::InvalidContribution);
        }
        if !(0.0..=1.0).contains(&reputation) {
            return Err(PoSIError::InvalidReputation);
        }
        if !(0.0..=1.0).contains(&dream_score) {
            return Err(PoSIError::InvalidDreamScore);
        }

        // Poids dynamiques et réalistes
        let contribution_weight = ((contributions as f64).min(2500.0) / 2500.0) * 0.42;
        let reputation_weight = reputation * 0.33;
        let dream_weight = dream_score * 0.25;

        let total = (contribution_weight + reputation_weight + dream_weight)
            .clamp(self.min_score, self.max_score);

        let score = PoSIScore {
            total,
            contribution_weight,
            reputation_weight,
            dream_weight,
            last_updated: Utc::now(),
        };

        self.sovereignty_score = total;
        self.last_calculation = Utc::now();
        self.total_calculations += 1;
        self.score_history.push(total);

        // Récompense optionnelle
        if let Some(r) = rewards {
            let bonus = if total > 0.88 { 45 } 
                       else if total > 0.80 { 28 } 
                       else if total > 0.75 { 15 } 
                       else { 6 };
            r.add_reward(crate::rewards::RewardReason::GovernanceContribution, bonus);
        }

        debug!(
            "[PoSI] Score calculé : {:.4} (contrib: {:.3}, rep: {:.3}, dream: {:.3})",
            total, contribution_weight, reputation_weight, dream_weight
        );

        Ok(score)
    }

    /// Version simplifiée pour compatibilité
    pub fn calculate_simple_score(&self, contributions: u64, reputation: f64, dream_score: f64) -> f64 {
        let c = (contributions as f64).min(2500.0) / 2500.0 * 0.42;
        let r = reputation * 0.33;
        let d = dream_score * 0.25;
        (c + r + d).clamp(0.0, 1.0)
    }

    pub fn is_sovereign(&self) -> bool {
        self.sovereignty_score >= 0.78
    }

    pub fn get_sovereignty_bonus(&self) -> f64 {
        if self.sovereignty_score > 0.88 { 1.65 }
        else if self.sovereignty_score > 0.80 { 1.35 }
        else if self.sovereignty_score > 0.75 { 1.15 }
        else { 1.0 }
    }

    pub fn summary(&self) -> String {
        format!(
            "PoSI Score: {:.4} | Calculations: {} | Sovereign: {}",
            self.sovereignty_score,
            self.total_calculations,
            self.is_sovereign()
        )
    }
}

impl Default for PoSI {
    fn default() -> Self {
        Self::new()
    }
}