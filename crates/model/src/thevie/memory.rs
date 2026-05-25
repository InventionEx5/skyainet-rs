// crates/model/src/thevie/memory.rs
// =====================================================
// Local Memory
// Mémoire Locale + Replay Buffer Intelligent
// Anti-répétition d’erreurs + Consolidation
// =====================================================

use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use tracing::debug;

use super::neural_mesh::{Query, Lesson};
use super::thevie_evolutif::Response;

/// Interaction stockée en mémoire
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub query: Query,
    pub response: Response,
    pub timestamp: u64,
    pub success: bool,
}

/// Mémoire locale d’un neurone
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalMemory {
    pub replay_buffer: VecDeque<Interaction>,
    pub max_buffer_size: usize,
    pub total_interactions: u64,
    pub consolidated_lessons: Vec<Lesson>,
    pub last_consolidation: u64,
}

impl LocalMemory {
    pub fn new() -> Self {
        Self {
            replay_buffer: VecDeque::with_capacity(128),
            max_buffer_size: 128,
            total_interactions: 0,
            consolidated_lessons: Vec::new(),
            last_consolidation: 0,
        }
    }

    /// Stocke une interaction après traitement
    pub fn store_interaction(&mut self, query: &Query, response: &Response) {
        let interaction = Interaction {
            query: query.clone(),
            response: response.clone(),
            timestamp: Self::now_millis(),
            success: response.quality > 0.75,
        };

        if self.replay_buffer.len() >= self.max_buffer_size {
            self.replay_buffer.pop_front();
        }
        self.replay_buffer.push_back(interaction);
        self.total_interactions += 1;
    }

    // =====================================================
    // REPLAY BUFFER - Réflexion anti-erreur
    // =====================================================
    pub fn replay_and_reflect(&self, current_query: &Query) -> Vec<String> {
        let mut reflections = Vec::new();
        let search_term = current_query.content.chars().take(25).collect::<String>().to_lowercase();

        for interaction in self.replay_buffer.iter().rev().take(10) {
            let past_query = interaction.query.content.to_lowercase();

            if past_query.contains(&search_term) {
                if !interaction.success {
                    reflections.push(format!(
                        "⚠️ Erreur passée évitée : « {} » → Qualité faible ({:.2})",
                        interaction.query.content, interaction.response.quality
                    ));
                } else {
                    reflections.push(format!(
                        "✅ Succès similaire : « {} » → Qualité ({:.2})",
                        interaction.query.content, interaction.response.quality
                    ));
                }
            }
        }

        if reflections.is_empty() {
            reflections.push("Aucune interaction similaire trouvée dans le replay buffer.".to_string());
        }

        debug!("🪞 Replay terminé : {} réflexions générées", reflections.len());
        reflections
    }

    /// Stocke une leçon venue du Neural Mesh
    pub fn store_lesson(&mut self, lesson: Lesson) {
        if self.consolidated_lessons.len() >= 64 {
            self.consolidated_lessons.remove(0);
        }
        self.consolidated_lessons.push(lesson);
    }

    /// Consolidation périodique (garde seulement les leçons de haute qualité)
    pub fn consolidate(&mut self) {
        let before = self.consolidated_lessons.len();
        self.consolidated_lessons.retain(|l| l.quality > 0.82);

        self.last_consolidation = Self::now_millis();
        debug!(
            "📦 Consolidation mémoire : {} → {} leçons conservées",
            before, self.consolidated_lessons.len()
        );
    }

    pub fn total_interactions(&self) -> u64 {
        self.total_interactions
    }

    pub fn get_recent_interactions(&self, count: usize) -> Vec<&Interaction> {
        self.replay_buffer.iter().rev().take(count).collect()
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
