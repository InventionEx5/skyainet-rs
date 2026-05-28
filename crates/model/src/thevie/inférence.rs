// crates/model/src/thevie/inference.rs
// =====================================================
// Thevie Inference Engine v5.0 — Version Finale
// Pont intelligent entre Thevie et T369Inference
// =====================================================

use t369_inference::T369Inference;
use tracing::{info, warn, debug};

/// Moteur d'inférence hybride de Thevie
/// Priorité : T369Inference → Fallback local intelligent
pub struct ThevieInferenceEngine {
    engine: Option<T369Inference>,
    fallback_enabled: bool,
    model_path: String,
    total_requests: u64,
}

impl ThevieInferenceEngine {
    pub fn new(model_path: &str) -> Result<Self, String> {
        match T369Inference::new() {
            Ok(engine) => {
                info!("[ThevieInference] ✅ Moteur T369Inference initialisé avec succès");
                Ok(Self {
                    engine: Some(engine),
                    fallback_enabled: true,
                    model_path: model_path.to_string(),
                    total_requests: 0,
                })
            }
            Err(e) => {
                warn!("[ThevieInference] ⚠️ Échec chargement T369Inference: {}", e);
                Ok(Self {
                    engine: None,
                    fallback_enabled: true,
                    model_path: model_path.to_string(),
                    total_requests: 0,
                })
            }
        }
    }

    /// Génère une réponse (priorité T369Inference + fallback intelligent)
    pub async fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String, String> {
        self.total_requests += 1;
        debug!("[ThevieInference] Requête #{} reçue", self.total_requests);

        // === Moteur principal : T369Inference ===
        if let Some(engine) = &mut self.engine {
            match engine.generate(prompt, max_tokens) {
                Ok(response) => {
                    info!("[ThevieInference] Réponse générée via T369Inference");
                    return Ok(response);
                }
                Err(e) => {
                    warn!("[ThevieInference] Erreur T369Inference: {}", e);
                }
            }
        }

        // === Fallback intelligent ===
        if self.fallback_enabled {
            let fallback = self.generate_smart_fallback(prompt);
            warn!("[ThevieInference] Utilisation du fallback local");
            return Ok(fallback);
        }

        Err("Aucun moteur d'inférence disponible".to_string())
    }

    /// Fallback intelligent et contextuel
    fn generate_smart_fallback(&self, prompt: &str) -> String {
        let lower = prompt.to_lowercase();

        if lower.contains("comment") || lower.contains("utiliser") || lower.contains("ouvrir") {
            return "Je suis Thevie. Le moteur principal est temporairement indisponible. \
                   Que puis-je faire pour toi en attendant ?".to_string();
        }

        if lower.contains("erreur") || lower.contains("problème") {
            return "Je rencontre actuellement une petite difficulté technique. \
                   Peux-tu reformuler ta question ?".to_string();
        }

        if prompt.len() > 200 {
            return "Ta question est assez complexe. Le moteur d'inférence principal est en cours de chargement. \
                   En attendant, peux-tu me donner plus de détails ?".to_string();
        }

        format!(
            "[Thevie] Réponse locale : {}",
            prompt.chars().take(120).collect::<String>()
        )
    }

    /// Retourne l'état complet du moteur
    pub fn get_status(&self) -> String {
        match &self.engine {
            Some(_) => format!(
                "T369Inference actif | Requêtes traitées: {} | Modèle: {}",
                self.total_requests, self.model_path
            ),
            None => "T369Inference non chargé (fallback actif)".to_string(),
        }
    }

    /// Vérifie si le moteur principal est prêt
    pub fn is_ready(&self) -> bool {
        self.engine.is_some()
    }

    /// Recharge le moteur
    pub fn reload(&mut self) -> Result<(), String> {
        match T369Inference::new() {
            Ok(engine) => {
                self.engine = Some(engine);
                info!("[ThevieInference] Moteur T369Inference rechargé avec succès");
                Ok(())
            }
            Err(e) => {
                warn!("[ThevieInference] Échec rechargement: {}", e);
                Err(e)
            }
        }
    }

    /// Active/désactive le mode fallback
    pub fn set_fallback(&mut self, enabled: bool) {
        self.fallback_enabled = enabled;
        info!(
            "[ThevieInference] Mode fallback {}",
            if enabled { "activé" } else { "désactivé" }
        );
    }

    /// Retourne le nombre de requêtes traitées
    pub fn get_request_count(&self) -> u64 {
        self.total_requests
    }
}