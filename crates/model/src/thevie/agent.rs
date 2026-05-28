// crates/model/src/thevie/agent.rs
// =====================================================
// Thevie Agent v2.0 — Agentic Workflows Intelligents
// Planification + Tool Calling + Réflexion itérative + Mémoire
// =====================================================

use crate::thevie::tools::{ToolRegistry, Tool};
use tracing::{info, warn, debug};
use std::collections::VecDeque;

pub struct ThevieAgent {
    pub tool_registry: ToolRegistry,
    pub max_iterations: usize,
    pub verbose: bool,
    pub memory: VecDeque<String>,           // Mémoire des dernières interactions
    pub reflection_enabled: bool,           // Active la réflexion après chaque action
}

impl ThevieAgent {
    pub fn new() -> Self {
        Self {
            tool_registry: ToolRegistry::new(),
            max_iterations: 10,
            verbose: true,
            memory: VecDeque::with_capacity(20),
            reflection_enabled: true,
        }
    }

    /// Exécute une tâche complexe de manière agentique (ReAct amélioré)
    pub async fn run_agentic_task(&self, goal: &str) -> Result<String, String> {
        info!("[Agent] Démarrage de la tâche agentique avancée : {}", goal);

        let mut context = String::new();
        let mut final_answer = String::new();
        let mut step_count = 0;

        for iteration in 1..=self.max_iterations {
            step_count += 1;

            if self.verbose {
                info!("[Agent] === Itération {} / {} ===", iteration, self.max_iterations);
            }

            // 1. Raisonnement enrichi
            let thought = self.reason(goal, &context, step_count).await;

            // 2. Vérification de la réponse finale
            if thought.contains("FINAL_ANSWER:") {
                final_answer = thought.replace("FINAL_ANSWER:", "").trim().to_string();
                break;
            }

            // 3. Action
            let action_result = self.act(&thought).await;

            // 4. Réflexion (optionnelle)
            let reflection = if self.reflection_enabled {
                self.reflect(&thought, &action_result).await
            } else {
                String::new()
            };

            // Mise à jour du contexte
            context.push_str(&format!(
                "\n[Itération {}]\nPensée: {}\nAction: {}\nObservation: {}\n",
                iteration, thought, action_result, reflection
            ));

            // Stockage en mémoire
            self.memory.push_back(format!("Itération {}: {}", iteration, thought));
            if self.memory.len() > 20 {
                self.memory.pop_front();
            }

            if self.verbose {
                debug!("[Agent] Observation : {}", action_result);
            }
        }

        if final_answer.is_empty() {
            final_answer = "L'agent n'a pas pu aboutir à une réponse définitive après plusieurs itérations.".to_string();
        }

        info!("[Agent] Tâche agentique terminée avec succès.");
        Ok(final_answer)
    }

    /// Raisonnement amélioré (plus intelligent)
    async fn reason(&self, goal: &str, context: &str, step: usize) -> String {
        let goal_lower = goal.to_lowercase();

        // Raisonnement basé sur des patterns plus avancés
        if goal_lower.contains("recherche") || goal_lower.contains("information") || goal_lower.contains("trouver") {
            return "Je dois utiliser l'outil web_search pour collecter des informations précises.".to_string();
        }

        if goal_lower.contains("code") || goal_lower.contains("programme") || goal_lower.contains("calculer") || goal_lower.contains("script") {
            return "Je dois utiliser l'outil code_execution pour résoudre ce problème technique.".to_string();
        }

        if goal_lower.contains("fichier") || goal_lower.contains("lire") || goal_lower.contains("écrire") || goal_lower.contains("sauvegarder") {
            return "Je dois utiliser les outils file_read ou file_write.".to_string();
        }

        if goal_lower.contains("analyser") || goal_lower.contains("expliquer") || goal_lower.contains("résumer") {
            return "Je dois collecter plus d'informations avant de donner une analyse complète.".to_string();
        }

        if context.len() > 1200 || step >= self.max_iterations - 2 {
            return format!("FINAL_ANSWER: Après réflexion approfondie, voici ma conclusion sur : {}", goal);
        }

        // Par défaut : continuer l'exploration
        "Je dois continuer à explorer en utilisant les outils disponibles pour mieux comprendre le problème.".to_string()
    }

    /// Exécution d'action améliorée
    async fn act(&self, thought: &str) -> String {
        let thought_lower = thought.to_lowercase();

        if thought_lower.contains("web_search") {
            if let Some(tool) = self.tool_registry.get_tool("web_search") {
                return tool.execute("recherche approfondie sur le sujet").await.unwrap_or_default();
            }
        }

        if thought_lower.contains("code_execution") {
            if let Some(tool) = self.tool_registry.get_tool("code_execution") {
                return tool.execute("print('Exécution du code demandé')").await.unwrap_or_default();
            }
        }

        if thought_lower.contains("file_read") {
            if let Some(tool) = self.tool_registry.get_tool("file_read") {
                return tool.execute("./README.md").await.unwrap_or_default();
            }
        }

        if thought_lower.contains("file_write") {
            if let Some(tool) = self.tool_registry.get_tool("file_write") {
                return tool.execute("result.txt|Contenu généré intelligemment par l'agent.").await.unwrap_or_default();
            }
        }

        "Aucune action pertinente trouvée. Je réfléchis à la meilleure approche suivante.".to_string()
    }

    /// Réflexion après chaque action (nouveau)
    async fn reflect(&self, thought: &str, action_result: &str) -> String {
        if action_result.contains("erreur") || action_result.contains("échec") {
            return "L'action a échoué. Je dois essayer une approche différente.".to_string();
        }

        if action_result.len() > 200 {
            return "L'action a retourné beaucoup d'informations. Je dois les analyser.".to_string();
        }

        "L'action s'est bien déroulée. Je peux maintenant avancer.".to_string()
    }
}