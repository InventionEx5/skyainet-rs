// crates/model/src/thevie/neurone.rs
// =====================================================
// Neurone
// Instance Vivante du Neural Mesh — Naissance, Évolution et Migration
// =====================================================

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use tracing::info;

use super::neural_mesh::{NeuralMesh, NeuronId, Lesson};
use super::personality::Personality;
use super::memory::LocalMemory;
use super::collective_consciousness::CollectiveConsciousness;
use super::evolution::EvolutionEngine;

/// Structure principale du Neurone
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neurone {
    pub id: NeuronId,
    pub personality: Personality,
    pub memory: LocalMemory,
    pub activity_score: u32,
    pub birth_time: u64,
    pub replication_count: u32,
    pub last_activity: u64,
    pub experts_competence: HashMap<String, f32>, // poids MoE locaux
}

impl Neurone {
    /// Création d’un nouveau neurone avec héritage collectif
    pub fn new(
        mesh: &mut NeuralMesh,
        collective: &CollectiveConsciousness,
    ) -> NeuronId {
        let now = Self::now_millis();

        let mut personality = collective.get_average_personality();
        personality.mutate_at_birth(0.03); // Légère diversité génétique

        let mut experts_competence = HashMap::new();
        for expert in ["text", "code", "analysis", "science", "ethics", "finance"] {
            experts_competence.insert(expert.to_string(), 0.70 + (rand::random::<f32>() * 0.18));
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

        info!(
            "🧬 Neurone {} né | Sagesse collective héritée: {:.2} | Experts: 6",
            id, collective.get_avg_wisdom()
        );

        id
    }

    /// Incrémente l’activité après chaque requête
    pub fn increment_activity(&mut self) {
        self.activity_score = self.activity_score.saturating_add(1);
        self.last_activity = Self::now_millis();
    }

    /// Évolution de la personnalité
    pub fn evolve_personality(&mut self, quality: f32, engine: &EvolutionEngine) {
        engine.evolve_personality(&mut self.personality, quality);

        // Mise à jour des compétences locales des experts
        for (_, comp) in self.experts_competence.iter_mut() {
            *comp = (*comp * 0.97 + quality * 0.03).clamp(0.5, 2.0);
        }
    }

    /// Récupère une leçon pertinente des pairs via le mesh
    pub fn get_lesson_from_mesh(&self, mesh: &NeuralMesh, query: &str) -> Option<Lesson> {
        let lessons = mesh.get_lessons_from_mesh(self.id, query);
        lessons.into_iter().max_by(|a, b| a.quality.partial_cmp(&b.quality).unwrap())
    }

    /// Partage une leçon avec le mesh
    pub fn share_lesson(&self, mesh: &mut NeuralMesh, lesson: Lesson) {
        mesh.circulate_lesson(self.id, &lesson);
        self.memory.store_lesson(lesson);
    }

    /// Prépare l’état pour migration
    pub fn get_state_for_migration(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Migration serialization failed")
    }

    /// Applique un état migré
    pub fn apply_migrated_state(&mut self, data: &[u8]) {
        if let Ok(state) = serde_json::from_slice::<Neurone>(data) {
            self.personality = state.personality;
            self.memory = state.memory;
            self.experts_competence = state.experts_competence;
            self.activity_score = state.activity_score;
            self.replication_count = state.replication_count;
            info!("📥 Neurone {} migré avec succès (activité: {})", self.id, self.activity_score);
        }
    }

    /// Réplication (création d’un clone avec mutation légère)
    pub fn replicate(&mut self, mesh: &mut NeuralMesh) -> NeuronId {
        let mut clone = self.clone();
        clone.id = 0;
        clone.replication_count += 1;
        clone.personality.mutate_at_birth(0.05); // Dérive génétique légère

        let new_id = mesh.add_neuron(clone);
        self.replication_count += 1;

        info!("🧬 Réplication réussie → Nouveau neurone {} (génération {})", new_id, self.replication_count);
        new_id
    }

    /// Vérifie si le neurone est en bonne santé
    pub fn is_healthy(&self) -> bool {
        self.activity_score > 8 
            && self.personality.wisdom > 0.55 
            && self.experts_competence.values().all(|&c| c > 0.6)
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