// crates/model/src/thevie/memory/replay_buffer.rs
// =====================================================
// Thevie Replay Buffer v3.1 — Prioritized + Reflective + Anti-Oubli
// Version optimisée et plus robuste
// =====================================================

use std::collections::{VecDeque, HashSet};
use rand::Rng;
use serde::{Serialize, Deserialize};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub query: String,
    pub response: String,
    pub quality: f32,
    pub timestamp: u64,
    pub error_type: Option<String>,
    pub importance: f32,
}

pub struct ReplayBuffer {
    buffer: VecDeque<Experience>,
    capacity: usize,
    total_samples: u64,
    total_quality_sum: f32,
    recent_queries: VecDeque<String>,     // Anti-répétition améliorée
    max_recent_size: usize,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            total_samples: 0,
            total_quality_sum: 0.0,
            recent_queries: VecDeque::with_capacity(64),
            max_recent_size: 64,
        }
    }

    /// Ajoute une expérience avec intelligence
    pub fn push(&mut self, mut experience: Experience) {
        experience.quality = experience.quality.clamp(0.0, 1.0);
        experience.importance = (1.0 - experience.quality).max(0.15);

        // Anti-répétition améliorée
        let query_key = experience.query.chars().take(50).collect::<String>().to_lowercase();
        if self.recent_queries.contains(&query_key) {
            experience.importance *= 0.5;
        }

        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.total_quality_sum -= old.quality;
            }
        }

        self.buffer.push_back(experience.clone());
        self.total_quality_sum += experience.quality;
        self.total_samples += 1;

        // Gestion anti-répétition
        self.recent_queries.push_back(query_key);
        if self.recent_queries.len() > self.max_recent_size {
            self.recent_queries.pop_front();
        }

        debug!(
            "🧠 ReplayBuffer: Expérience ajoutée | Qualité: {:.2} | Importance: {:.2}",
            experience.quality, experience.importance
        );
    }

    /// Échantillonnage prioritaire amélioré
    pub fn prioritized_sample(&self, batch_size: usize) -> Vec<(Experience, f32)> {
        if self.buffer.is_empty() {
            return vec![];
        }

        let mut rng = rand::thread_rng();
        let mut batch = Vec::with_capacity(batch_size.min(self.buffer.len()));

        let total_importance: f32 = self.buffer.iter().map(|e| e.importance).sum();
        if total_importance < 0.01 {
            return self.sample(batch_size);
        }

        for _ in 0..batch_size.min(self.buffer.len()) {
            let mut cumulative = 0.0;
            let target = rng.gen::<f32>() * total_importance;

            for exp in &self.buffer {
                cumulative += exp.importance;
                if cumulative >= target {
                    let weight = (exp.importance / total_importance).powf(0.6);
                    batch.push((exp.clone(), weight));
                    break;
                }
            }
        }

        batch
    }

    /// Échantillonnage uniforme classique
    pub fn sample(&self, batch_size: usize) -> Vec<Experience> {
        if self.buffer.is_empty() {
            return vec![];
        }

        let mut rng = rand::thread_rng();
        let mut batch = Vec::with_capacity(batch_size.min(self.buffer.len()));

        for _ in 0..batch_size.min(self.buffer.len()) {
            let idx = rng.gen_range(0..self.buffer.len());
            batch.push(self.buffer[idx].clone());
        }
        batch
    }

    /// Réflexion intelligente sur les erreurs passées
    pub fn reflect_on_past_errors(&self, current_query: &str) -> Vec<String> {
        let mut reflections = Vec::new();
        let query_lower = current_query.to_lowercase();

        for exp in self.buffer.iter().rev().take(15) {
            if exp.quality < 0.6 {
                let similarity = self.calculate_similarity(&query_lower, &exp.query.to_lowercase());
                if similarity > 0.35 {
                    reflections.push(format!(
                        "⚠️ Erreur évitée : {} (qualité: {:.2})",
                        exp.query.chars().take(55).collect::<String>(),
                        exp.quality
                    ));
                }
            }
        }

        if reflections.is_empty() {
            reflections.push("Aucune erreur similaire détectée.".to_string());
        }

        reflections
    }

    fn calculate_similarity(&self, a: &str, b: &str) -> f32 {
        let words_a: HashSet<_> = a.split_whitespace().collect();
        let words_b: HashSet<_> = b.split_whitespace().collect();
        let intersection = words_a.intersection(&words_b).count() as f32;
        let union = (words_a.len() + words_b.len() - intersection) as f32;
        if union > 0.0 { intersection / union } else { 0.0 }
    }

    pub fn stats(&self) -> (usize, f32, u64) {
        let avg_quality = if !self.buffer.is_empty() {
            self.total_quality_sum / self.buffer.len() as f32
        } else {
            0.0
        };
        (self.buffer.len(), avg_quality, self.total_samples)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}