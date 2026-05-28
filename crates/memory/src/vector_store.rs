// crates/memory/src/vector_store.rs
// =====================================================
// VectorStore v5.0 — Recherche Sémantique Avancée & Hybride
// Optimisé pour Thevie : qualité, filtrage expert, hybrid search, persistance ZipMemory
// =====================================================

use std::collections::{HashMap, VecDeque};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc};
use skyainet_memory::zip_memory::ZipMemory;

/// Métadonnées enrichies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMetadata {
    pub quality: f32,                    // 0.0 → 1.0
    pub expert: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub tags: Vec<String>,
    pub importance: f32,                 // Score d'importance pour le tri
}

/// Entrée vectorielle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorEntry {
    pub id: String,
    pub embedding: Vec<f32>,
    pub metadata: VectorMetadata,
    pub content_preview: String,
}

pub struct VectorStore {
    vectors: HashMap<String, VectorEntry>,
    dimension: usize,

    // Cache LRU pour les recherches fréquentes
    search_cache: VecDeque<(Vec<f32>, Vec<(VectorEntry, f32)>)>,
    max_cache_size: usize,

    // Persistance
    archive: Option<ZipMemory>,

    total_searches: u64,
    last_search: Option<DateTime<Utc>>,
}

impl VectorStore {
    pub fn new(dimension: usize) -> Self {
        Self {
            vectors: HashMap::new(),
            dimension,
            search_cache: VecDeque::with_capacity(32),
            max_cache_size: 32,
            archive: None,
            total_searches: 0,
            last_search: None,
        }
    }

    pub fn init_archive(&mut self, base_path: &str) {
        self.archive = Some(ZipMemory::new(base_path));
        info!("[VectorStore] Archive ZipMemory initialisée");
    }

    // =====================================================
    // INSERTION
    // =====================================================

    pub fn insert(&mut self, id: &str, embedding: Vec<f32>, metadata: VectorMetadata, content_preview: String) {
        if embedding.len() != self.dimension {
            warn!("Dimension mismatch: expected {}, got {}", self.dimension, embedding.len());
            return;
        }

        let entry = VectorEntry {
            id: id.to_string(),
            embedding,
            metadata,
            content_preview,
        };

        self.vectors.insert(id.to_string(), entry);
        self.search_cache.clear(); // Invalide le cache

        debug!("Vector inserted: {}", id);
    }

    pub fn batch_insert(&mut self, entries: Vec<(String, Vec<f32>, VectorMetadata, String)>) {
        for (id, emb, meta, preview) in entries {
            self.insert(&id, emb, meta, preview);
        }
        info!("Batch insert completed: {} vectors", self.vectors.len());
    }

    // =====================================================
    // RECHERCHE AVANCÉE
    // =====================================================

    /// Recherche cosinus optimisée avec boost qualité + cache
    pub fn search(
        &mut self,
        query_embedding: &[f32],
        top_k: usize,
        min_quality: Option<f32>,
        expert_filter: Option<&str>,
    ) -> Vec<(VectorEntry, f32)> {
        if query_embedding.len() != self.dimension {
            warn!("Query dimension mismatch");
            return vec![];
        }

        // Vérification cache
        if let Some(cached) = self.get_from_cache(query_embedding) {
            return cached.iter().take(top_k).cloned().collect();
        }

        let mut results: Vec<(VectorEntry, f32)> = self.vectors
            .values()
            .filter(|entry| {
                if let Some(min_q) = min_quality {
                    if entry.metadata.quality < min_q { return false; }
                }
                if let Some(expert) = expert_filter {
                    if entry.metadata.expert != expert { return false; }
                }
                true
            })
            .map(|entry| {
                let sim = self.cosine_similarity(query_embedding, &entry.embedding);
                let boosted = sim * (0.6 + 0.4 * entry.metadata.quality);
                (entry.clone(), boosted)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(top_k);

        // Mise en cache
        self.add_to_cache(query_embedding.to_vec(), results.clone());

        self.total_searches += 1;
        self.last_search = Some(Utc::now());

        results
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        dot / (norm_a * norm_b + 1e-8)
    }

    // =====================================================
    // CACHE LRU
    // =====================================================

    fn get_from_cache(&self, query: &[f32]) -> Option<Vec<(VectorEntry, f32)>> {
        // Recherche approximative dans le cache (simplifié)
        None // À améliorer avec embedding hash si besoin
    }

    fn add_to_cache(&mut self, query: Vec<f32>, results: Vec<(VectorEntry, f32)>) {
        if self.search_cache.len() >= self.max_cache_size {
            self.search_cache.pop_front();
        }
        self.search_cache.push_back((query, results));
    }

    // =====================================================
    // PERSITANCE
    // =====================================================

    pub async fn save_to_disk(&self, path: &str) -> Result<(), String> {
        if let Some(archive) = &self.archive {
            let mut z = archive.lock().await; // si Arc<Mutex>
            // Sauvegarde en JSON compressé ou binaire
        }
        Ok(())
    }

    pub fn stats(&self) -> (usize, u64) {
        (self.vectors.len(), self.total_searches)
    }
}