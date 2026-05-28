// crates/model/src/thevie/inference.rs
// =====================================================
// Hybrid Inference Engine - Thevie v4.1 (Version Complète)
// Compatible avec T369Inference (Roman Neural Inference)
// =====================================================

use t369_inference::T369InferenceEngine;
use tracing::{info, warn};

/// Moteur d'inférence hybride de Thevie
pub struct HybridInferenceEngine {
    engine: Option<T369InferenceEngine>,
    fallback_enabled: bool,
    model_path: String,
}

impl HybridInferenceEngine {
    pub fn new(model_path: &str, _lora_path: Option<&str>) -> Result<Self, String> {
        match T369InferenceEngine::new(model_path) {
            Ok(engine) => {
                info!("[Inference] Moteur T369Inference chargé avec succès");
                Ok(Self {
                    engine: Some(engine),
                    fallback_enabled: true,
                    model_path: model_path.to_string(),
                })
            }
            Err(e) => {
                warn!("[Inference] Échec chargement T369Inference: {}", e);
                Ok(Self {
                    engine: None,
                    fallback_enabled: true,
                    model_path: model_path.to_string(),
                })
            }
        }
    }

    /// Génère une réponse (avec fallback intelligent)
    pub async fn generate(
        &mut self,
        prompt: &str,
        max_tokens: u32,
        _lora_adapter: Option<String>,
    ) -> Result<String, String> {
        
        // TODO implémenté : Support LoRA futur (désactivé pour l'instant)
        // if let Some(lora_name) = lora_adapter {
        //     self.apply_lora(&lora_name);
        // }

        // === Utilisation du moteur T369Inference ===
        if let Some(engine) = &mut self.engine {
            match engine.generate(prompt, max_tokens as usize) {
                Ok(response) => {
                    info!("[Inference] Réponse générée via T369Inference");
                    return Ok(response);
                }
                Err(e) => {
                    warn!("[Inference] Erreur T369Inference: {}", e);
                }
            }
        }

        // === Fallback (si le moteur principal échoue) ===
        if self.fallback_enabled {
            warn!("[Inference] Utilisation du fallback local");
            return Ok(self.local_fallback(prompt));
        }

        Err("Aucun moteur d'inférence disponible".to_string())
    }

    /// Réponse de fallback intelligente
    fn local_fallback(&self, prompt: &str) -> String {
        if prompt.to_lowercase().contains("comment") || 
           prompt.to_lowercase().contains("utiliser") || 
           prompt.to_lowercase().contains("ouvrir") {
            return "Je suis Thevie. Pour l'instant, je réponds en mode fallback. \
                   Le moteur T369Inference est en cours de chargement.".to_string();
        }

        format!(
            "[Thevie Fallback] Réponse générée localement pour : {}",
            prompt.chars().take(80).collect::<String>()
        )
    }

    /// Retourne l'état du moteur
    pub fn get_status(&self) -> String {
        if self.engine.is_some() {
            format!("T369Inference actif | Modèle: {}", self.model_path)
        } else {
            "T369Inference non chargé (fallback actif)".to_string()
        }
    }

    /// Recharge le moteur (méthode implémentée)
    pub fn reload(&mut self) -> Result<(), String> {
        match T369InferenceEngine::new(&self.model_path) {
            Ok(engine) => {
                self.engine = Some(engine);
                info!("[Inference] Moteur T369Inference rechargé avec succès");
                Ok(())
            }
            Err(e) => {
                warn!("[Inference] Échec rechargement: {}", e);
                Err(e)
            }
        }
    }

    /// Active ou désactive le mode fallback
    pub fn set_fallback(&mut self, enabled: bool) {
        self.fallback_enabled = enabled;
        info!("[Inference] Fallback {}", if enabled { "activé" } else { "désactivé" });
    }

    /// Retourne le chemin du modèle actuel
    pub fn get_model_path(&self) -> &str {
        &self.model_path
    }
}