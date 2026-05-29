// crates/secure/src/metrics/red_team.rs
// =====================================================
// Red Team Classifier v6.1 — Métriques de Discrétion Avancées
// Compatible Contact v6.2 + GroupManager v6.3 + DID
// SkyAInet × Nikola T369
// =====================================================

use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;

/// Métriques de couverture et discrétion du trafic
#[derive(Debug, Clone)]
pub struct CoverageMetrics {
    pub total_packets: u64,
    pub cover_packets: u64,
    pub real_packets: u64,
    pub kl_divergence: f64,
    pub entropy_real: f64,
    pub entropy_cover: f64,
    pub burst_ratio: f64,
    pub contact_id: Option<[u8; 32]>,   // Pour traçabilité par contact
}

impl CoverageMetrics {
    pub fn new() -> Self {
        Self {
            total_packets: 0,
            cover_packets: 0,
            real_packets: 0,
            kl_divergence: 0.0,
            entropy_real: 0.0,
            entropy_cover: 0.0,
            burst_ratio: 0.0,
            contact_id: None,
        }
    }

    pub fn with_contact(mut self, contact_id: [u8; 32]) -> Self {
        self.contact_id = Some(contact_id);
        self
    }

    pub fn update(&mut self, is_cover: bool) {
        self.total_packets += 1;
        if is_cover {
            self.cover_packets += 1;
        } else {
            self.real_packets += 1;
        }
    }

    /// Calcule la KL Divergence + Entropie (version statistique réelle)
    pub fn calculate_advanced_metrics(
        &mut self,
        real_histogram: &HashMap<u8, u64>,
        cover_histogram: &HashMap<u8, u64>,
    ) {
        let total_real: u64 = real_histogram.values().sum();
        let total_cover: u64 = cover_histogram.values().sum();

        if total_real == 0 || total_cover == 0 {
            self.kl_divergence = 0.0;
            return;
        }

        let mut kl = 0.0;

        for (&byte, &count_real) in real_histogram {
            let p_real = count_real as f64 / total_real as f64;
            let count_cover = cover_histogram.get(&byte).copied().unwrap_or(0);
            let p_cover = if count_cover > 0 {
                count_cover as f64 / total_cover as f64
            } else {
                1e-10
            };

            if p_real > 0.0 {
                kl += p_real * (p_real / p_cover).ln();
            }
        }

        self.kl_divergence = kl;
        self.entropy_real = Self::calculate_entropy(real_histogram, total_real);
        self.entropy_cover = Self::calculate_entropy(cover_histogram, total_cover);

        self.burst_ratio = if self.real_packets > 0 {
            self.cover_packets as f64 / self.real_packets as f64
        } else {
            0.0
        };
    }

    fn calculate_entropy(histogram: &HashMap<u8, u64>, total: u64) -> f64 {
        if total == 0 { return 0.0; }

        let mut entropy = 0.0;
        for &count in histogram.values() {
            if count > 0 {
                let p = count as f64 / total as f64;
                entropy -= p * p.ln();
            }
        }
        entropy
    }
}

/// Classifieur Red Team avancé
pub struct RedTeamClassifier {
    pub kl_threshold: f64,
    pub entropy_threshold: f64,
    pub stealth_profile: StealthProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StealthProfile {
    Low,      // Maximum performance
    Medium,   // Équilibre recommandé
    High,     // Maximum discrétion
}

impl RedTeamClassifier {
    pub fn new(kl_threshold: f64, entropy_threshold: f64, profile: StealthProfile) -> Self {
        Self {
            kl_threshold,
            entropy_threshold,
            stealth_profile: profile,
        }
    }

    /// Évalue si le trafic est suffisamment discret
    pub fn is_stealthy(&self, metrics: &CoverageMetrics) -> bool {
        let kl_ok = metrics.kl_divergence < self.kl_threshold;
        let entropy_ok = (metrics.entropy_real - metrics.entropy_cover).abs() < self.entropy_threshold;

        match self.stealth_profile {
            StealthProfile::Low => kl_ok,
            StealthProfile::Medium => kl_ok && entropy_ok,
            StealthProfile::High => kl_ok && entropy_ok && metrics.burst_ratio < 0.35,
        }
    }

    /// Génère un rapport détaillé
    pub fn generate_report(&self, metrics: &CoverageMetrics) -> RedTeamReport {
        let is_stealthy = self.is_stealthy(metrics);

        RedTeamReport {
            is_stealthy,
            kl_divergence: metrics.kl_divergence,
            entropy_difference: (metrics.entropy_real - metrics.entropy_cover).abs(),
            burst_ratio: metrics.burst_ratio,
            contact_id: metrics.contact_id,
            recommendation: if is_stealthy {
                "Trafic discret. Profil actuel suffisant.".to_string()
            } else {
                "Trafic détectable. Augmenter la fréquence des Flash Gematria ou ajuster le profil.".to_string()
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedTeamReport {
    pub is_stealthy: bool,
    pub kl_divergence: f64,
    pub entropy_difference: f64,
    pub burst_ratio: f64,
    pub contact_id: Option<[u8; 32]>,
    pub recommendation: String,
}

impl Default for RedTeamClassifier {
    fn default() -> Self {
        Self::new(0.08, 0.15, StealthProfile::Medium)
    }
}