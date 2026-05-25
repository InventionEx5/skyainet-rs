// crates/core/src/economics.rs
// =====================================================
// Node Economics v5.0 — Incitations & Marketplace
// SkyAInet × Thevie
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeTier {
    Mini,       // Gratuit
    Light,      // ~6€/mois
    Full,       // ~18€/mois
    DreamWeaver,// ~32€/mois
    Validator,  // ~55€/mois
}

impl NodeTier {
    pub fn monthly_price_eur(&self) -> u64 {
        match self {
            NodeTier::Mini => 0,
            NodeTier::Light => 6,
            NodeTier::Full => 18,
            NodeTier::DreamWeaver => 32,
            NodeTier::Validator => 55,
        }
    }

    pub fn compute_power_multiplier(&self) -> f64 {
        match self {
            NodeTier::Mini => 1.0,
            NodeTier::Light => 2.5,
            NodeTier::Full => 6.0,
            NodeTier::DreamWeaver => 8.5,
            NodeTier::Validator => 12.0,
        }
    }

    pub fn dream_priority(&self) -> u8 {
        match self {
            NodeTier::Mini => 1,
            NodeTier::Light => 3,
            NodeTier::Full => 6,
            NodeTier::DreamWeaver => 9,
            NodeTier::Validator => 7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEconomics {
    pub tier: NodeTier,
    pub is_rented_out: bool,
    pub rental_price_per_hour: u64,      // en SKY
    pub total_earned_sky: u128,
    pub last_payout: Option<DateTime<Utc>>,
    pub contribution_score: f64,                    // Score général (PoUW + Dream)
    pub thevie_evolution_contribution: f64,         // ← NOUVEAU : Contribution à l’évolution de Thevie (0.0 → 1.0)
}

impl NodeEconomics {
    pub fn new(tier: NodeTier) -> Self {
        Self {
            tier,
            is_rented_out: false,
            rental_price_per_hour: match tier {
                NodeTier::Full => 12,
                NodeTier::DreamWeaver => 25,
                NodeTier::Validator => 40,
                _ => 0,
            },
            total_earned_sky: 0,
            last_payout: None,
            contribution_score: 0.5,
            thevie_evolution_contribution: 0.25,     // ← NOUVEAU
        }
    }

    /// Calcule les gains estimés par mois (en SKY)
    pub fn estimated_monthly_earnings(&self) -> u128 {
        if !self.is_rented_out {
            return 0;
        }
        // Estimation : 18h/jour × 30 jours × prix/heure × taux d'utilisation
        (self.rental_price_per_hour as u128 * 18 * 30 * 65) / 100
    }

    /// Ajoute des récompenses (PoUW + Dream Contributions)
    pub fn add_rewards(&mut self, amount: u128, reason: &str) {
        self.total_earned_sky += amount;
        println!("[Economics] +{} SKY ({})", amount, reason);
    }

    /// Active la mise en location
    pub fn rent_out(&mut self) {
        if self.tier == NodeTier::Mini || self.tier == NodeTier::Light {
            return;
        }
        self.is_rented_out = true;
        println!("[Economics] Nœud mis en location à {} SKY/heure", self.rental_price_per_hour);
    }

    /// Désactive la mise en location
    pub fn stop_renting(&mut self) {
        self.is_rented_out = false;
    }

    /// Enregistre une contribution à l’évolution de Thevie
    pub fn record_thevie_evolution_contribution(&mut self, amount: f64) {
        self.thevie_evolution_contribution = (self.thevie_evolution_contribution + amount).min(1.0);
        
        // Bonus sur le score général
        self.contribution_score = (self.contribution_score + amount * 0.3).min(1.0);
        
        println!(
            "[Economics] Contribution à l’évolution de Thevie +{:.2} → {:.2}",
            amount, self.thevie_evolution_contribution
        );
    }

    /// Calcule le multiplicateur de récompense basé sur la contribution à Thevie
    pub fn get_thevie_evolution_multiplier(&self) -> f64 {
        1.0 + (self.thevie_evolution_contribution * 1.5) // Jusqu’à +150% de récompenses
    }
}