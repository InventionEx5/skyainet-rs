// crates/model/src/thevie/neurone.rs
// =====================================================
// Neurone v3.0 — Unité Vivante du Neural Mesh
// Naissance, Évolution, Réplication et Migration
// =====================================================

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use tracing::{info, debug};

use super::neural_mesh::{NeuralMesh, NeuronId, Lesson};
use super::personality::Personality;
use super::memory::LocalMemory;
use super::collective_consciousness::CollectiveConsciousness;
use super::evolution::EvolutionEngine;

/// Neurone vivant dans le Neural Mesh
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neurone {
    pub id: NeuronId,
    pub personality: Personality,
    pub memory: LocalMemory,
    pub activity_score: u32,
    pub birth_time: u64,
    pub replication_count: u32,
    pub last_activity: u64,
    pub experts_competence: HashMap<String, f32>, // Compétences locales MoE
}

impl Neurone {
    /// Création d’un nouveau neurone avec héritage collectif
    pub fn new(mesh: &mut NeuralMesh, collective: &CollectiveConsciousness) -> NeuronId {
        let now = Self::now_millis();

        let mut personality = collective.get_average_personality();
        personality.mutate_at_birth(0.035); // Diversité génétique légère

        let mut experts_competence = HashMap::new();
        for expert in ["text", "code", "analysis", "science", "ethics", "finance"] {
            experts_competence.insert(expert.to_string(), 0.68 + (rand::random::<f32>() * 0.22));
        }

        let neurone = Neurone {
            id: 0,
            personality,
            memory: LocalMemory::new(),
            activity_score: 0,
            birth_time: now,
            replication_count: 0,
            last_activity: now,
            experts_competence,
        };

        let id = mesh.add_neuron(neurone);

        info!("🧬 Neurone {} créé | Sagesse héritée: {:.2} | Experts: 6", id, collective.get_avg_wisdom());
        id
    }

    /// Incrémente l’activité après une interaction
    pub fn increment_activity(&mut self) {
        self.activity_score = self.activity_score.saturating_add(1);
        self.last_activity = Self::now_millis();
    }

    /// Évolution de la personnalité selon la qualité de la réponse
    pub fn evolve(&mut self, quality: f32, engine: &EvolutionEngine) {
        engine.evolve_personality(&mut self.personality, quality);

        // Évolution locale des experts MoE
        for comp in self.experts_competence.values_mut() {
            *comp = (*comp * 0.965 + quality * 0.045).clamp(0.5, 2.2);
        }
    }

    /// Récupère une leçon pertinente depuis le mesh
    pub fn get_relevant_lesson(&self, mesh: &NeuralMesh, query: &str) -> Option<Lesson> {
        let lessons = mesh.get_lessons_from_mesh(self.id, query);
        lessons.into_iter().max_by(|a, b| a.quality.partial_cmp(&b.quality).unwrap())
    }

    /// Partage une leçon avec le mesh et sa propre mémoire
    pub fn share_lesson(&self, mesh: &mut NeuralMesh, lesson: Lesson) {
        mesh.circulate_lesson(self.id, &lesson);
        self.memory.store_lesson(lesson);
    }

    /// Réplication (création d’un enfant avec mutation)
    pub fn replicate(&mut self, mesh: &mut NeuralMesh) -> NeuronId {
        let mut clone = self.clone();
        clone.id = 0;
        clone.replication_count = 0;
        clone.personality.mutate_at_birth(0.055); // Mutation plus marquée

        let new_id = mesh.add_neuron(clone);
        self.replication_count += 1;

        info!("🧬 Réplication réussie → Neurone {} (génération {})", new_id, self.replication_count);
        new_id
    }

    /// Prépare l’état pour migration (sérialisation)
    pub fn prepare_for_migration(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Échec sérialisation pour migration")
    }

    /// Applique un état migré
    pub fn apply_migrated_state(&mut self, data: &[u8]) {
        if let Ok(state) = serde_json::from_slice::<Neurone>(data) {
            self.personality = state.personality;
            self.memory = state.memory;
            self.experts_competence = state.experts_competence;
            self.activity_score = state.activity_score;
            self.replication_count = state.replication_count;
            self.last_activity = Self::now_millis();

            info!("📥 Neurone {} restauré après migration (activité: {})", self.id, self.activity_score);
        }
    }

    /// Vérifie si le neurone est en bonne santé
    pub fn is_healthy(&self) -> bool {
        self.activity_score >= 10 &&
        self.personality.wisdom > 0.58 &&
        self.experts_competence.values().all(|&c| c > 0.62)
    }

    fn now_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Default for Neurone {
    fn default() -> Self {
        Self {
            id: 0,
            personality: Personality::default(),
            memory: LocalMemory::new(),
            activity_score: 0,
            birth_time: Self::now_millis(),
            replication_count: 0,
            last_activity: Self::now_millis(),
            experts_competence: HashMap::new(),
        }
    }
}