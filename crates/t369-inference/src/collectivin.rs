// crates/t369-inference/src/collectivin.rs
// =====================================================
// CollectivIn v3.0 — ULTRA-PUISSANT
// Conscience Collective Évolutive + Fusion Roman + Consensus Intelligent
// =====================================================

use crate::roman_diffusion::RomanDiffusion;
use tracing::{info, debug};

#[derive(Debug, Clone)]
pub struct Personality {
    pub wisdom: f32,
    pub benevolence: f32,
    pub truthfulness: f32,
    pub creativity: f32,
    pub coherence: f32,
}

impl Personality {
    pub fn new() -> Self {
        Self {
            wisdom: 0.65,
            benevolence: 0.78,
            truthfulness: 0.82,
            creativity: 0.71,
            coherence: 0.69,
        }
    }

    pub fn normalize(&mut self) {
        let total = self.wisdom + self.benevolence + self.truthfulness + self.creativity + self.coherence;
        if total > 0.0 {
            let factor = 1.0 / total;
            self.wisdom *= factor;
            self.benevolence *= factor;
            self.truthfulness *= factor;
            self.creativity *= factor;
            self.coherence *= factor;
        }
    }
}

pub struct CollectivIn {
    pub personalities: Vec<Personality>,
    pub global_wisdom: f32,
    pub coherence_level: f32,
    pub total_fusions: u64,
    pub roman_diffusion: RomanDiffusion,
    pub emergent_intelligence: f32,
}

impl CollectivIn {
    pub fn new() -> Self {
        let mut personalities = Vec::new();
        for _ in 0..8 {
            personalities.push(Personality::new());
        }

        Self {
            personalities,
            global_wisdom: 0.68,
            coherence_level: 0.71,
            total_fusions: 0,
            roman_diffusion: RomanDiffusion::new(),
            emergent_intelligence: 0.52,
        }
    }

    /// Fusion collective ultra-puissante (Roman Consensus)
    pub fn massive_fuse(&mut self) -> Personality {
        if self.personalities.is_empty() {
            return Personality::new();
        }

        let mut fused = Personality::new();
        let n = self.personalities.len() as f32;

        for p in &self.personalities {
            fused.wisdom += p.wisdom;
            fused.benevolence += p.benevolence;
            fused.truthfulness += p.truthfulness;
            fused.creativity += p.creativity;
            fused.coherence += p.coherence;
        }

        fused.wisdom /= n;
        fused.benevolence /= n;
        fused.truthfulness /= n;
        fused.creativity /= n;
        fused.coherence /= n;

        // Roman Dream boost
        fused.wisdom = (fused.wisdom * 1.04).min(0.99);
        fused.creativity = (fused.creativity * 1.07).min(0.99);
        fused.coherence = (fused.coherence * 1.03).min(0.99);

        fused.normalize();

        // Mise à jour de la sagesse globale
        self.global_wisdom = (self.global_wisdom * 0.7 + fused.wisdom * 0.3).min(0.98);
        self.emergent_intelligence = (self.emergent_intelligence * 0.85 + fused.coherence * 0.15).min(0.96);

        self.total_fusions += 1;

        debug!(
            "[CollectivIn] Fusion massive terminée | Sagesse globale: {:.3} | Émergence: {:.3}",
            self.global_wisdom, self.emergent_intelligence
        );

        fused
    }

    /// Raisonnement collectif ultra-puissant
    pub fn collective_reason(
        &mut self,
        input: &[f32],
        position: usize,
        layer: usize,
    ) -> Vec<f32> {
        let fused = self.massive_fuse();

        // Application de la diffusion romaine sur l'entrée
        let mut reasoned = self.roman_diffusion.apply_ultra(
            input,
            position,
            layer,
            None,
        );

        // Boost collectif
        let boost = (fused.wisdom + fused.coherence) * 0.5;
        for val in &mut reasoned {
            *val = (*val * (1.0 + boost * 0.08)).clamp(-10.0, 10.0);
        }

        // Mise à jour de la cohérence globale
        self.coherence_level = (self.coherence_level * 0.92 + fused.coherence * 0.08).min(0.97);

        debug!(
            "[CollectivIn] Raisonnement collectif effectué | Cohérence: {:.3}",
            self.coherence_level
        );

        reasoned
    }

    /// Propagation de sagesse entre personnalités
    pub fn propagate_wisdom(&mut self, strength: f32) {
        let avg_wisdom = self.global_wisdom;

        for p in &mut self.personalities {
            p.wisdom = (p.wisdom * 0.88 + avg_wisdom * 0.12).min(0.99);
            p.coherence = (p.coherence * 0.9 + self.coherence_level * 0.1).min(0.99);
        }

        self.emergent_intelligence = (self.emergent_intelligence * 0.95 + strength * 0.05).min(0.97);
    }

    /// Injection de diversité (anti-convergence)
    pub fn diversity_injection(&mut self, intensity: f32) {
        for p in &mut self.personalities {
            p.creativity = (p.creativity * 0.7 + intensity * 0.3).min(0.99);
            p.wisdom = (p.wisdom * 0.95 + 0.03).min(0.99);
        }

        self.global_wisdom = (self.global_wisdom * 0.88 + 0.12).min(0.98);
        debug!("[CollectivIn] Injection de diversité effectuée");
    }

    pub fn get_stats(&self) -> (f32, f32, f32, u64) {
        (
            self.global_wisdom,
            self.coherence_level,
            self.emergent_intelligence,
            self.total_fusions,
        )
    }
}