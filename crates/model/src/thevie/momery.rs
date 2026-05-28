// crates/model/src/thevie/memory.rs
// =====================================================
// Local Memory v3.0 — Mémoire Locale Intelligente
// Replay Buffer + Consolidation + Anti-répétition d’erreurs
// =====================================================

use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use tracing::{debug, info};

use super::neural_mesh::{Query, Lesson};
use super::thevie_evolutif::Response;

/// Interaction complète stockée en mémoire
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub query: Query,
    pub response: Response,
    pub timestamp: u64,
    pub success: bool,
    pub quality: f32,
}

/// Mémoire locale d’un neurone
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalMemory {
    /// Replay Buffer (Prioritized + Réflexion)
    replay_buffer: VecDeque<Interaction>,
    max_buffer_size: usize,

    /// Leçons consolidées (haute qualité)
    consolidated_lessons: Vec<Lesson>,

    pub total_interactions: u64,
    pub last_consolidation: u64,
}

impl LocalMemory {
    pub fn new() -> Self {
        Self {
            replay_buffer: VecDeque::with_capacity(160),
            max_buffer_size: 160,
            consolidated_lessons: Vec::with_capacity(80),
            total_interactions: 0,
            last_consolidation: Self::now_millis(),
        }
    }

    /// Stocke une interaction et met à jour les statistiques
    pub fn store_interaction(&mut self, query: Query, response: Response) {
        let interaction = Interaction {
            query: query.clone(),
            response: response.clone(),
            timestamp: Self::now_millis(),
            success: response.quality > 0.78,
            quality: response.quality,
        };

        // Gestion circulaire du buffer
        if self.replay_buffer.len() >= self.max_buffer_size {
            self.replay_buffer.pop_front();
        }
        self.replay_buffer.push_back(interaction);

        self.total_interactions += 1;

        // Consolidation automatique toutes les 50 interactions
        if self.total_interactions % 50 == 0 {
            self.consolidate();
        }
    }

    /// Replay Buffer : Réflexion intelligente sur les interactions passées
    pub fn replay_and_reflect(&self, current_query: &Query) -> Vec<String> {
        let mut reflections = Vec::new();
        let query_lower = current_query.content.to_lowercase();

        for interaction in self.replay_buffer.iter().rev().take(12) {
            let past_lower = interaction.query.content.to_lowercase();

            if past_lower.contains(&query_lower) || query_lower.contains(&past_lower) {
                if !interaction.success {
                    reflections.push(format!(
                        "⚠️ Erreur similaire détectée : « {} » (qualité {:.2})",
                        interaction.query.content.chars().take(60).collect::<String>(),
                        interaction.quality
                    ));
                } else if interaction.quality > 0.88 {
                    reflections.push(format!(
                        "✅ Succès passé : « {} » (qualité {:.2})",
                        interaction.query.content.chars().take(55).collect::<String>(),
                        interaction.quality
                    ));
                }
            }
        }

        if reflections.is_empty() {
            reflections.push("Aucune expérience similaire trouvée dans le replay buffer.".to_string());
        }

        debug!("🪞 Replay Buffer → {} réflexions générées", reflections.len());
        reflections
    }

    /// Stocke une leçon consolidée
    pub fn store_lesson(&mut self, lesson: Lesson) {
        if self.consolidated_lessons.len() >= 80 {
            self.consolidated_lessons.remove(0);
        }
        self.consolidated_lessons.push(lesson);
    }

    /// Consolidation intelligente (ne garde que les leçons de haute qualité)
    pub fn consolidate(&mut self) {
        let before = self.consolidated_lessons.len();
        
        self.consolidated_lessons.retain(|lesson| lesson.quality >= 0.83);
        self.last_consolidation = Self::now_millis();

        debug!(
            "📦 Consolidation terminée : {} → {} leçons conservées",
            before, self.consolidated_lessons.len()
        );
    }

    /// Récupère les N dernières interactions
    pub fn get_recent_interactions(&self, count: usize) -> Vec<&Interaction> {
        self.replay_buffer.iter().rev().take(count).collect()
    }

    /// Récupère les leçons consolidées
    pub fn get_consolidated_lessons(&self) -> &[Lesson] {
        &self.consolidated_lessons
    }

    pub fn total_interactions(&self) -> u64 {
        self.total_interactions
    }

    pub fn buffer_usage(&self) -> f32 {
        self.replay_buffer.len() as f32 / self.max_buffer_size as f32
    }

    fn now_millis() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Default for LocalMemory {
    fn default() -> Self {
        Self::new()
    }
}