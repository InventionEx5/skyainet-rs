// crates/model/src/thevie/model_registry.rs
// =====================================================
// Model Registry v1.0 — Gestion Centralisée des Modèles
// =====================================================

use std::collections::HashMap;
use tracing::info;

#[derive(Clone, Debug)]
pub struct ModelInfo {
    pub name: String,
    pub backend: String,
    pub model_id: String,           // ex: "gpt-4o", "claude-3-5-sonnet", "llama-3.1-8b"
    pub cost_per_1k_tokens: f32,
    pub avg_quality: f32,           // 0.0 → 1.0
    pub avg_speed: f32,             // tokens/seconde
    pub specialties: Vec<String>,   // ex: ["ethics", "code", "reasoning"]
    pub supports_lora: bool,
    pub is_local: bool,
}

pub struct ModelRegistry {
    pub models: HashMap<String, ModelInfo>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            models: HashMap::new(),
        };

        // Modèles par défaut
        registry.register_default_models();
        registry
    }

    fn register_default_models(&mut self) {
        // OpenAI
        self.register_model(ModelInfo {
            name: "gpt-4o".to_string(),
            backend: "openai".to_string(),
            model_id: "gpt-4o".to_string(),
            cost_per_1k_tokens: 0.005,
            avg_quality: 0.94,
            avg_speed: 85.0,
            specialties: vec!["code".to_string(), "reasoning".to_string(), "creativity".to_string()],
            supports_lora: false,
            is_local: false,
        });

        // Anthropic
        self.register_model(ModelInfo {
            name: "claude-3-5-sonnet".to_string(),
            backend: "anthropic".to_string(),
            model_id: "claude-3-5-sonnet-20241022".to_string(),
            cost_per_1k_tokens: 0.003,
            avg_quality: 0.95,
            avg_speed: 72.0,
            specialties: vec!["ethics".to_string(), "reasoning".to_string(), "analysis".to_string()],
            supports_lora: false,
            is_local: false,
        });

        // vLLM Local
        self.register_model(ModelInfo {
            name: "llama-3.1-8b".to_string(),
            backend: "vllm".to_string(),
            model_id: "meta-llama/Meta-Llama-3.1-8B-Instruct".to_string(),
            cost_per_1k_tokens: 0.0,
            avg_quality: 0.82,
            avg_speed: 45.0,
            specialties: vec!["general".to_string(), "code".to_string()],
            supports_lora: true,
            is_local: true,
        });

        // Modèle distillé (exemple)
        self.register_model(ModelInfo {
            name: "thevie-distilled-3b".to_string(),
            backend: "vllm".to_string(),
            model_id: "thevie/distilled-3b".to_string(),
            cost_per_1k_tokens: 0.0,
            avg_quality: 0.78,
            avg_speed: 120.0,
            specialties: vec!["general".to_string(), "fast".to_string()],
            supports_lora: true,
            is_local: true,
        });
    }

    pub fn register_model(&mut self, model: ModelInfo) {
        info!("[ModelRegistry] Modèle enregistré : {}", model.name);
        self.models.insert(model.name.clone(), model);
    }

    /// Retourne le meilleur modèle selon le type de tâche
    pub fn get_best_model_for_task(&self, task_type: &str, prefer_local: bool) -> Option<&ModelInfo> {
        let mut best_model: Option<&ModelInfo> = None;
        let mut best_score = 0.0;

        for model in self.models.values() {
            let mut score = model.avg_quality * 0.6 + (model.avg_speed / 150.0) * 0.4;

            // Bonus si le modèle est spécialisé dans la tâche
            if model.specialties.iter().any(|s| s.contains(task_type)) {
                score += 0.15;
            }

            // Bonus si on préfère les modèles locaux
            if prefer_local && model.is_local {
                score += 0.10;
            }

            // Pénalité de coût (sauf si local)
            if !model.is_local {
                score -= model.cost_per_1k_tokens * 50.0;
            }

            if score > best_score {
                best_score = score;
                best_model = Some(model);
            }
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
}