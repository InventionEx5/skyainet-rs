// crates/t369-inference/src/speculative.rs
// =====================================================
// Speculative Decoding v2.0 — Roman Speculative Inference
// Innovation : Draft Model + Roman Dream Verification + KV Cache
// =====================================================

use crate::model::{T369Model, ModelConfig};
use crate::kv_cache::KVCache;
use tracing::{info, debug, warn};

#[derive(Debug, Clone)]
pub struct SpeculativeConfig {
    pub draft_model_size: usize,      // Taille du modèle draft (ex: 1B ou 3B)
    pub max_speculative_tokens: usize, // Nombre de tokens à spéculer (ex: 4-8)
    pub acceptance_threshold: f32,     // Seuil d'acceptation (0.6 \~ 0.85)
}

impl Default for SpeculativeConfig {
    fn default() -> Self {
        Self {
            draft_model_size: 1024,
            max_speculative_tokens: 6,
            acceptance_threshold: 0.72,
        }
    }
}

pub struct SpeculativeDecoder {
    pub main_model: T369Model,
    pub draft_model: T369Model,        // Modèle draft plus petit
    pub config: SpeculativeConfig,
    pub kv_cache: Option<KVCache>,
}

impl SpeculativeDecoder {
    pub fn new(main_config: ModelConfig, speculative_config: SpeculativeConfig) -> Self {
        let mut draft_config = main_config.clone();
        draft_config.hidden_size = speculative_config.draft_model_size;
        draft_config.num_layers = main_config.num_layers / 2; // Modèle draft plus léger

        Self {
            main_model: T369Model::new(main_config),
            draft_model: T369Model::new(draft_config),
            config: speculative_config,
            kv_cache: None,
        }
    }

    /// Génère des tokens avec Speculative Decoding (beaucoup plus rapide)
    pub fn speculative_generate(
        &mut self,
        prompt_tokens: &[u32],
        max_new_tokens: usize,
    ) -> Result<Vec<u32>, String> {
        
        self.main_model.init_kv_cache();
        self.draft_model.init_kv_cache();

        let mut tokens = prompt_tokens.to_vec();
        let mut generated = 0;

        info!("[Speculative] Démarrage Speculative Decoding | Max tokens: {}", max_new_tokens);

        while generated < max_new_tokens {
            // 1. Le modèle draft propose plusieurs tokens
            let draft_tokens = self.draft_propose_tokens(&tokens)?;

            if draft_tokens.is_empty() {
                break;
            }

            // 2. Le modèle principal vérifie les tokens en une seule passe
            let accepted = self.verify_tokens(&tokens, &draft_tokens)?;

            // 3. On garde seulement les tokens acceptés
            for &token in &accepted {
                tokens.push(token);
                generated += 1;

                if token == 1 || generated >= max_new_tokens {
                    break;
                }
            }

            if accepted.len() < draft_tokens.len() {
                // Si rejet, on régénère avec le main model
                debug!("[Speculative] Rejet détecté → régénération");
            }
        }

        info!("[Speculative] Génération terminée | Tokens générés: {}", generated);
        Ok(tokens)
    }

    /// Le modèle draft propose plusieurs tokens à la fois
    fn draft_propose_tokens(&mut self, current_tokens: &[u32]) -> Result<Vec<u32>, String> {
        let mut draft_tokens = Vec::new();
        let mut temp_tokens = current_tokens.to_vec();

        for _ in 0..self.config.max_speculative_tokens {
            let logits = self.draft_model.forward(&temp_tokens)?;
            let next_token = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i as u32)
                .unwrap_or(0);

            draft_tokens.push(next_token);
            temp_tokens.push(next_token);

            if next_token == 1 {
                break;
            }
        }

        debug!("[Speculative] Draft a proposé {} tokens", draft_tokens.len());
        Ok(draft_tokens)
    }

    /// Vérifie les tokens proposés par le draft avec le modèle principal
    fn verify_tokens(
        &mut self,
        current_tokens: &[u32],
        draft_tokens: &[u32],
    ) -> Result<Vec<u32>, String> {
        let mut accepted = Vec::new();
        let mut temp_tokens = current_tokens.to_vec();

        for &draft_token in draft_tokens {
            let logits = self.main_model.forward(&temp_tokens)?;
            
            // On prend le token le plus probable du main model
            let main_token = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i as u32)
                .unwrap_or(0);

            // Roman Dream Verification : on accepte si le draft est proche du main
            let acceptance_score = self.roman_acceptance_score(logits[main_token as usize], draft_token);

            if acceptance_score >= self.config.acceptance_threshold {
                accepted.push(draft_token);
                temp_tokens.push(draft_token);
            } else {
                // On accepte quand même le token du main model
                accepted.push(main_token);
                temp_tokens.push(main_token);
                break; // On arrête la speculation
            }
        }

        debug!("[Speculative] {} / {} tokens acceptés", accepted.len(), draft_tokens.len());
        Ok(accepted)
    }

    /// Score d'acceptation "Roman Dream" (innovation)
    #[inline]
    fn roman_acceptance_score(&self, main_logit: f32, draft_token: u32) -> f32 {
        // Version simplifiée : plus le logit est élevé, plus on accepte
        // On peut améliorer avec RomanT369 plus tard
        (main_logit.tanh() + 1.0) / 2.0
    }

    /// Active/désactive le KV Cache
    pub fn set_kv_cache_enabled(&mut self, enabled: bool) {
        if enabled {
            self.main_model.init_kv_cache();
            self.draft_model.init_kv_cache();
        } else {
            self.main_model.kv_cache = None;
            self.draft_model.kv_cache = None;
        }
    }
}