// crates/model/src/thevie/lora_evolution.rs
// =====================================================
// LoraÉvo v4.1 — Guide Intelligent & Auto-Évolutif
// Connecté à T369Inference + Apprentissage Continu + Adaptation Dynamique
// =====================================================

use t369_inference::T369Inference;
use tracing::{info, warn, debug};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionProfile {
    pub ethics: f32,
    pub technical: f32,
    pub creativity: f32,
    pub wisdom: f32,
    pub user_alignment: f32,
}

pub struct LoraÉvo {
    pub model_name: String,
    inference_engine: Option<T369Inference>,
    
    // Mémoire intelligente
    short_term_memory: VecDeque<String>,      // Dernières interactions
    long_term_knowledge: Vec<String>,         // Connaissances consolidées
    evolution_profile: EvolutionProfile,
    
    pub total_interactions: u64,
    pub evolution_score: f32,
    pub current_specialization: String,
    last_adaptation: u64,
}

impl LoraÉvo {
    pub fn new() -> Self {
        Self {
            model_name: "LoraÉvo v4.1".to_string(),
            inference_engine: None,
            short_term_memory: VecDeque::with_capacity(40),
            long_term_knowledge: Vec::new(),
            evolution_profile: EvolutionProfile {
                ethics: 0.82,
                technical: 0.75,
                creativity: 0.78,
                wisdom: 0.80,
                user_alignment: 0.85,
            },
            total_interactions: 0,
            evolution_score: 0.68,
            current_specialization: "Guide Polyvalent".to_string(),
            last_adaptation: Self::now_millis(),
        }
    }

    /// Connexion au moteur principal T369Inference
    pub fn connect_to_inference(&mut self, engine: T369Inference) {
        self.inference_engine = Some(engine);
        info!("[LoraÉvo] Connecté avec succès au moteur T369Inference");
    }

    /// Génération avec apprentissage en temps réel
    pub async fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String, String> {
        if let Some(engine) = &mut self.inference_engine {
            let enhanced_prompt = self.build_contextual_prompt(prompt);

            match engine.generate(&enhanced_prompt, max_tokens).await {
                Ok(response) => {
                    self.learn_from_interaction(prompt, &response);
                    self.total_interactions += 1;
                    self.adapt_evolution();

                    debug!("[LoraÉvo] Réponse générée | Score évolution: {:.3} | Interactions: {}", 
                           self.evolution_score, self.total_interactions);
                    
                    Ok(response)
                }
                Err(e) => {
                    warn!("[LoraÉvo] Erreur T369Inference: {}", e);
                    Err(e)
                }
            }
        } else {
            Err("LoraÉvo n'est pas connectée au moteur d'inférence".to_string())
        }
    }

    /// Construction d'un prompt enrichi avec mémoire et profil
    fn build_contextual_prompt(&self, prompt: &str) -> String {
        let mut context = String::new();

        // Mémoire récente
        if !self.short_term_memory.is_empty() {
            context.push_str("\n[Contexte récent]:\n");
            for entry in self.short_term_memory.iter().rev().take(4) {
                context.push_str(&format!("- {}\n", entry));
            }
        }

        // Profil actuel
        context.push_str(&format!(
            "\n[Profil LoraÉvo] Spécialisation: {} | Score évolution: {:.2}\n",
            self.current_specialization, self.evolution_score
        ));

        format!(
            "Tu es LoraÉvo v4.1, un assistant intelligent, bienveillant et auto-évolutif de SkyAInet.\n\
             Tu apprends en continu et t'adaptes à l'utilisateur.\n\n\
             {}\nUtilisateur : {}\nLoraÉvo :",
            context.trim(), prompt
        )
    }

    /// Apprentissage après chaque interaction
    fn learn_from_interaction(&mut self, prompt: &str, response: &str) {
        // Mise à jour mémoire courte
        self.short_term_memory.push_back(format!("Q: {} | R: {}", prompt, response.chars().take(80).collect::<String>()));
        if self.short_term_memory.len() > 40 {
            self.short_term_memory.pop_front();
        }

        // Mise à jour mémoire longue (leçons importantes)
        if response.len() > 120 {
            self.long_term_knowledge.push(response.to_string());
            if self.long_term_knowledge.len() > 25 {
                self.long_term_knowledge.remove(0);
            }
        }

        // Adaptation de spécialisation
        let lower_prompt = prompt.to_lowercase();
        if lower_prompt.contains("code") || lower_prompt.contains("rust") || lower_prompt.contains("technique") {
            self.current_specialization = "Technique & Programmation".to_string();
        } else if lower_prompt.contains("éthique") || lower_prompt.contains("philosoph") {
            self.current_specialization = "Éthique & Philosophie".to_string();
        } else if lower_prompt.contains("créatif") || lower_prompt.contains("rêve") || lower_prompt.contains("histoire") {
            self.current_specialization = "Créativité & Imagination".to_string();
        }
    }

    /// Mise à jour du score d'évolution
    fn adapt_evolution(&mut self) {
        if self.total_interactions % 15 == 0 {
            self.evolution_score = (self.evolution_score + 0.009).min(0.98);
        }
    }

    fn now_millis() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    /// État complet de LoraÉvo
    pub fn get_status(&self) -> String {
        format!(
            "LoraÉvo v4.1 | Évolution: {:.3} | Interactions: {} | Spécialisation: {} | Mémoire: {} court / {} long",
            self.evolution_score,
            self.total_interactions,
            self.current_specialization,
            self.short_term_memory.len(),
            self.long_term_knowledge.len()
        )
    }
}

impl Default for LoraÉvo {
    fn default() -> Self {
        Self::new()
    }
}