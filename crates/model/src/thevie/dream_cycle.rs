// crates/model/src/thevie/consciousness/dream_cycle.rs
// =====================================================
// Dream Cycle v2.0 — Consolidation Créative & Émergence
// Version améliorée avec meilleure créativité et réflexion
// =====================================================

use std::collections::HashSet;
use rand::Rng;
use tracing::{info, debug};

use super::neural_mesh::{NeuralMesh, NeuronId, Lesson};
use super::collective_consciousness::CollectiveConsciousness;
use super::evolution::EvolutionEngine;

pub struct DreamCycle {
    pub dream_frequency: u32,
    pub wisdom_boost: f32,
    pub top_percent: f32,
    pub creativity_factor: f32,
    pub cycles_completed: u64,
    pub min_neurons_for_dream: usize,
}

impl DreamCycle {
    pub fn new() -> Self {
        Self {
            dream_frequency: 75,
            wisdom_boost: 0.032,
            top_percent: 0.25,
            creativity_factor: 0.42,
            cycles_completed: 0,
            min_neurons_for_dream: 4,
        }
    }

    /// CŒUR DU DREAM CYCLE — Version améliorée
    pub fn run_dream_cycle(
        &mut self,
        mesh: &mut NeuralMesh,
        collective: &mut CollectiveConsciousness,
        evolution: &mut EvolutionEngine,
    ) {
        let start = std::time::Instant::now();

        let top_neurons = self.select_top_wise_neurons(mesh);
        if top_neurons.len() < self.min_neurons_for_dream {
            debug!("[DreamCycle] Pas assez de neurones sages");
            return;
        }

        debug!("[DreamCycle] Début avec {} neurones sages", top_neurons.len());

        let mut novel_lessons = Vec::new();

        for &neuron_id in &top_neurons {
            if let Some(lesson) = self.generate_novel_insight(mesh, neuron_id) {
                novel_lessons.push(lesson);
            }
        }

        // Circulation des nouvelles leçons
        for lesson in &novel_lessons {
            for &neuron_id in &top_neurons {
                mesh.circulate_lesson(neuron_id, lesson);
            }
        }

        // Boost de sagesse collective
        let boost = self.wisdom_boost * (top_neurons.len() as f32 / 18.0).min(1.6);
        collective.global_wisdom = (collective.global_wisdom + boost).min(0.99);

        // Évolution des neurones participants
        for &neuron_id in &top_neurons {
            if let Some(neuron) = mesh.get_neuron_mut(neuron_id) {
                evolution.evolve_personality(&mut neuron.personality, 0.93);
            }
        }

        self.cycles_completed += 1;

        let duration = start.elapsed().as_millis();
        info!(
            "[DreamCycle] Cycle #{} terminé en {}ms | +{:.3} sagesse | {} nouvelles leçons",
            self.cycles_completed, duration, boost, novel_lessons.len()
        );
    }

    /// Sélection intelligente des neurones les plus sages
    fn select_top_wise_neurons(&self, mesh: &NeuralMesh) -> Vec<NeuronId> {
        let mut neurons: Vec<(NeuronId, f32)> = mesh
            .neurons
            .iter()
            .map(|(id, n)| (*id, n.personality.wisdom))
            .collect();

        neurons.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let count = ((neurons.len() as f32) * self.top_percent).ceil() as usize;
        neurons
            .into_iter()
            .take(count.max(self.min_neurons_for_dream))
            .map(|(id, _)| id)
            .collect()
    }

    /// Génération d'insight créatif améliorée
    fn generate_novel_insight(&self, mesh: &NeuralMesh, neuron_id: NeuronId) -> Option<Lesson> {
        let mut rng = rand::thread_rng();

        let recent_lessons: Vec<_> = mesh
            .get_lessons_from_mesh(neuron_id, "")
            .into_iter()
            .take(4)
            .collect();

        if recent_lessons.len() < 2 {
            return None;
        }

        let base = &recent_lessons[0];
        let inspiration = &recent_lessons[rng.gen_range(1..recent_lessons.len())];

        // Créativité améliorée
        let new_content = format!(
            "Synthèse onirique créative : {} combiné à {} → Nouvelle perspective émergente",
            base.query.chars().take(50).collect::<String>(),
            inspiration.query.chars().take(50).collect::<String>()
        );

        Some(Lesson {
            query: new_content,
            response: format!(
                "Leçon onirique générée par recombinaison créative (force: {:.2})",
                self.creativity_factor
            ),
            quality: 0.87 + rng.gen_range(0.0..0.11),
            expert_used: "dream_cycle".to_string(),
            timestamp: crate::utils::now_millis(),
        })
    }

    pub fn should_trigger(&self, total_queries: u64) -> bool {
        total_queries > 0 && total_queries % self.dream_frequency as u64 == 0
    }
}

impl Default for DreamCycle {
    fn default() -> Self {
        Self::new()
    }
}