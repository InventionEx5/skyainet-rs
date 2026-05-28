// crates/t369-inference/src/inference.rs
// =====================================================
// T369Inference v8.0 — ULTRA ULTRA PUISSANT
// Roman Neural Inference Engine - Version Finale Maximale
// =====================================================

use crate::model::{T369Model, ModelConfig};
use crate::kv_cache::KVCache;
use crate::speculative::{SpeculativeDecoder, SpeculativeConfig};
use crate::parallel::{ParallelExecutor, ParallelConfig, ParallelStrategy};
use crate::tokenizer::BpeTokenizer;
use tracing::{info, debug, warn};
use std::sync::Arc;
use std::sync::Mutex;

pub struct T369Inference {
    pub model: Arc<Mutex<T369Model>>,
    pub kv_cache: Option<KVCache>,
    pub use_kv_cache: bool,
    pub speculative_decoder: Option<SpeculativeDecoder>,
    pub parallel_executor: Option<ParallelExecutor>,
    pub parallel_mode: ParallelMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParallelMode {
    None,
    Pipeline,
    Tensor,
    Speculative,
}

impl T369Inference {
    pub fn new() -> Result<Self, String> {
        let config = ModelConfig::default();
        let model = T369Model::new(config);

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
            kv_cache: None,
            use_kv_cache: true,
            speculative_decoder: None,
            parallel_executor: None,
            parallel_mode: ParallelMode::None,
        })
    }

    // =====================================================
    // INITIALISATION
    // =====================================================

    pub fn init_kv_cache(&mut self) {
        if self.kv_cache.is_none() {
            let model = self.model.lock().unwrap();
            let cache = KVCache::new(
                model.config.num_layers,
                model.config.num_kv_heads,
                model.config.head_dim,
                model.config.max_seq_len,
            );
            self.kv_cache = Some(cache);
            info!("[Inference] KV Cache initialisé");
        }
    }

    pub fn enable_speculative_decoding(&mut self, config: SpeculativeConfig) {
        let model_config = self.model.lock().unwrap().config.clone();
        let speculative = SpeculativeDecoder::new(model_config, config);
        self.speculative_decoder = Some(speculative);
        self.parallel_mode = ParallelMode::Speculative;
        info!("[Inference] Speculative Decoding activé");
    }

    pub fn set_parallel_mode(&mut self, mode: ParallelMode) {
        self.parallel_mode = mode;
        match mode {
            ParallelMode::Pipeline => info!("[Inference] Mode Pipeline Parallel activé"),
            ParallelMode::Tensor => info!("[Inference] Mode Tensor Parallel activé"),
            ParallelMode::Speculative => info!("[Inference] Mode Speculative activé"),
            ParallelMode::None => {}
        }
    }

    pub fn enable_pipeline_parallel(&mut self) {
        let model = self.model.lock().unwrap().clone();
        let config = ParallelConfig {
            strategy: ParallelStrategy::Pipeline,
            num_workers: 4,
            pipeline_stages: 4,
            tensor_parallel_degree: 1,
        };
        let executor = ParallelExecutor::new(model, config);
        self.parallel_executor = Some(executor);
        self.parallel_mode = ParallelMode::Pipeline;
        info!("[Inference] Pipeline Parallel activé (4 stages)");
    }

    pub fn enable_tensor_parallel(&mut self) {
        let model = self.model.lock().unwrap().clone();
        let config = ParallelConfig {
            strategy: ParallelStrategy::Tensor,
            num_workers: 4,
            pipeline_stages: 1,
            tensor_parallel_degree: 4,
        };
        let executor = ParallelExecutor::new(model, config);
        self.parallel_executor = Some(executor);
        self.parallel_mode = ParallelMode::Tensor;
        info!("[Inference] Tensor Parallel activé (4 workers)");
    }

    // =====================================================
    // GÉNÉRATION ULTRA-PUISSANTE
    // =====================================================

    pub fn generate(
        &mut self,
        prompt: &str,
        max_new_tokens: usize,
    ) -> Result<String, String> {
        match self.parallel_mode {
            ParallelMode::Speculative => self.speculative_generate(prompt, max_new_tokens),
            ParallelMode::Pipeline | ParallelMode::Tensor => self.parallel_generate(prompt, max_new_tokens),
            _ => self.standard_generate(prompt, max_new_tokens),
        }
    }

    /// Génération standard ULTRA-PUISSANTE (tous les modules activés)
    fn standard_generate(
        &mut self,
        prompt: &str,
        max_new_tokens: usize,
    ) -> Result<String, String> {
        
        if self.use_kv_cache {
            self.init_kv_cache();
        }

        let tokenizer = {
            let model = self.model.lock().unwrap();
            model.tokenizer.clone().ok_or("Tokenizer non chargé")?
        };

        let mut tokens = tokenizer.encode(prompt);
        let mut generated_text = prompt.to_string();

        info!("[Inference] Génération ULTRA-PUISSANTE démarrée (MoE + CollectivIn + InSelf + InAware + InDream)");

        for step in 0..max_new_tokens {
            let logits = {
                let mut model = self.model.lock().unwrap();
                model.forward(&tokens)?
            };

            // === InAware : Conscience de l'incertitude ===
            let aware = {
                let mut model = self.model.lock().unwrap();
                model.in_aware.generate_with_awareness(&logits, &prompt, 1)
            };

            let next_token = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i as u32)
                .unwrap_or(0);

            tokens.push(next_token);

            if let Some(token_str) = tokenizer.decode(&[next_token]).split_whitespace().next() {
                generated_text.push_str(token_str);
            }

            if next_token == 1 { break; }

            // === InSelf : Auto-amélioration toutes les 5 itérations ===
            if step % 5 == 0 && step > 0 {
                let mut model = self.model.lock().unwrap();
                model.in_self.evolve_self();
            }

            if let Some(cache) = &mut self.kv_cache {
                if step % 8 == 0 { cache.clear(); }
            }
        }

        // === InSelf : Auto-évolution finale ===
        {
            let mut model = self.model.lock().unwrap();
            if model.in_self.is_evolving {
                model.in_self.evolve_self();
            }
        }

        info!("[Inference] Génération terminée | Tokens: {}", tokens.len());
        Ok(generated_text)
    }

    /// Génération avec Pipeline / Tensor Parallel
    fn parallel_generate(
        &mut self,
        prompt: &str,
        max_new_tokens: usize,
    ) -> Result<String, String> {
        
        let tokenizer = {
            let model = self.model.lock().unwrap();
            model.tokenizer.clone().ok_or("Tokenizer non chargé")?
        };

        let mut tokens = tokenizer.encode(prompt);
        let mut generated_text = prompt.to_string();

        info!("[Inference] Génération parallèle ULTRA (mode: {:?})", self.parallel_mode);

        for _ in 0..max_new_tokens {
            let logits = if let Some(executor) = &self.parallel_executor {
                executor.execute_parallel(&tokens)?
            } else {
                let mut model = self.model.lock().unwrap();
                model.forward(&tokens)?
            };

            let next_token = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i as u32)
                .unwrap_or(0);

            tokens.push(next_token);

            if let Some(token_str) = tokenizer.decode(&[next_token]).split_whitespace().next() {
                generated_text.push_str(token_str);
            }

            if next_token == 1 { break; }
        }

        Ok(generated_text)
    }

    /// Génération Speculative
    fn speculative_generate(
        &mut self,
        prompt: &str,
        max_new_tokens: usize,
    ) -> Result<String, String> {
        let tokenizer = {
            let model = self.model.lock().unwrap();
            model.tokenizer.clone().ok_or("Tokenizer non chargé")?
        };

        let prompt_tokens = tokenizer.encode(prompt);

        if let Some(decoder) = &mut self.speculative_decoder {
            let tokens = decoder.speculative_generate(&prompt_tokens, max_new_tokens)?;
            let mut generated_text = prompt.to_string();
            for &token in &tokens[prompt_tokens.len()..] {
                if let Some(token_str) = tokenizer.decode(&[token]).split_whitespace().next() {
                    generated_text.push_str(token_str);
                }
            }
            Ok(generated_text)
        } else {
            warn!("[Inference] Speculative non initialisé → fallback");
            self.standard_generate(prompt, max_new_tokens)
        }
    }

    // =====================================================
    // UTILITAIRES
    // =====================================================

    pub fn set_kv_cache_enabled(&mut self, enabled: bool) {
        self.use_kv_cache = enabled;
        if !enabled { self.kv_cache = None; }
    }

    pub fn load_tokenizer(&mut self, tokenizer: BpeTokenizer) {
        let mut model = self.model.lock().unwrap();
        model.set_tokenizer(tokenizer);
    }

    pub fn clear_kv_cache(&mut self) {
        if let Some(cache) = &mut self.kv_cache {
            cache.clear();
        }
    }

    /// Retourne les statistiques de tous les modules ultra-puissants
    pub fn get_ultra_stats(&self) -> String {
        let model = self.model.lock().unwrap();
        format!(
            "InSelf cycles: {} | Wisdom: {:.3} | CollectivIn fusions: {} | InAware confidence: {:.2}",
            model.in_self.self_improvement_cycles,
            model.in_self.cumulative_wisdom,
            model.collectiv_in.total_fusions,
            model.in_aware.average_confidence
        )
    }
}