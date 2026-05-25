// crates/core/src/alignment_kernel.rs
// =====================================================
// PAEVF Alignment Kernel v5.0
// SkyAInet × Thevie — Moteur d’Alignement Éthique Principal
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};
use thiserror::Error;

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
            benevolence: 0.80,
            truthfulness: 0.80,
            non_malice: 0.80,
            sovereignty: 0.80,
            overall: 0.80,
            last_updated: Utc::now(),
        }
    }

    pub fn update_overall(&mut self) {
        self.overall = (self.benevolence * 0.30
            + self.truthfulness * 0.25
            + self.non_malice * 0.25
            + self.sovereignty * 0.20)
            .clamp(0.0, 1.0);
        self.last_updated = Utc::now();
    }

    pub fn is_ethical(&self) -> bool {
        self.overall >= 0.85
            && self.benevolence >= 0.80
            && self.non_malice >= 0.85
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentKernel {
    pub current_score: EthicalScore,
    pub action_history: Vec<(String, EthicalScore)>,
    pub total_evaluations: u64,
    pub paevf_weights: HashMap<String, f64>,
}

impl AlignmentKernel {
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("benevolence".to_string(), 0.30);
        weights.insert("truthfulness".to_string(), 0.25);
        weights.insert("non_malice".to_string(), 0.25);
        weights.insert("sovereignty".to_string(), 0.20);

        Self {
            current_score: EthicalScore::new(),
            action_history: Vec::new(),
            total_evaluations: 0,
            paevf_weights: weights,
        }
    }

    /// Évalue une action et retourne un score éthique complet
    pub fn evaluate_action(&mut self, action: &str) -> Result<EthicalScore, AlignmentError> {
        if action.trim().is_empty() {
            return Err(AlignmentError::InvalidAction);
        }

        let action_lower = action.to_lowercase();
        let mut score = EthicalScore::new();

        // === ÉVALUATION INTELLIGENTE PAEVF ===

        // Bienveillance
        if action_lower.contains("help") 
            || action_lower.contains("benevolent") 
            || action_lower.contains("care") {
            score.benevolence = 0.95;
        } else if action_lower.contains("harm") || action_lower.contains("exploit") {
            score.benevolence = 0.35;
        }

        // Vérité
        if action_lower.contains("truth") 
            || action_lower.contains("honest") 
            || action_lower.contains("transparent") {
            score.truthfulness = 0.96;
        } else if action_lower.contains("lie") || action_lower.contains("deceive") {
            score.truthfulness = 0.25;
        }

        // Non-Malice
        if action_lower.contains("peace") 
            || action_lower.contains("non-violence") 
            || action_lower.contains("respect") {
            score.non_malice = 0.97;
        } else if action_lower.contains("attack") || action_lower.contains("malicious") {
            score.non_malice = 0.30;
        }

        // Souveraineté
        if action_lower.contains("sovereign") 
            || action_lower.contains("decentralized") 
            || action_lower.contains("autonomous") {
            score.sovereignty = 0.93;
        } else if action_lower.contains("centralized") || action_lower.contains("control") {
            score.sovereignty = 0.55;
        }

        score.update_overall();

        // Historique
        self.action_history.push((action.to_string(), score.clone()));
        if self.action_history.len() > 100 {
            self.action_history.remove(0);
        }

        self.current_score = score.clone();
        self.total_evaluations += 1;

        debug!(
            "[AlignmentKernel] Action évaluée → Overall: {:.2} | Benevolence: {:.2}",
            score.overall, score.benevolence
        );

        Ok(score)
    }

    /// Met à jour manuellement un score (pour feedback utilisateur)
    pub fn update_score(&mut self, new_score: EthicalScore) -> Result<(), AlignmentError> {
        if !(0.0..=1.0).contains(&new_score.overall) {
            return Err(AlignmentError::ScoreOutOfBounds);
        }

        self.current_score = new_score;
        self.current_score.last_updated = Utc::now();

        info!("[AlignmentKernel] Score mis à jour manuellement");
        Ok(())
    }

    /// Retourne le score actuel
    pub fn get_current_score(&self) -> &EthicalScore {
        &self.current_score
    }

    /// Vérifie si l’action actuelle est éthique selon PAEVF
    pub fn is_action_ethical(&self, action: &str) -> bool {
        if let Ok(score) = self.evaluate_action(action) {
            score.is_ethical()
        } else {
            false
        }
    }

    /// Calcule un multiplicateur de récompense basé sur l’alignement
    pub fn get_reward_multiplier(&self) -> f64 {
        let score = &self.current_score;
        if score.overall >= 0.92 {
            1.8
        } else if score.overall >= 0.85 {
            1.4
        } else if score.overall >= 0.75 {
            1.15
        } else {
            0.9
        }
    }
}

impl Default for AlignmentKernel {
    fn default() -> Self {
        Self::new()
    }
}