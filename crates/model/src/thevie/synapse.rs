// crates/model/src/thevie/synapse.rs
// =====================================================
// Synapse v2.0 — Connexion Neurone à Neurone
// SkyAInet × Thevie — Version Étendue & Ingénieuse
// =====================================================

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub type NeuronId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Synapse {
    pub from: NeuronId,
    pub to: NeuronId,
    pub strength: f32,           // 0.0 → 1.0 (force de la connexion)
    pub usage_count: u32,
    pub last_used: u64,          // timestamp en millisecondes
    pub decay_rate: f32,         // taux de dégradation (optionnel)
}

impl Synapse {
    pub fn new(from: NeuronId, to: NeuronId) -> Self {
        Self {
            from,
            to,
            strength: 0.5,
            usage_count: 0,
            last_used: Self::now_millis(),
            decay_rate: 0.01,
        }
    }

    /// Renforce la synapse (règle Hebbienne)
    pub fn strengthen(&mut self, amount: f32) {
        self.strength = (self.strength + amount).min(1.0);
        self.usage_count += 1;
        self.last_used = Self::now_millis();
    }

    /// Affaiblit la synapse (règle Anti-Hebbienne)
    pub fn weaken(&mut self, amount: f32) {
        self.strength = (self.strength - amount).max(0.05);
        self.last_used = Self::now_millis();
    }

    /// Applique une légère dégradation naturelle
    pub fn decay(&mut self) {
        self.strength = (self.strength - self.decay_rate).max(0.05);
    }

    /// Vérifie si la synapse est encore active
    pub fn is_active(&self) -> bool {
        self.strength > 0.1 && self.usage_count > 0
    }

    /// Âge de la synapse en secondes
    pub fn age_seconds(&self) -> u64 {
        (Self::now_millis() - self.last_used) / 1000
    }

    fn now_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Default for Synapse {
    fn default() -> Self {
        Self::new(0, 0)
    }
}