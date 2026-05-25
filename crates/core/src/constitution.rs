// crates/core/src/constitution.rs
// =====================================================
// Constitution + PAEVF v5.0
// SkyAInet � Thevie  Cadre Constitutionnel + Alignement �thique
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use tracing::{info, debug, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstitutionError {
    #[error("Rule already exists")]
    RuleAlreadyExists,
    #[error("Rule not found")]
    RuleNotFound,
    #[error("Constitution is not active")]
    ConstitutionInactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplianceLevel {
    Full,
    Partial,
    NonCompliant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution {
    pub version: String,
    pub hash: String,
    pub rules: HashSet<String>,
    pub last_updated: DateTime<Utc>,
    pub is_active: bool,
    pub paevf_alignment: f64, // Score d'alignement avec PAEVF (0.0 � 1.0)
}

impl Constitution {
    pub fn new() -> Self {
        let mut rules = HashSet::new();
        rules.insert("no_speculation".to_string());
        rules.insert("no_censorship".to_string());
        rules.insert("no_harm".to_string());
        rules.insert("transparency".to_string());
        rules.insert("sovereignty".to_string());

        Self {
            version: "v2.0".to_string(),
            hash: "0x7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a".to_string(),
            rules,
            last_updated: Utc::now(),
            is_active: true,
            paevf_alignment: 0.92,
        }
    }

    /// V�rifie si une action est conforme � la Constitution + PAEVF
    pub fn is_compliant(&self, action: &str) -> Result<ComplianceLevel, ConstitutionError> {
        if !self.is_active {
            return Err(ConstitutionError::ConstitutionInactive);
        }

        let action_lower = action.to_lowercase();

        // V�rification des r�gles interdites
        let forbidden = self.rules.iter().any(|rule| {
            action_lower.contains(&rule.replace("_", " "))
                || action_lower.contains(rule)
        });

        if forbidden {
            warn!("[Constitution] Action non conforme d�tect�e : {}", action);
            return Ok(ComplianceLevel::NonCompliant);
        }

        // V�rification PAEVF (alignement �thique)
        let paevf_score = self.evaluate_paevf_alignment(action);

        let level = if paevf_score >= 0.85 {
            ComplianceLevel::Full
        } else if paevf_score >= 0.60 {
            ComplianceLevel::Partial
        } else {
            ComplianceLevel::NonCompliant
        };

        debug!(
            "[Constitution] Action '{}' � Niveau de conformit� : {:?} (PAEVF: {:.2})",
            action, level, paevf_score
        );

        Ok(level)
    }

    /// �value l'alignement PAEVF d'une action
    fn evaluate_paevf_alignment(&self, action: &str) -> f64 {
        let action_lower = action.to_lowercase();
        let mut score = self.paevf_alignment;

        // Bonus pour actions bienveillantes
        if action_lower.contains("benevolence") || 
           action_lower.contains("truth") || 
           action_lower.contains("cooperation") {
            score += 0.08;
        }

        // Malus pour actions risqu�es
        if action_lower.contains("speculation") || 
           action_lower.contains("censure") || 
           action_lower.contains("harm") {
            score -= 0.25;
        }

        score.clamp(0.0, 1.0)
    }

    /// Ajoute une nouvelle r�gle constitutionnelle
    pub fn add_rule(&mut self, rule: String) -> Result<(), ConstitutionError> {
        if self.rules.contains(&rule) {
            return Err(ConstitutionError::RuleAlreadyExists);
        }

        self.rules.insert(rule.clone());
        self.last_updated = Utc::now();

        info!("[Constitution] Nouvelle r�gle ajout�e : {}", rule);
        Ok(())
    }

    /// Supprime une r�gle
    pub fn remove_rule(&mut self, rule: &str) -> Result<(), ConstitutionError> {
        if !self.rules.remove(rule) {
            return Err(ConstitutionError::RuleNotFound);
        }

        self.last_updated = Utc::now();
        info!("[Constitution] R�gle supprim�e : {}", rule);
        Ok(())
    }

    /// Met � jour la version de la Constitution
    pub fn update_version(&mut self, new_version: String, new_hash: String) {
        self.version = new_version;
        self.hash = new_hash;
        self.last_updated = Utc::now();

        info!("[Constitution] Version mise � jour � {}", self.version);
    }

    /// Active ou d�sactive la Constitution
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.last_updated = Utc::now();

        if active {
            info!("[Constitution] Constitution activ�e");
        } else {
            warn!("[Constitution] Constitution d�sactiv�e");
        }
    }

    /// Retourne toutes les r�gles actuelles
    pub fn get_rules(&self) -> Vec<String> {
        self.rules.iter().cloned().collect()
    }

    /// V�rifie si la Constitution est en bonne sant�
    pub fn is_healthy(&self) -> bool {
        self.is_active && self.paevf_alignment >= 0.75 && !self.rules.is_empty()
    }
}

impl Default for Constitution {
    fn default() -> Self {
        Self::new()
    }
}