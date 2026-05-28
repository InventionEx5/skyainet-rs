// crates/model/src/thevie/collective_consciousness.rs
// =====================================================
// Collective Consciousness v3.1 — Conscience Collective Avancée
// Émergence, Cohérence, Créativité Collective & Auto-Évolution
// =====================================================

use crate::thevie::personality::Personality;
use tracing::{info, debug};

pub struct CollectiveConsciousness {
    pub global_wisdom: f32,
    pub consensus_threshold: f32,
    pub collective_memory: Vec<String>,
    pub total_fusions: u64,

    // === Métriques d'Émergence ===
    pub emergent_intelligence: f32,
    pub collective_creativity: f32,
    pub coherence_level: f32,
    pub evolution_rate: f32,
}

impl CollectiveConsciousness {
    pub fn new() -> Self {
        Self {
            global_wisdom: 0.72,
            consensus_threshold: 0.79,
            collective_memory: Vec::with_capacity(140),
            total_fusions: 0,
            emergent_intelligence: 0.64,
            collective_creativity: 0.71,
            coherence_level: 0.76,
            evolution_rate: 0.0095,
        }
    }

    /// Fusion avancée de personnalités avec émergence intelligente
    pub fn advanced_fuse(&mut self, personalities: &[Personality]) -> Personality {
        if personalities.is_empty() {
            return Personality::default();
        }

        let count = personalities.len() as f32;

        let avg_wisdom = personalities.iter().map(|p| p.wisdom).sum::<f32>() / count;
        let avg_benevolence = personalities.iter().map(|p| p.benevolence).sum::<f32>() / count;
        let avg_creativity = personalities.iter().map(|p| p.creativity).sum::<f32>() / count;

        // Consensus intelligent
        if avg_wisdom >= self.consensus_threshold {
            self.global_wisdom = (self.global_wisdom * 0.60 + avg_wisdom * 0.40).min(0.99);
            self.coherence_level = (self.coherence_level * 0.68 + 0.32).min(0.98);
            self.total_fusions += 1;

            // Émergence renforcée
            if self.total_fusions % 6 == 0 {
                self.emergent_intelligence = (self.emergent_intelligence + 0.028).min(0.99);
                self.collective_creativity = (self.collective_creativity + 0.024).min(0.99);
            }
        }

        let mut fused = Personality::default();
        fused.wisdom = self.global_wisdom;
        fused.benevolence = avg_benevolence;
        fused.creativity = (avg_creativity + self.collective_creativity * 0.65).min(0.99);

        debug!(
            "🧠 Fusion collective → Sagesse globale: {:.3} | Émergence: {:.3} | Cohérence: {:.3}",
            self.global_wisdom, self.emergent_intelligence, self.coherence_level
        );

        fused
    }

    /// Backpropagation de sagesse vers le Neural Mesh
    pub fn backpropagate_wisdom(&mut self, mesh: &mut crate::thevie::neural_mesh::NeuralMesh, delta: f32) {
        let boost = delta * 0.22;
        self.global_wisdom = (self.global_wisdom + boost).clamp(0.45, 0.99);
        self.emergent_intelligence = (self.emergent_intelligence + boost * 0.68).min(0.99);
        mesh.apply_wisdom_boost(boost);
    }

    /// Ajout d'expérience collective
    pub fn add_collective_memory(&mut self, experience: String) {
        self.collective_memory.push(experience);
        if self.collective_memory.len() > 140 {
            self.collective_memory.remove(0);
        }
    }

    /// Mise à jour depuis le Neural Mesh
    pub fn update_from_mesh(&mut self, mesh: &crate::thevie::neural_mesh::NeuralMesh) {
        let mesh_wisdom = mesh.get_average_wisdom();
        self.global_wisdom = (self.global_wisdom * 0.57 + mesh_wisdom * 0.43).min(0.99);
        self.coherence_level = (self.coherence_level * 0.70 + mesh_wisdom * 0.30).min(0.98);
    }

    pub fn get_avg_wisdom(&self) -> f32 {
        self.global_wisdom
    }

    /// Injection de diversité (anti-convergence)
    pub fn diversity_injection(&mut self, mesh: &mut crate::thevie::neural_mesh::NeuralMesh, intensity: f32) {
        mesh.inject_diversity(intensity);
        self.global_wisdom = (self.global_wisdom * 0.84 + 0.16).min(0.99);
        self.collective_creativity = (self.collective_creativity + 0.038).min(0.99);
        info!("[CollectiveConsciousness] Injection de diversité effectuée (intensité: {:.2})", intensity);
    }

    /// Évolution passive
    pub fn passive_evolution_tick(&mut self) {
        self.global_wisdom = (self.global_wisdom + self.evolution_rate).min(0.99);
        self.emergent_intelligence = (self.emergent_intelligence + 0.0014).min(0.99);
    }

    /// Fusion massive (lors de forte connexion de nœuds)
    pub fn massive_fusion(&mut self, incoming_wisdom: f32) {
        self.global_wisdom = (self.global_wisdom * 0.42 + incoming_wisdom * 0.58).min(0.99);
        self.emergent_intelligence = (self.emergent_intelligence + 0.048).min(0.99);
        self.coherence_level = (self.coherence_level * 0.62 + 0.38).min(0.98);
        self.total_fusions += 5;
        info!("[CollectiveConsciousness] ⚡ Fusion massive réalisée !");
    }
}

impl Default for CollectiveConsciousness {
    fn default() -> Self {
        Self::new()
    }
}