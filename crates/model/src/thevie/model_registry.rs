// crates/model/src/thevie/model_registry.rs
// =====================================================
// Model Registry v2.0 — Gestion Centralisée et Intelligente des Modèles
// Sélection dynamique selon tâche, coût, vitesse et préférences
// =====================================================

use std::collections::HashMap;
use tracing::{info, debug};

#[derive(Clone, Debug)]
pub struct ModelInfo {
    pub name: String,
    pub backend: String,
    pub model_id: String,
    pub cost_per_1k_tokens: f32,
    pub avg_quality: f32,      // 0.0 → 1.0
    pub avg_speed: f32,        // tokens/seconde
    pub specialties: Vec<String>,
    pub supports_lora: bool,
    pub is_local: bool,
    pub context_window: u32,   // tokens
}

pub struct ModelRegistry {
    models: HashMap<String, ModelInfo>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            models: HashMap::new(),
        };
        registry.register_default_models();
        info!("[ModelRegistry] {} modèles chargés", registry.models.len());
        registry
    }

    fn register_default_models(&mut self) {
        // === Modèles Locaux (Priorité) ===
        self.register(ModelInfo {
            name: "thevie-distilled-3b".to_string(),
            backend: "t369".to_string(),
            model_id: "thevie/distilled-3b".to_string(),
            cost_per_1k_tokens: 0.0,
            avg_quality: 0.81,
            avg_speed: 135.0,
            specialties: vec!["general".into(), "fast".into(), "thevie".into()],
            supports_lora: true,
            is_local: true,
            context_window: 8192,
        });

        self.register(ModelInfo {
            name: "llama-3.1-8b".to_string(),
            backend: "vllm".to_string(),
            model_id: "meta-llama/Meta-Llama-3.1-8B-Instruct".to_string(),
            cost_per_1k_tokens: 0.0,
            avg_quality: 0.84,
            avg_speed: 48.0,
            specialties: vec!["code".into(), "reasoning".into(), "general".into()],
            supports_lora: true,
            is_local: true,
            context_window: 32768,
        });

        // === Modèles Cloud ===
        self.register(ModelInfo {
            name: "gpt-4o".to_string(),
            backend: "openai".to_string(),
            model_id: "gpt-4o".to_string(),
            cost_per_1k_tokens: 0.005,
            avg_quality: 0.94,
            avg_speed: 92.0,
            specialties: vec!["code".into(), "creativity".into(), "multimodal".into()],
            supports_lora: false,
            is_local: false,
            context_window: 128000,
        });

        self.register(ModelInfo {
            name: "claude-3-5-sonnet".to_string(),
            backend: "anthropic".to_string(),
            model_id: "claude-3-5-sonnet-20241022".to_string(),
            cost_per_1k_tokens: 0.003,
            avg_quality: 0.96,
            avg_speed: 68.0,
            specialties: vec!["ethics".into(), "reasoning".into(), "analysis".into()],
            supports_lora: false,
            is_local: false,
            context_window: 200000,
        });

        // Modèle de fallback rapide
        self.register(ModelInfo {
            name: "deepseek-r1".to_string(),
            backend: "deepseek".to_string(),
            model_id: "deepseek-r1".to_string(),
            cost_per_1k_tokens: 0.0014,
            avg_quality: 0.89,
            avg_speed: 110.0,
            specialties: vec!["code".into(), "math".into(), "fast".into()],
            supports_lora: false,
            is_local: false,
            context_window: 64000,
        });
    }

    pub fn register(&mut self, model: ModelInfo) {
        info!("[ModelRegistry] Modèle enregistré → {} ({})", model.name, model.backend);
        self.models.insert(model.name.clone(), model);
    }

    /// Sélection intelligente du meilleur modèle selon le contexte
    pub fn get_best_model(&self, task_type: &str, prefer_local: bool, max_budget: f32) -> Option<&ModelInfo> {
        let mut best_model: Option<&ModelInfo> = None;
        let mut best_score = -1.0;

        for model in self.models.values() {
            if model.cost_per_1k_tokens > max_budget && !model.is_local {
                continue;
            }

            let mut score = model.avg_quality * 0.55 + (model.avg_speed / 150.0) * 0.35;

            // Bonus spécialité
            if model.specialties.iter().any(|s| s.contains(task_type)) {
                score += 0.18;
            }

            // Bonus local
            if prefer_local && model.is_local {
                score += 0.22;
            }

            // Pénalité coût (seulement pour les modèles cloud)
            if !model.is_local {
                score -= model.cost_per_1k_tokens * 45.0;
            }

            if score > best_score {
                best_score = score;
                best_model = Some(model);
            }
        }

        if let Some(m) = best_model {
            debug!("[ModelRegistry] Meilleur modèle choisi pour '{}' → {} (score: {:.3})", task_type, m.name, best_score);
        }

        best_model
    }

    pub fn get_model(&self, name: &str) -> Option<&ModelInfo> {
        self.models.get(name)
    }

    pub fn list_models(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    pub fn list_local_models(&self) -> Vec<&ModelInfo> {
        self.models.values().filter(|m| m.is_local).collect()
    }

    pub fn total_models(&self) -> usize {
        self.models.len()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}