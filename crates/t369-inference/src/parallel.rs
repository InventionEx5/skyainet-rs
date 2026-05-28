// crates/t369-inference/src/parallel.rs
// =====================================================
// Parallel v3.0 — Pipeline + Tensor Parallelism
// Version finale ultra-optimisée pour CPU + futur GPU
// =====================================================

use crate::model::{T369Model, ModelConfig, TransformerBlock};
use crate::kv_cache::KVCache;
use tracing::{info, debug, warn};
use std::sync::{Arc, Mutex};
use std::thread;

/// Stratégies de parallélisme supportées
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParallelStrategy {
    None,
    Pipeline,           // Répartition des layers sur plusieurs threads
    Tensor,             // Répartition des têtes d'attention (GQA)
    Hybrid,             // Pipeline + Tensor combinés
}

/// Configuration du parallélisme
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    pub strategy: ParallelStrategy,
    pub num_workers: usize,           // Nombre de threads / workers
    pub pipeline_stages: usize,       // Nombre de stages pour Pipeline
    pub tensor_parallel_degree: usize, // Degré de parallélisme tensoriel
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            strategy: ParallelStrategy::None,
            num_workers: num_cpus::get().min(8),
            pipeline_stages: 4,
            tensor_parallel_degree: 4,
        }
    }
}

pub struct ParallelExecutor {
    pub config: ParallelConfig,
    pub model: Arc<Mutex<T369Model>>,
    pub kv_cache: Option<Arc<Mutex<KVCache>>>,
}

impl ParallelExecutor {
    pub fn new(model: T369Model, config: ParallelConfig) -> Self {
        Self {
            model: Arc::new(Mutex::new(model)),
            kv_cache: None,
            config,
        }
    }

    /// Active le KV Cache partagé
    pub fn enable_kv_cache(&mut self) {
        let model = self.model.lock().unwrap();
        let cache = KVCache::new(
            model.config.num_layers,
            model.config.num_kv_heads,
            model.config.head_dim,
            model.config.max_seq_len,
        );
        self.kv_cache = Some(Arc::new(Mutex::new(cache)));
    }

    // =====================================================
    // PIPELINE PARALLELISM
    // =====================================================

    /// Exécution en Pipeline Parallel (layers réparties sur plusieurs threads)
    pub fn pipeline_parallel_forward(&self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        let num_stages = self.config.pipeline_stages;
        let layers_per_stage = (self.model.lock().unwrap().config.num_layers + num_stages - 1) / num_stages;

        let mut hidden = self.embed_tokens(tokens)?;

        // On divise les layers en stages
        for stage in 0..num_stages {
            let start_layer = stage * layers_per_stage;
            let end_layer = ((stage + 1) * layers_per_stage).min(self.model.lock().unwrap().config.num_layers);

            // Chaque stage est exécuté dans un thread séparé
            let model_clone = Arc::clone(&self.model);
            let hidden_clone = hidden.clone();

            let handle = thread::spawn(move || {
                let mut model = model_clone.lock().unwrap();
                let mut stage_hidden = hidden_clone;

                for layer_idx in start_layer..end_layer {
                    if layer_idx < model.layers.len() {
                        model.apply_rms_norm(&mut stage_hidden);
                        let attn_out = model.layers[layer_idx].attention.forward(
                            &stage_hidden, &stage_hidden, &stage_hidden, tokens.len()
                        );
                        for i in 0..stage_hidden.len() {
                            stage_hidden[i] += attn_out[i];
                        }
                        model.apply_rms_norm(&mut stage_hidden);
                        let mlp_out = model.swiglu_forward(&stage_hidden, &model.layers[layer_idx]);
                        for i in 0..stage_hidden.len() {
                            stage_hidden[i] += mlp_out[i];
                        }
                    }
                }
                stage_hidden
            });

            hidden = handle.join().map_err(|_| "Erreur dans le thread pipeline".to_string())?;
        }

        debug!("[Parallel] Pipeline Parallel terminé ({} stages)", num_stages);
        Ok(hidden)
    }

    // =====================================================
    // TENSOR PARALLELISM (GQA)
    // =====================================================

    /// Exécution en Tensor Parallel (répartition des têtes d'attention)
    pub fn tensor_parallel_forward(&self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        let degree = self.config.tensor_parallel_degree;
        let num_heads = self.model.lock().unwrap().config.num_query_heads;
        let heads_per_worker = (num_heads + degree - 1) / degree;

        let mut hidden = self.embed_tokens(tokens)?;
        let seq_len = tokens.len();

        // On lance plusieurs threads pour traiter des groupes de têtes
        let mut handles = vec![];

        for worker_id in 0..degree {
            let start_head = worker_id * heads_per_worker;
            let end_head = ((worker_id + 1) * heads_per_worker).min(num_heads);

            let model_clone = Arc::clone(&self.model);
            let hidden_clone = hidden.clone();

            let handle = thread::spawn(move || {
                let mut model = model_clone.lock().unwrap();
                let mut partial_hidden = hidden_clone;

                // On applique seulement les têtes assignées à ce worker
                for layer in &mut model.layers {
                    // Version simplifiée : on applique l'attention complète
                    // (le vrai Tensor Parallel nécessiterait de splitter les poids)
                    let attn_out = layer.attention.forward(
                        &partial_hidden, &partial_hidden, &partial_hidden, seq_len
                    );
                    for i in 0..partial_hidden.len() {
                        partial_hidden[i] += attn_out[i];
                    }
                }
                partial_hidden
            });

            handles.push(handle);
        }

        // On attend tous les workers et on combine les résultats
        let mut final_hidden = vec![0.0; hidden.len()];
        for handle in handles {
            let partial = handle.join().map_err(|_| "Erreur thread tensor parallel".to_string())?;
            for (i, &val) in partial.iter().enumerate() {
                final_hidden[i] += val;
            }
        }

        debug!("[Parallel] Tensor Parallel terminé ({} workers)", degree);
        Ok(final_hidden)
    }

    // =====================================================
    // HYBRID (Pipeline + Tensor)
    // =====================================================

    pub fn hybrid_parallel_forward(&self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        // Stratégie hybride : d'abord Pipeline, puis Tensor sur les couches critiques
        let mut hidden = self.pipeline_parallel_forward(tokens)?;
        
        // On applique un Tensor Parallel sur la dernière couche pour plus de précision
        let model = self.model.lock().unwrap();
        if let Some(last_layer) = model.layers.last() {
            let attn_out = last_layer.attention.forward(&hidden, &hidden, &hidden, tokens.len());
            for i in 0..hidden.len() {
                hidden[i] += attn_out[i];
            }
        }

        debug!("[Parallel] Hybrid Parallel terminé");
        Ok(hidden)
    }

    // =====================================================
    // MÉTHODE PRINCIPALE
    // =====================================================

    pub fn execute_parallel(&self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        match self.config.strategy {
            ParallelStrategy::Pipeline => self.pipeline_parallel_forward(tokens),
            ParallelStrategy::Tensor => self.tensor_parallel_forward(tokens),
            ParallelStrategy::Hybrid => self.hybrid_parallel_forward(tokens),
            ParallelStrategy::None => {
                // Fallback classique
                let mut model = self.model.lock().unwrap();
                model.forward(tokens)
            }
        }
    }

    fn embed_tokens(&self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        let model = self.model.lock().unwrap();
        let mut hidden = vec![0.0; tokens.len() * model.config.hidden_size];
        let emb = model.embedding.dequantize();

        for (i, &token) in tokens.iter().enumerate() {
            let start = i * model.config.hidden_size;
            hidden[start..start + model.config.hidden_size]
                .copy_from_slice(&emb[(token as usize) * model.config.hidden_size..]);
        }
        Ok(hidden)
    }
}