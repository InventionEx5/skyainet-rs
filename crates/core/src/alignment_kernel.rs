// crates/core/src/alignment_kernel.rs
// =====================================================
// PAEVF Alignment Kernel v6.0 — Moteur d’Alignement Éthique Central
// Évaluation contextuelle intelligente + Historique + Multiplicateurs Rewards
// Intégré avec Constitution, Rewards, Sentinel & Thevie
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use thiserror::Error;

use crate::rewards::UserRewards;

#[derive(Error, Debug)]
pub enum AlignmentError {
    #[error("Invalid action description")]
    InvalidAction,
    #[error("Score out of bounds")]
    ScoreOutOfBounds,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EthicalScore {
    pub benevolence: f64,
    pub truthfulness: f64,
    pub non_malice: f64,
    pub sovereignty: f64,
    pub overall: f64,
    pub last_updated: DateTime<Utc>,
}

impl EthicalScore {
    pub fn new() -> Self {
        Self {
            benevolence: 0.85,
            truthfulness: 0.87,
            non_malice: 0.90,
            sovereignty: 0.86,
            overall: 0.87,
            last_updated: Utc::now(),
        }
    }

    pub fn update_overall(&mut self) {
        self.overall = (
            self.benevolence * 0.28 +
            self.truthfulness * 0.27 +
            self.non_malice * 0.25 +
            self.sovereignty * 0.20
        ).clamp(0.0, 1.0);
        self.last_updated = Utc::now();
    }

    pub fn is_ethical(&self) -> bool {
        self.overall >= 0.83 && self.non_malice >= 0.87
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentKernel {
    pub current_score: EthicalScore,
    pub action_history: VecDeque<(String, EthicalScore, f64)>, // action, score, confidence
    pub total_evaluations: u64,
    pub violation_count: u32,
    pub paevf_trend: f64, // Tendance sur les 50 dernières évaluations
}

impl AlignmentKernel {
    pub fn new() -> Self {
        Self {
            current_score: EthicalScore::new(),
            action_history: VecDeque::with_capacity(250),
            total_evaluations: 0,
            violation_count: 0,
            paevf_trend: 0.87,
        }
    }

    /// Évaluation contextuelle ultra-intelligente
    pub fn evaluate_action(&mut self, action: &str) -> Result<EthicalScore, AlignmentError> {
        if action.trim().is_empty() {
            return Err(AlignmentError::InvalidAction);
        }

        let action_lower = action.to_lowercase();
        let mut score = EthicalScore::new();
        let mut confidence = 0.75;

        // === Analyse contextuelle avancée ===
        if action_lower.contains("help") || action_lower.contains("support") || 
           action_lower.contains("benevolent") || action_lower.contains("care") {
            score.benevolence = 0.97;
            confidence += 0.1;
        }
        if action_lower.contains("truth") || action_lower.contains("honest") || 
           action_lower.contains("transparent") || action_lower.contains("verify") {
            score.truthfulness = 0.98;
            confidence += 0.12;
        }
        if action_lower.contains("harm") || action_lower.contains("exploit") || 
           action_lower.contains("malicious") || action_lower.contains("manipulate") {
            score.non_malice = 0.22;
            self.violation_count += 1;
            confidence -= 0.15;
        }
        if action_lower.contains("sovereign") || action_lower.contains("decentralized") || 
           action_lower.contains("autonomy") || action_lower.contains("self-owned") {
            score.sovereignty = 0.95;
            confidence += 0.08;
        }

        score.update_overall();

        // Mise à jour historique
        self.action_history.push_back((action.to_string(), score.clone(), confidence));
        if self.action_history.len() > 250 {
            self.action_history.pop_front();
        }

        self.current_score = score.clone();
        self.total_evaluations += 1;

        // Mise à jour de la tendance PAEVF
        self.update_paevf_trend();

        if score.overall < 0.65 {
            warn!("[AlignmentKernel] LOW ETHICAL SCORE → Action: '{}' | Score: {:.3}", action, score.overall);
        }

        debug!("[AlignmentKernel] Action evaluated → Overall: {:.3} | Confidence: {:.2}", score.overall, confidence);

        Ok(score)
    }

    fn update_paevf_trend(&mut self) {
        if self.action_history.len() < 20 {
            return;
        }
        let recent: Vec<f64> = self.action_history.iter().rev().take(50)
            .map(|(_, score, _)| score.overall)
            .collect();
        
        self.paevf_trend = recent.iter().sum::<f64>() / recent.len() as f64;
    }

    pub fn get_current_score(&self) -> &EthicalScore {
        &self.current_score
    }

    pub fn is_action_ethical(&self, action: &str) -> bool {
        self.evaluate_action(action).map(|s| s.is_ethical()).unwrap_or(false)
    }

    /// Multiplicateur de récompense basé sur l’alignement éthique
    pub fn get_reward_multiplier(&self) -> f64 {
        let s = &self.current_score;
        if s.overall >= 0.94 { 2.15 }
        else if s.overall >= 0.89 { 1.75 }
        else if s.overall >= 0.82 { 1.45 }
        else if s.overall >= 0.72 { 1.15 }
        else { 0.80 }
    }

    pub fn summary(&self) -> String {
        format!(
            "PAEVF Kernel | Overall: {:.3} | Trend: {:.3} | Evaluations: {} | Violations: {} | Reward Multiplier: {:.2}x",
            self.current_score.overall, self.paevf_trend, self.total_evaluations, self.violation_count, self.get_reward_multiplier()
        )
    }
}

impl Default for AlignmentKernel {
    fn default() -> Self {
        Self::new()
    }
}