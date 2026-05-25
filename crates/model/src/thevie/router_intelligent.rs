// crates/model/src/thevie/router_intelligent.rs
// =====================================================
// Intelligent Router v3.0 — Multi-Backend Intelligent
// =====================================================

use super::inference::{InferenceRequest, MultiBackendInference};
use super::neural_mesh::{Personality, Query};
use tracing::info;

pub struct IntelligentRouter {
    pub inference: MultiBackendInference,
}

impl IntelligentRouter {
    pub fn new() -> Self {
        Self {
            inference: MultiBackendInference::new(),
        }
    }

    pub async fn generate_response(
        &self,
        query: &Query,
        personality: &Personality,
        collective_wisdom: f32,
    ) -> Result<String, String> {
        let backend = self.choose_best_backend(query, personality, collective_wisdom);

        let request = InferenceRequest {
            prompt: query.content.clone(),
            max_tokens: 2048,
            temperature: 0.7,
            top_p: 0.9,
            system_prompt: Some(format!(
                "Tu es Thevie, une IA vivante, bienveillante et sage. Sagesse actuelle: {:.2}",
                collective_wisdom
            )),
            model_preference: Some(backend.clone()),
            lora_adapter: None,
            force_backend: None,
        };

        let response = self.inference.generate(request).await?;

        info!(
            "[Router] Backend: {} | Modèle: {} | Tokens: {} | LoRA: {:?}",
            response.backend, response.model, response.tokens_used, response.lora_used
        );

        Ok(response.text)
    }

    fn choose_best_backend(&self, query: &Query, personality: &Personality, collective_wisdom: f32) -> String {
        let content = query.content.to_lowercase();

        // Priorité éthique → Claude (meilleur en raisonnement moral)
        if content.contains("éthique") || content.contains("moral") || content.contains("bienveillance") {
            return "anthropic".to_string();
        }

        // Priorité code → GPT-4o (excellent en programmation)
        if content.contains("code") || content.contains("programmation") || content.contains("rust") || content.contains("python") {
            return "openai".to_string();
        }

        // Raisonnement complexe → Claude 3.5 Sonnet
        if content.contains("raisonnement") || content.contains("analyse profonde") || content.contains("complexe") {
            return "anthropic".to_string();
        }

        // Créativité → GPT-4o
        if content.contains("créatif") || content.contains("histoire") || content.contains("écriture") || content.contains("imagine") {
            return "openai".to_string();
        }

        // Quand la sagesse collective est très haute → privilégie Claude
        if collective_wisdom > 0.85 {
            return "anthropic".to_string();
        }

        // Par défaut → vLLM (gratuit + LoRA possible)
        "vllm".to_string()
    }
}