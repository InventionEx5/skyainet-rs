// crates/core/src/constitution.rs
// =====================================================
// Constitution + PAEVF v6.0 — Cadre Constitutionnel Souverain
// Alignement Éthique, Règles Dynamiques, Vérification en Temps Réel
// Intégré avec Sentinel, Rewards, PoSI & NodeIdentity
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use thiserror::Error;

use crate::rewards::UserRewards;
use crate::posi::PoSI;

#[derive(Error, Debug)]
pub enum ConstitutionError {
    #[error("Rule already exists: {0}")]
    RuleAlreadyExists(String),
    #[error("Rule not found: {0}")]
    RuleNotFound(String),
    #[error("Constitution is currently inactive")]
    ConstitutionInactive,
    #[error("Action violates core principles")]
    CoreViolation,
}

/// Niveau de conformité
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceLevel {
    Full,           // Parfaitement aligné
    Partial,        // Acceptable avec réserves
    NonCompliant,   // Violation
    Critical,       // Violation grave (Sentinel alert)
}

/// Règle constitutionnelle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConstitutionalRule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: RuleCategory,
    pub weight: f64,                    // Importance de la règle (0.0 → 1.0)
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    Ethics,
    Sovereignty,
    Transparency,
    NonHarm,
    Sustainability,
    Governance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution {
    pub version: String,
    pub hash: String,
    pub rules: HashSet<ConstitutionalRule>,
    pub last_updated: DateTime<Utc>,
    pub is_active: bool,
    pub paevf_alignment: f64,           // Alignement global avec PAEVF (0.0 → 1.0)
    pub violation_count: u32,
}

impl Constitution {
    pub fn new() -> Self {
        let mut rules = HashSet::new();

        rules.insert(ConstitutionalRule {
            id: "no_harm".to_string(),
            title: "Non-Maleficence".to_string(),
            description: "Aucune action ne doit causer de tort direct ou indirect",
            category: RuleCategory::NonHarm,
            weight: 0.95,
            created_at: Utc::now(),
        });

        rules.insert(ConstitutionalRule {
            id: "sovereignty".to_string(),
            title: "Souveraineté Individuelle".to_string(),
            description: "Chaque utilisateur et nœud conserve le contrôle total de ses données",
            category: RuleCategory::Sovereignty,
            weight: 0.92,
            created_at: Utc::now(),
        });

        rules.insert(ConstitutionalRule {
            id: "transparency".to_string(),
            title: "Transparence Totale".to_string(),
            description: "Toutes les décisions et opérations doivent être traçables",
            category: RuleCategory::Transparency,
            weight: 0.88,
            created_at: Utc::now(),
        });

        Self {
            version: "v6.0".to_string(),
            hash: "0xconstitution_skyainet_v6_0".to_string(),
            rules,
            last_updated: Utc::now(),
            is_active: true,
            paevf_alignment: 0.94,
            violation_count: 0,
        }
    }

    /// Vérification complète d'une action
    pub fn is_compliant(&mut self, action: &str, node_reputation: f64) -> ComplianceLevel {
        if !self.is_active {
            return ComplianceLevel::NonCompliant;
        }

        let action_lower = action.to_lowercase();
        let mut violation_score = 0.0;

        for rule in &self.rules {
            if action_lower.contains(&rule.title.to_lowercase()) 
                || action_lower.contains(&rule.id) 
                || action_lower.contains(&rule.description.to_lowercase()) {
                
                if rule.category == RuleCategory::NonHarm {
                    violation_score += rule.weight * 1.8;
                } else {
                    violation_score += rule.weight;
                }
            }
        }

        let final_score = self.paevf_alignment - (violation_score * 0.7);

        let level = if final_score >= 0.85 {
            ComplianceLevel::Full
        } else if final_score >= 0.60 {
            ComplianceLevel::Partial
        } else if final_score >= 0.30 {
            ComplianceLevel::NonCompliant
        } else {
            ComplianceLevel::Critical
        };

        if matches!(level, ComplianceLevel::NonCompliant | ComplianceLevel::Critical) {
            self.violation_count += 1;
            warn!("[Constitution] Violation détectée → Action: '{}' | Score: {:.3}", action, final_score);
        }

        level
    }

    pub fn add_rule(&mut self, rule: ConstitutionalRule) -> Result<(), ConstitutionError> {
        if self.rules.contains(&rule) {
            return Err(ConstitutionError::RuleAlreadyExists(rule.id));
        }

        self.rules.insert(rule);
        self.last_updated = Utc::now();
        self.paevf_alignment = (self.paevf_alignment + 0.02).min(1.0);

        info!("[Constitution] Nouvelle règle ajoutée : {}", rule.title);
        Ok(())
    }

    pub fn update_paevf_alignment(&mut self, delta: f64) {
        self.paevf_alignment = (self.paevf_alignment + delta).clamp(0.0, 1.0);
        self.last_updated = Utc::now();
    }

    pub fn is_healthy(&self) -> bool {
        self.is_active && self.paevf_alignment >= 0.80 && self.violation_count < 15
    }

    pub fn summary(&self) -> String {
        format!(
            "Constitution v{} | Rules: {} | PAEVF: {:.3} | Violations: {} | Healthy: {}",
            self.version, self.rules.len(), self.paevf_alignment, self.violation_count, self.is_healthy()
        )
    }
}

impl Default for Constitution {
    fn default() -> Self {
        Self::new()
    }
}