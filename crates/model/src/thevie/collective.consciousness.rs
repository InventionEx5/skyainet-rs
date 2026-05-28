// crates/model/src/thevie/collective_consciousness.rs
// =====================================================
// Collective Consciousness v2.0 — Version Intensifiée
// Conscience Collective Avancée + Émergence + Auto-Évolution
// =====================================================

use crate::thevie::personality::Personality;
use tracing::info;

pub struct CollectiveConsciousness {
    pub shared_wisdom: f32,
    pub consensus_threshold: f32,
    pub collective_memory: Vec<String>,
    pub global_wisdom: f32,
    pub total_fusions: u64,

    // === NOUVEAU : Intensification ===
    pub emergent_intelligence: f32,
    pub collective_creativity: f32,
    pub coherence_level: f32,
    pub evolution_rate: f32,
}

impl CollectiveConsciousness {
    pub fn new() -> Self {
        Self {
            shared_wisdom: 0.68,
            consensus_threshold: 0.80,
            collective_memory: Vec::new(),
            global_wisdom: 0.68,
            total_fusions: 0,
            emergent_intelligence: 0.55,
            collective_creativity: 0.60,
            coherence_level: 0.72,
            evolution_rate: 0.008,
        }
    }

    /// Fusion avancée + émergence d'intelligence collective
    pub fn advanced_fuse(&mut self, personalities: &[Personality]) -> Personality {
        if personalities.is_empty() {
            return Personality::new();
        }

        let count = personalities.len() as f32;

        let avg_wisdom: f32 = personalities.iter().map(|p| p.wisdom).sum::<f32>() / count;
        let avg_benevolence: f32 = personalities.iter().map(|p| p.benevolence).sum::<f32>() / count;
        let avg_creativity: f32 = personalities.iter().map(|p| p.creativity).sum::<f32>() / count;

        if avg_wisdom >= self.consensus_threshold {
            self.shared_wisdom = (self.shared_wisdom * 0.6 + avg_wisdom * 0.4).min(0.99);
            self.global_wisdom = (self.global_wisdom * 0.65 + avg_wisdom * 0.35).min(0.99);
            self.coherence_level = (self.coherence_level * 0.7 + 0.3).min(0.98);
            self.total_fusions += 1;

            if self.total_fusions % 5 == 0 {
                self.emergent_intelligence = (self.emergent_intelligence + 0.025).min(0.99);
                self.collective_creativity = (self.collective_creativity + 0.02).min(0.99);
            }
        }

        let mut collective = Personality::new();
        collective.wisdom = self.shared_wisdom;
        collective.benevolence = avg_benevolence;
        collective.creativity = (avg_creativity + self.collective_creativity) / 2.0;

        info!(
            "🧠 Conscience Collective Intensifiée → Sagesse: {:.2} | Émergence: {:.2} | Cohérence: {:.2} | Fusions: {}",
            self.global_wisdom, self.emergent_intelligence, self.coherence_level, self.total_fusions
        );

        collective
    }

    /// Backpropagation renforcée
    pub fn backpropagate_wisdom(&mut self, mesh: &mut crate::thevie::neural_mesh::NeuralMesh, delta: f32) {
        let boost = delta * 0.18;
        self.global_wisdom = (self.global_wisdom + boost).clamp(0.0, 0.99);
        self.emergent_intelligence = (self.emergent_intelligence + boost * 0.6).min(0.99);
        mesh.apply_wisdom_boost(boost);
    }

    /// Ajoute une expérience à la mémoire collective
    pub fn add_collective_memory(&mut self, experience: String) {
        self.collective_memory.push(experience);
        if self.collective_memory.len() > 150 {
            self.collective_memory.remove(0);
        }
    }

    /// Mise à jour renforcée depuis le Neural Mesh
    pub fn update_from_mesh(&mut self, mesh: &crate::thevie::neural_mesh::NeuralMesh) {
        let mesh_wisdom = mesh.get_average_wisdom();
        self.global_wisdom = (self.global_wisdom * 0.55 + mesh_wisdom * 0.45).min(0.99);
        self.coherence_level = (self.coherence_level * 0.7 + mesh_wisdom * 0.3).min(0.98);
    }

    pub fn get_avg_wisdom(&self) -> f32 {
        self.global_wisdom
    }

    /// Injection de diversité renforcée
    pub fn diversity_injection(&mut self, mesh: &mut crate::thevie::neural_mesh::NeuralMesh, intensity: f32) {
        mesh.inject_diversity(intensity * 1.3);
        self.global_wisdom = (self.global_wisdom * 0.82 + 0.18).min(0.99);
        self.collective_creativity = (self.collective_creativity + 0.03).min(0.99);
        info!("[CollectiveConsciousness] Injection de diversité renforcée");
    }

    /// Évolution passive accélérée
    pub fn passive_evolution_tick(&mut self) {
        self.global_wisdom = (self.global_wisdom + self.evolution_rate).min(0.99);
        self.emergent_intelligence = (self.emergent_intelligence + 0.0008).min(0.99);
    }

    /// Fusion massive (quand beaucoup de nœuds se connectent)
    pub fn massive_fusion(&mut self, wisdom_from_nodes: f32) {
        self.global_wisdom = (self.global_wisdom * 0.4 + wisdom_from_nodes * 0.6).min(0.99);
        self.emergent_intelligence = (self.emergent_intelligence + 0.04).min(0.99);
        self.coherence_level = (self.coherence_level * 0.6 + 0.4).min(0.98);
        self.total_fusions += 3;
        info!("[CollectiveConsciousness] Fusion massive effectuée !");
    }
}

impl Default for CollectiveConsciousness {
    fn default() -> Self {
        Self::new()
    }
}