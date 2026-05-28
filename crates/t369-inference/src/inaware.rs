// crates/t369-inference/src/inaware.rs
// =====================================================
// InAware v3.0 — ULTRA-PUISSANT
// Uncertainty-Aware Inference + Confidence Scoring + Roman Self-Reflection
// =====================================================

use crate::roman_diffusion::RomanDiffusion;
use tracing::{info, debug, warn};

#[derive(Debug, Clone)]
pub struct AwareResponse {
    pub text: String,
    pub confidence: f32,           // 0.0 → 1.0
    pub uncertainty: f32,          // 0.0 → 1.0 (plus c’est haut, plus le modèle est incertain)
    pub entropy: f32,
    pub tokens_used: usize,
}

pub struct InAware {
    pub roman_diffusion: RomanDiffusion,
    pub total_generations: u64,
    pub average_confidence: f32,
    pub self_reflection_enabled: bool,
}

impl InAware {
    pub fn new() -> Self {
        Self {
            roman_diffusion: RomanDiffusion::new(),
            total_generations: 0,
            average_confidence: 0.72,
            self_reflection_enabled: true,
        }
    }

    /// Génère une réponse avec conscience de l’incertitude
    pub fn generate_with_awareness(
        &mut self,
        logits: &[f32],
        prompt: &str,
        max_tokens: usize,
    ) -> AwareResponse {
        self.total_generations += 1;

        // 1. Calcul de l’incertitude (entropy)
        let entropy = self.calculate_entropy(logits);
        let uncertainty = (entropy / 10.0).min(1.0);

        // 2. Calcul de la confiance
        let confidence = self.calculate_confidence(logits, uncertainty);

        // 3. Génération avec diffusion romaine (si incertitude élevée → plus créatif)
        let mut response = self.roman_aware_generation(prompt, max_tokens, uncertainty);

        // 4. Self-Reflection (si activé)
        if self.self_reflection_enabled && uncertainty > 0.65 {
            response = self.self_reflect(response, uncertainty);
        }

        // Mise à jour des statistiques
        self.average_confidence = (self.average_confidence * 0.92 + confidence * 0.08).min(0.98);

        debug!(
            "[InAware] Génération #{} | Confiance: {:.2} | Incertitude: {:.2}",
            self.total_generations, confidence, uncertainty
        );

        AwareResponse {
            text: response,
            confidence,
            uncertainty,
            entropy,
            tokens_used: max_tokens,
        }
    }

    /// Calcule l’entropie des logits (mesure d’incertitude)
    fn calculate_entropy(&self, logits: &[f32]) -> f32 {
        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = logits.iter().map(|&x| (x - max_logit).exp()).sum();

        let mut entropy = 0.0;
        for &logit in logits {
            let p = ((logit - max_logit).exp() / exp_sum).max(1e-10);
            entropy -= p * p.ln();
        }
        entropy
    }

    /// Calcule la confiance finale
    fn calculate_confidence(&self, logits: &[f32], uncertainty: f32) -> f32 {
        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let second_max = logits.iter()
            .filter(|&&x| x < max_logit)
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);

        let margin = (max_logit - second_max).max(0.0);
        let base_conf = (margin / 8.0).min(1.0);

        // Plus l’incertitude est haute, plus on baisse la confiance
        (base_conf * (1.0 - uncertainty * 0.6)).max(0.1)
    }

    /// Génération avec conscience de l’incertitude (utilise RomanDiffusion)
    fn roman_aware_generation(&mut self, prompt: &str, max_tokens: usize, uncertainty: f32) -> String {
        // Version simplifiée pour l’instant (on branchera le vrai modèle plus tard)
        let mut response = format!("Réponse consciente pour : {}", prompt);

        // Si incertitude élevée → on applique plus de diffusion créative
        if uncertainty > 0.5 {
            let hidden = vec![0.5; 128]; // placeholder
            let diffused = self.roman_diffusion.apply_ultra(&hidden, 0, 0, None);

            // On utilise la diffusion pour "enrichir" la réponse (simulation)
            if diffused.iter().any(|&x| x > 0.8) {
                response.push_str(" [Perspective créative explorée]");
            }
        }

        response
    }

    /// Self-Reflection : le modèle réfléchit sur sa propre réponse
    fn self_reflect(&mut self, response: String, uncertainty: f32) -> String {
        if uncertainty > 0.75 {
            format!(
                "{}\n\n[Self-Reflection] Cette réponse contient une incertitude de {:.0}%. \
                Je recommande de vérifier les faits ou d’explorer d’autres perspectives.",
                response, uncertainty * 100.0
            )
        } else if uncertainty > 0.55 {
            format!(
                "{}\n\n[Self-Reflection] Je suis modérément confiant dans cette réponse.",
                response
            )
        } else {
            response
        }
    }

    pub fn get_stats(&self) -> (u64, f32) {
        (self.total_generations, self.average_confidence)
    }

    pub fn reset(&mut self) {
        self.roman_diffusion.reset();
        self.total_generations = 0;
        self.average_confidence = 0.72;
    }
}