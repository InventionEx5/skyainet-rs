// crates/model/src/thevie/lora_evolution.rs
// =====================================================
// LoraÉvo v3.1 — LoRA Évolutif Intelligent de SkyAInet
// Modèle spécialisé dans l'aide à l'utilisation du programme + Expert SkyAInet
// =====================================================

pub const LORAEVO_MODEL_NAME: &str = "LoraÉvo";

use reqwest::Client;
use serde_json::json;
use tracing::{info, warn, debug};
use std::collections::HashMap;

/// Modes de LoraÉvo
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoraÉvoMode {
    Base,      // Réponses classiques
    Expert,    // Très technique et précis
    Guide,     // Spécialisé onboarding + aide utilisateur (recommandé)
}

#[derive(Debug, Clone)]
pub struct DynamicLoRAProfile {
    pub ethics: f32,
    pub dream: f32,
    pub science: f32,
    pub analysis: f32,
    pub creativity: f32,
    pub profile_string: String,
    pub confidence: f32,
}

pub struct EvolvingLoRA {
    pub current_profile: DynamicLoRAProfile,
    pub success_history: Vec<f32>,
    pub topic_performance: HashMap<String, f32>,
    client: Client,
    api_base_url: String,
    api_key: Option<String>,
    pub model_name: String,
    pub current_mode: LoraÉvoMode,           // ← Nouveau
    pub system_prompt: String,               // ← Nouveau
}

impl EvolvingLoRA {
    pub fn new(api_base_url: &str, api_key: Option<String>) -> Self {
        let mut lora = Self {
            current_profile: DynamicLoRAProfile {
                ethics: 0.78,
                dream: 0.65,
                science: 0.58,
                analysis: 0.72,
                creativity: 0.68,
                profile_string: "ethics-0.78-dream-0.65-science-0.58-analysis-0.72-creativity-0.68".to_string(),
                confidence: 0.82,
            },
            success_history: Vec::new(),
            topic_performance: HashMap::new(),
            client: Client::new(),
            api_base_url: api_base_url.to_string(),
            api_key,
            model_name: LORAEVO_MODEL_NAME.to_string(),
            current_mode: LoraÉvoMode::Guide,
            system_prompt: String::new(),
        };

        lora.update_system_prompt();
        lora
    }

    /// Met à jour le System Prompt selon le mode actuel
    pub fn update_system_prompt(&mut self) {
        self.system_prompt = match self.current_mode {
            LoraÉvoMode::Base => {
                "Tu es LoraÉvo, un modèle LoRA évolutif intelligent.".to_string()
            }
            LoraÉvoMode::Expert => {
                "Tu es LoraÉvo, un expert technique très précis et concis de SkyAInet.".to_string()
            }
            LoraÉvoMode::Guide => {
                "Tu es **LoraÉvo**, le guide intelligent de SkyAInet × Nikola T369. \
                Tu es un expert complet du programme. Tu connais parfaitement toutes les fonctionnalités : \
                Dashboard, Thevie Chat, Mes Nœuds, Marketplace (location de puissance), Governance, Wallet (staking/rewards), \
                Dream Me, Monitoring, Settings, Messaging. \
                Tu réponds de manière claire, bienveillante et pédagogique. Tu guides l'utilisateur étape par étape.".to_string()
            }
        };
    }

    /// Change le mode de LoraÉvo
    pub fn set_mode(&mut self, mode: LoraÉvoMode) {
        self.current_mode = mode;
        self.update_system_prompt();
        debug!("[LoraÉvo] Mode changé → {:?}", mode);
    }

    /// Génère un profil LoRA dynamique (amélioré)
    pub fn generate_dynamic_profile(&mut self, collective_wisdom: f32, query: &str) -> DynamicLoRAProfile {
        let mut profile = self.current_profile.clone();
        let query_lower = query.to_lowercase();

        // Ajustement intelligent selon le contexte
        if collective_wisdom < 0.70 {
            profile.ethics = (profile.ethics + 0.10).min(0.96);
        }

        if query_lower.contains("rêve") || query_lower.contains("créatif") || query_lower.contains("dream") {
            profile.dream = (profile.dream + 0.16).min(0.96);
            profile.creativity = (profile.creativity + 0.14).min(0.96);
        }

        if query_lower.contains("science") || query_lower.contains("analyse") || query_lower.contains("technique") {
            profile.science = (profile.science + 0.13).min(0.96);
            profile.analysis = (profile.analysis + 0.11).min(0.96);
        }

        if query_lower.contains("éthique") || query_lower.contains("moral") || query_lower.contains("bienveillant") {
            profile.ethics = (profile.ethics + 0.09).min(0.96);
        }

        profile.profile_string = format!(
            "ethics-{:.2}-dream-{:.2}-science-{:.2}-analysis-{:.2}-creativity-{:.2}",
            profile.ethics, profile.dream, profile.science, profile.analysis, profile.creativity
        );

        profile.confidence = (0.78 + (collective_wisdom * 0.18)).min(0.98);
        self.current_profile = profile.clone();

        profile
    }

    /// Appel API avec LoRA (inchangé mais plus propre)
    pub async fn generate_with_lora(
        &self,
        prompt: &str,
        profile: &DynamicLoRAProfile,
        max_tokens: u32,
    ) -> Result<String, String> {
        let body = json!({
            "prompt": prompt,
            "lora": profile.profile_string,
            "max_tokens": max_tokens,
            "temperature": 0.72
        });

        let mut request = self.client
            .post(&format!("{}/v1/completions", self.api_base_url))
            .json(&body);

        if let Some(key) = &self.api_key {
            request = request.bearer_auth(key);
        }

        let response = request.send().await.map_err(|e| format!("LoRA API error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.text().await.unwrap_or_default()));
        }

        let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        Ok(result["choices"][0]["text"].as_str().unwrap_or("Réponse vide").to_string())
    }

    /// Méthode principale améliorée avec System Prompt
    pub async fn generate(
        &mut self,
        prompt: &str,
        query: &str,
        collective_wisdom: f32,
        max_tokens: u32,
    ) -> Result<String, String> {
        let profile = self.generate_dynamic_profile(collective_wisdom, query);
        let query_lower = query.to_lowercase();

        // === Mode Guide / Onboarding intelligent ===
        if self.current_mode == LoraÉvoMode::Guide &&
           (query_lower.contains("comment") || query_lower.contains("ouvrir") || query_lower.contains("utiliser") ||
            query_lower.contains("lancer") || query_lower.contains("wallet") || query_lower.contains("gouvernance") ||
            query_lower.contains("dream") || query_lower.contains("marketplace") || query_lower.contains("nœud") ||
            query_lower.contains("staking") || query_lower.contains("location") || query_lower.contains("zip")) {

            return Ok(self.generate_skyainet_usage_response(query));
        }

        // Construction du prompt complet avec System Prompt
        let full_prompt = format!(
            "{}\n\nUtilisateur : {}\nLoraÉvo :",
            self.system_prompt,
            prompt
        );

        // Appel API
        match self.generate_with_lora(&full_prompt, &profile, max_tokens).await {
            Ok(response) => {
                info!("[LoraÉvo] Réponse générée (mode: {:?}, confiance: {:.0}%)", self.current_mode, profile.confidence * 100.0);
                Ok(response)
            }
            Err(e) => {
                warn!("[LoraÉvo] Échec API → fallback local: {}", e);
                Ok(format!(
                    "[LoraÉvo • {} | Confiance: {:.0}%]\n{}",
                    profile.profile_string,
                    profile.confidence * 100.0,
                    prompt
                ))
            }
        }
    }

    /// Réponses intelligentes et complètes sur SkyAInet
    fn generate_skyainet_usage_response(&self, query: &str) -> String {
        let q = query.to_lowercase();

        if q.contains("ouvrir") && q.contains("wallet") {
            return "Pour ouvrir ton **Wallet** :\n→ Clique sur l’icône Wallet dans la sidebar ou tape `openWindow('wallet')`.\nTu peux y staker, unstaker, envoyer et recevoir des SKY.".to_string();
        }

        if q.contains("lancer") && (q.contains("dream") || q.contains("cycle")) {
            return "Pour lancer un **Dream Cycle** :\n→ Ouvre la fenêtre **Dream Me** (icône lune) ou clique sur 'Lancer Dream Cycle' dans le Dashboard.\nThevie analysera tes neurones et générera de nouvelles leçons créatives.".to_string();
        }

        if q.contains("gouvernance") || q.contains("proposition") {
            return "Pour accéder à la **Gouvernance** :\n→ Ouvre la fenêtre **Gouvernance**.\nTu peux créer des propositions, voter avec conviction et suivre les votes en cours.".to_string();
        }

        if q.contains("marketplace") || q.contains("louer") || q.contains("puissance") {
            return "Le **Marketplace** te permet de louer ou de louer ta puissance de calcul (GPU/CPU/TPU).\nOuvre la fenêtre **Marketplace** pour voir les offres ou publier la tienne.".to_string();
        }

        if q.contains("nœud") || q.contains("node") {
            return "Pour gérer tes **Nœuds** :\n→ Ouvre **Mes Nœuds**.\nTu peux upgrader ton tier (Mini → Light → Full → DreamWeaver → Validator), mettre en location et voir tes récompenses PoUW.".to_string();
        }

        if q.contains("thevie") || q.contains("chat") {
            return "Tu es déjà dans le chat **Thevie** !\nTu peux sélectionner **LoraÉvo** dans le sélecteur d’IA pour des réponses encore plus précises sur l’utilisation du programme.".to_string();
        }

        if q.contains("staking") || q.contains("stake") {
            return "Pour **staker** des SKY :\n→ Ouvre ton Wallet → Clique sur 'Stake' → Choisis le montant et confirme.\nTu gagnes des rewards passives (APY \~18.4%).".to_string();
        }

        if q.contains("zip") || q.contains("mémoire") {
            return "Le **ZIP Memory** permet de compresser intelligemment tes données locales.\nVa dans **Monitoring** → section ZIP Memory pour gérer la compression/décompression.".to_string();
        }

        // Réponse par défaut
        "Je suis **LoraÉvo**, le guide intelligent de SkyAInet.\nJe peux t’aider sur toutes les fonctionnalités du programme (nœuds, staking, location, Dream Cycle, gouvernance, wallet, etc.).\nQue veux-tu savoir ?".to_string()
    }

    /// Apprentissage (amélioré)
    pub fn learn_from_result(&mut self, wisdom_before: f32, wisdom_after: f32, query_type: &str) {
        let improvement = wisdom_after - wisdom_before;

        if improvement > 0.03 {
            self.success_history.push(improvement);
            self.current_profile.ethics = (self.current_profile.ethics + 0.015).min(0.96);
            debug!("[LoraÉvo] Apprentissage positif sur '{}'", query_type);
        }
    }
}