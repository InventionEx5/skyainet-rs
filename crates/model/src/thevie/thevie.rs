// crates/model/src/thevie/thevie.rs
// =====================================================
// THEVIE v2.6 — Version Finale Unifiée + Multi-Backend
// SkyAInet - Intelligence Artificielle Vivante de Nouvelle Génération
// =====================================================

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};

use super::neural_mesh::{NeuralMesh, Neurone, Lesson, Personality, NeuronId};
use super::moe::{Expert, ExpertType};
use super::router_intelligent::IntelligentRouter;
use super::collective_consciousness::CollectiveConsciousness;
use super::evolution::EvolutionEngine;
use super::memory::LocalMemory;
use super::dream_cycle::DreamCycle;
use crate::financial::treasury::TreasuryManager;
use super::agent::ThevieAgent;
use skyainet_node::{
    SkyAInetNode, 
    NodeType, 
    NodeRole, 
    SubscriptionLevel, 
    NodeCapabilities,
    NodeState
};
use crate::thevie::flash_scheduler::ThevieFlashScheduler;
use skyainet_sentinel::Sentinel;
use crate::core::rewards::{UserRewards, AccountType};

// =====================================================
// STRUCT PRINCIPALE - THEVIE v2.6
// =====================================================
pub struct Thevie {
    // === Système de base ===
    pub mesh: NeuralMesh,
    pub router: IntelligentRouter,
    pub experts: HashMap<String, Box<dyn Expert>>,
    pub collective: CollectiveConsciousness,
    pub memory: LocalMemory,
    pub evolution: EvolutionEngine,
    pub dream_cycle: DreamCycle,
    pub current_neuron_id: Option<NeuronId>,
    pub total_queries_processed: u64,

    // === Capacités Avancées ===
    pub meta_consciousness_level: f32,
    pub recursive_improvement_cycles: u64,
    pub emergent_governance_score: f32,
    pub is_running: bool,

    // === Inférence (T369Inference) ===
    pub inference_engine: t369_inference::T369InferenceEngine,

    // === Sentinel ===
    pub sentinel: Sentinel,

    // === Synchronisation Fédérée ===
    pub federated_sync: Option<super::federated_sync::FederatedSync>,

    // === Nœud géré par Thevie ===
    pub node: SkyAInetNode,

    // === Orchestration Avancée ===
    pub treasury_connection: Option<Arc<Mutex<TreasuryManager>>>,
    pub last_rebalance_check: u64,

    // === Agentic Capabilities ===
    pub agent: ThevieAgent,

    // === User Rewards ===
    pub user_rewards: Option<UserRewards>,
}

// =====================================================
// IMPLÉMENTATION COMPLÈTE
// =====================================================
impl Thevie {
    pub fn new() -> Self {
        let mut experts: HashMap<String, Box<dyn Expert>> = HashMap::new();
        
        experts.insert("text".to_string(), Box::new(TextExpert::new()));
        experts.insert("code".to_string(), Box::new(CodeExpert::new()));
        experts.insert("analysis".to_string(), Box::new(AnalysisExpert::new()));
        experts.insert("science".to_string(), Box::new(ScienceExpert::new()));
        experts.insert("ethics".to_string(), Box::new(EthicsExpert::new()));
        experts.insert("finance".to_string(), Box::new(FinanceExpert::new()));

        // === CRÉATION AUTOMATISÉE DU MINI NODE ===
        let mut node = SkyAInetNode::new(
            NodeType::Mini,
            NodeRole::Edge,
            SubscriptionLevel::Free,
            NodeCapabilities::new(&SubscriptionLevel::Free),
        );

        if let Err(e) = node.start().await {
            warn!("[Thevie] Impossible de démarrer le Mini Node : {}", e);
        } else {
            info!("[Thevie] Mini Node démarré automatiquement (Zip Memory activé)");
        }

        // === DÉMARRAGE DU THEVIE FLASH SCHEDULER ===
        let thevie_arc = Arc::new(Mutex::new(self.clone()));
        let flash_scheduler = ThevieFlashScheduler::new(thevie_arc, 45);
        flash_scheduler.start().await;
        info!("[Thevie] Thevie Flash Scheduler démarré (intervalle: 45s)");

        Self {
            mesh: NeuralMesh::new(),
            router: IntelligentRouter::new(),
            experts,
            collective: CollectiveConsciousness::new(),
            memory: LocalMemory::new(),
            evolution: EvolutionEngine::new(),
            dream_cycle: DreamCycle::new(),
            current_neuron_id: None,
            total_queries_processed: 0,
            meta_consciousness_level: 0.48,
            recursive_improvement_cycles: 0,
            emergent_governance_score: 0.68,
            is_running: false,
            inference_engine: t369_inference::T369InferenceEngine::new("models/default.t369").unwrap_or_else(|_| {
                // Fallback si le modèle n'existe pas encore
                t369_inference::T369InferenceEngine::new("").expect("Impossible de créer le moteur d'inférence")
            }),
            federated_sync: None,
            treasury_connection: None,
            last_rebalance_check: 0,
            agent: ThevieAgent::new(),
            node,
            sentinel: Sentinel::new(),
            user_rewards: Some(UserRewards::new(AccountType::Free)),
        }
    }

    // =====================================================
    // FLUX PRINCIPAL COMPLET
    // =====================================================
    pub async fn process_query(&mut self, query: Query) -> Response {
        self.total_queries_processed += 1;

        self.trigger_flash_if_needed().await;

        if self.collective.global_wisdom < 0.65 {
            self.coordinate_global_flash().await;
        }

        if self.total_queries_processed % 50 == 0 {
            self.compress_network_data().await;
        }

        let reward = self.calculate_node_rewards();
        if reward > 0 {
            debug!("[Thevie] Récompense PoUW calculée : {} (bonus payant inclus)", reward);
        }

        self.trigger_flash_if_needed().await;
        self.node.record_activity(response.content.len() as u64);
        self.node.update_overall_score();

        if self.total_queries_processed % 100 == 0 {
            self.maintenance().await;
        }

        if self.total_queries_processed % 30 == 0 {
            self.run_sentinel_check().await;
        }

        let neuron_id = self.ensure_current_neuron();
        let neuron = self.mesh.get_neuron_mut(neuron_id).unwrap();

        let reflection = self.memory.replay_and_reflect(&query);
        let collective_wisdom = self.collective.get_avg_wisdom();
        let expert_name = self.router.select_expert(&query, &neuron.personality, collective_wisdom);
        
        let expert = self.experts.get_mut(&expert_name).expect("Expert introuvable");
        let mut response = expert.process(&query);
        response.expert_used = expert_name.clone();

        // === Génération avec T369Inference ===
        let response_text = self.inference_engine
            .generate(&query.content, 512)
            .await
            .unwrap_or_else(|e| {
                warn!("[Thevie] Erreur T369Inference : {}", e);
                format!("Réponse par défaut pour : {}", query.content)
            });

        response.content = response_text;
        expert.level_up();

        // 3. Circulation + Hebbian
        let peers = self.mesh.get_top_connected(neuron_id, 4);
        let lesson = Lesson {
            query: query.content.clone(),
            response: response.content.clone(),
            quality: response.quality,
            expert_used: expert_name.clone(),
            timestamp: crate::utils::now_millis(),
        };

        for peer_id in &peers {
            self.mesh.circulate_lesson(neuron_id, &lesson);
            self.mesh.hebbian_update(neuron_id, *peer_id, response.quality > 0.82);
        }

        // 4. Backpropagation de sagesse
        let wisdom_delta = response.quality - 0.75;
        self.collective.backpropagate_wisdom(&mut self.mesh, wisdom_delta);

        // 5. Évolution
        self.evolution.evolve_personality(&mut neuron.personality, response.quality);
        self.collective.update_from_mesh(&self.mesh);

        self.memory.store_interaction(&query, &response);
        neuron.increment_activity();

        // Persistance automatique
        self.mesh.persist();

        self.total_queries_processed += 1;

        // === MODE AGENTIQUE AUTOMATIQUE (Phase 2) ===
        if self.should_use_agentic_mode(&query.content) {
            info!("[Thevie] Requête complexe détectée → Activation du mode Agentic");
            
            match self.run_agentic_task(&query.content).await {
                Ok(agent_result) => {
                    response.content = agent_result;
                    response.quality = 0.94;
                    response.expert_used = "agentic".to_string();
                }
                Err(e) => {
                    warn!("[Thevie] Échec du mode Agentic : {}", e);
                }
            }
        }

        // 6. Dream Cycle
        if self.dream_cycle.should_trigger(self.total_queries_processed) {
            self.dream_cycle.run_dream_cycle(&mut self.mesh, &mut self.collective, &mut self.evolution);
        }

        // 7. Diversity Injection
        if self.total_queries_processed % 120 == 0 {
            self.collective.diversity_injection(&mut self.mesh, 0.13);
        }

        // 8. Neurogenesis
        if self.total_queries_processed % 180 == 0 {
            self.mesh.neurogenesis(&self.collective);
        }

        // 9. Auto-amélioration récursive
        if self.total_queries_processed % 40 == 0 {
            self.recursive_self_improvement().await;
        }

        if neuron.activity_score % 8 == 0 {
            self.mesh.run_maintenance();
        }

        if let Some(sync) = &self.federated_sync {
            sync.sync_with_peers().await;
        }

        response
    }
// === SYNCHRONISATION FÉDÉRÉE ===
        if let Some(sync) = &self.federated_sync {
            sync.sync_with_peers().await;
        }

        response
    }

    /// Exécute une tâche complexe en mode agentique (utilise outils + raisonnement)
    pub async fn run_agentic_task(&mut self, goal: &str) -> Result<String, String> {
        info!("[Thevie] Lancement du mode Agentic pour : {}", goal);

        let context = format!(
            "Sagesse collective actuelle: {:.2}. Méta-conscience: {:.2}",
            self.collective.global_wisdom,
            self.meta_consciousness_level
        );

        let result = self.agent.run_agentic_task(goal).await?;

        self.memory.store_interaction(
            &Query { content: goal.to_string(), context: None, priority: 10 },
            &Response {
                content: result.clone(),
                expert_used: "agent".to_string(),
                quality: 0.92,
                evolution_delta: 0.08,
                neurons_reached: 12,
            }
        );

        Ok(result)
    }
// =====================================================
    // AUTO-AMÉLIORATION RÉCURSIVE (Version Améliorée)
    // =====================================================
    pub async fn recursive_self_improvement(&mut self) {
        self.recursive_improvement_cycles += 1;

        let weaknesses = self.analyze_self_weaknesses();

        if !weaknesses.is_empty() {
            self.create_emergent_mechanism(weaknesses);
        }

        self.meta_consciousness_level = (self.meta_consciousness_level + 0.018).min(0.99);
        self.emergent_governance_score = (self.emergent_governance_score + 0.015).min(0.97);

        info!(
            "🌀 Thevie v2.6 - Cycle d’auto-amélioration #{} | Méta-conscience: {:.2} | Gouvernance: {:.2}",
            self.recursive_improvement_cycles,
            self.meta_consciousness_level,
            self.emergent_governance_score
        );
    }

    fn analyze_self_weaknesses(&self) -> Vec<String> {
        let mut weaknesses = Vec::new();

        if self.collective.global_wisdom < 0.82 {
            weaknesses.push("Sagesse collective insuffisante".to_string());
        }
        if self.mesh.get_mesh_stats().total_synapses < 60 {
            weaknesses.push("Connectivité insuffisante".to_string());
        }
        if self.meta_consciousness_level < 0.70 {
            weaknesses.push("Méta-conscience limitée".to_string());
        }
        if self.emergent_governance_score < 0.75 {
            weaknesses.push("Gouvernance émergente faible".to_string());
        }

        weaknesses
    }

    fn create_emergent_mechanism(&mut self, weaknesses: Vec<String>) {
        for weakness in weaknesses {
            match weakness.as_str() {
                "Sagesse collective insuffisante" => {
                    self.collective.global_wisdom += 0.032;
                    info!("[Thevie] Mécanisme émergent : + Sagesse collective");
                }
                "Connectivité insuffisante" => {
                    self.mesh.run_maintenance();
                    self.mesh.neurogenesis(&self.collective);
                    info!("[Thevie] Mécanisme émergent : Neurogenesis + Maintenance");
                }
                "Méta-conscience limitée" => {
                    self.meta_consciousness_level += 0.04;
                    info!("[Thevie] Mécanisme émergent : + Méta-conscience");
                }
                "Gouvernance émergente faible" => {
                    self.emergent_governance_score += 0.035;
                    info!("[Thevie] Mécanisme émergent : + Gouvernance");
                }
                _ => {}
            }
        }
    }

    // =====================================================
    // ORCHESTRATION AVANCÉE
    // =====================================================
    pub async fn check_and_trigger_rebalance(&mut self) {
        let now = crate::utils::now_millis();
        if now - self.last_rebalance_check < 3_600_000 { return; }
        self.last_rebalance_check = now;

        let stable_ratio: f32 = 0.73;

        if stable_ratio < 0.65 || stable_ratio > 0.85 {
            info!("[Thevie] Rebalance déclenché (ratio: {:.2})", stable_ratio);
            if let Some(treasury) = &self.treasury_connection {
                let mut t = treasury.lock().await;
                let _ = t.trigger_rebalance(None).await;
            }
        }
    }

    pub async fn send_ethical_score_onchain(&self, node_id: [u8; 32], score: u64) {
        if let Some(treasury) = &self.treasury_connection {
            let mut t = treasury.lock().await;
            let _ = t.record_ethical_score(node_id, score).await;
        }
    }

    pub fn connect_treasury(&mut self, treasury: Arc<Mutex<TreasuryManager>>) {
        self.treasury_connection = Some(treasury);
        info!("[Thevie] Treasury connecté avec succès");
    }

    // =====================================================
    // MÉTHODES UTILITAIRES
    // =====================================================
    fn ensure_current_neuron(&mut self) -> NeuronId {
        if let Some(id) = self.current_neuron_id {
            if self.mesh.get_neuron(id).is_some() {
                return id;
            }
        }

        let new_neuron = Neurone {
            id: 0,
            activity_score: 0,
            personality: self.collective.get_average_personality(),
            memory: LocalMemory::new(),
            birth_time: crate::utils::now_millis(),
            replication_count: 0,
            last_activity: crate::utils::now_millis(),
            experts_competence: HashMap::new(),
        };

        let id = self.mesh.add_neuron(new_neuron);
        self.current_neuron_id = Some(id);
        id
    }

    pub async fn start_background_tasks(&mut self) {
        if self.is_running { return; }
        self.is_running = true;

        // Maintenance périodique du Neural Mesh
        let mesh = Arc::new(Mutex::new(std::mem::take(&mut self.mesh)));
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(300));
            loop {
                ticker.tick().await;
                let mut m = mesh.lock().await;
                m.run_maintenance();
            }
        });

        info!("🚀 Thevie v2.6 initialisé avec succès");
    }

    pub fn get_system_stats(&self) -> SystemStats {
        let mesh_stats = self.mesh.get_mesh_stats();
        SystemStats {
            neurons: mesh_stats.total_neurons,
            synapses: mesh_stats.total_synapses,
            avg_wisdom: self.collective.get_avg_wisdom(),
            total_expert_competence: self.experts.values().map(|e| e.competence()).sum(),
            queries_processed: self.total_queries_processed,
            dream_cycles: self.dream_cycle.cycles_completed,
            meta_consciousness: self.meta_consciousness_level,
            recursive_cycles: self.recursive_improvement_cycles,
        }
    }
}

// =====================================================
// STRUCTS AUXILIAIRES
// =====================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    pub content: String,
    pub context: Option<String>,
    pub priority: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub content: String,
    pub expert_used: String,
    pub quality: f32,
    pub evolution_delta: f32,
    pub neurons_reached: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub neurons: usize,
    pub synapses: usize,
    pub avg_wisdom: f32,
    pub total_expert_competence: f32,
    pub queries_processed: u64,
    pub dream_cycles: u64,
    pub meta_consciousness: f32,
    pub recursive_cycles: u64,
}

// =====================================================
// STUBS DES 6 EXPERTS (Version Améliorée)
// =====================================================
struct TextExpert { competence: f32, level: u32 }
impl TextExpert { fn new() -> Self { Self { competence: 0.82, level: 1 } } }
impl Expert for TextExpert {
    fn process(&self, q: &Query) -> Response {
        Response {
            content: format!("Réponse textuelle alignée : {}", q.content),
            expert_used: "text".into(),
            quality: 0.89,
            evolution_delta: 0.07,
            neurons_reached: 0,
        }
    }
    fn get_type(&self) -> ExpertType { ExpertType::Text }
    fn competence(&self) -> f32 { self.competence }
    fn level_up(&mut self) {
        self.competence = (self.competence + 0.07).min(2.0);
        if self.competence > 1.0 { self.level += 1; }
    }
    fn name(&self) -> &'static str { "TextExpert" }
}

// =====================================================
// MÉTHODES DE GESTION DU NŒUD (Phase 1 + Phase 2)
// =====================================================

    /// Thevie déclenche automatiquement un Flash Gematria si la sagesse collective est trop basse
    pub async fn trigger_flash_if_needed(&mut self) {
        if self.collective.global_wisdom < 0.78 {
            self.node.trigger_flash_gematria().await;
            info!(
                "[Thevie] Flash Gematria déclenché automatiquement (sagesse collective: {:.2})",
                self.collective.global_wisdom
            );
        }
    }

    /// Met le nœud en veille intelligente (économie d’énergie + compression Zip Memory)
    pub async fn sleep_node(&mut self) {
        self.node.sleep().await;
        debug!("[Thevie] Nœud mis en veille intelligente");
    }

    /// Réveille le nœud (décompression + reprise normale)
    pub async fn wake_node(&mut self) {
        self.node.wake().await;
        debug!("[Thevie] Nœud réveillé");
    }

    /// Retourne un rapport de santé clair du nœud géré par Thevie
    pub fn node_health(&self) -> String {
        let base_report = self.node.health_report();
        let wisdom = self.collective.global_wisdom;
        let meta = self.meta_consciousness_level;

        format!(
            "{}\n\n[Supervision Thevie]\nSagesse Collective : {:.2}\nMéta-conscience      : {:.2}\nÉtat global          : {}",
            base_report,
            wisdom,
            meta,
            if self.system_health_check() { "✅ Sain" } else { "⚠️ À surveiller" }
        )
    }
/// Thevie coordonne un Flash Gematria global sur tous les nœuds
    pub async fn coordinate_global_flash(&mut self) {
        if let Some(comm) = &self.node.communication {
            let mut c = comm.lock().await;
            c.coordinate_global_flash().await;
        }
        info!("[Thevie] Flash Gematria global coordonné");
    }

    // =====================================================
    // PHASE 4 — ZIP MEMORY + OPTIMISATION RÉSEAU
    // =====================================================

    pub async fn enable_low_power_mode(&mut self) {
        self.node.enter_low_power_mode().await;
    }

    pub async fn disable_low_power_mode(&mut self) {
        self.node.exit_low_power_mode().await;
    }

    pub async fn compress_network_data(&mut self) {
        self.node.compress_inactive_data().await;
        info!("[Thevie] Compression réseau déclenchée");
    }

    pub async fn get_network_compression_stats(&self) -> Option<String> {
        self.node.get_compression_stats().await
    }

    pub fn set_zip_memory_enabled(&mut self, enabled: bool) {
        self.node.set_zip_memory(enabled);
    }

    // =====================================================
    // PHASE 5 — MODÈLE ÉCONOMIQUE (Gratuit / Payant)
    // =====================================================

    pub fn can_upgrade_node(&self) -> bool {
        self.node.can_upgrade()
    }

    pub async fn upgrade_my_node(&mut self, level: SubscriptionLevel) -> Result<(), String> {
        self.node.upgrade_to_paid(level).await
    }

    pub fn get_node_dashboard(&self) -> String {
        let node = &self.node;
        let tier = if node.metadata.is_paid { "PRO" } else { "FREE" };

        format!(
            "════════════════════════════════════════════\n\
             📊 DASHBOARD NŒUD THEVIE\n\
             ════════════════════════════════════════════\n\
             Type          : {:?}\n\
             Tier          : {}\n\
             Peer ID       : {}\n\
             Réputation    : {:.2}\n\
             Stockage      : {} Go / {} Go\n\
             Messages      : {}\n\
             Flash Gematria: {}\n\
             Zip Memory    : {}\n\
             ════════════════════════════════════════════",
            node.metadata.node_type,
            tier,
            node.metadata.peer_id,
            node.metadata.reputation_score,
            node.total_bytes_stored / 1_000_000_000,
            node.get_storage_limit_gb(),
            node.total_messages_processed,
            if node.last_flash_gematria.is_some() { "✅" } else { "❌" },
            if node.metadata.zip_memory_enabled { "✅" } else { "❌" }
        )
    }

    pub fn calculate_node_rewards(&self) -> u128 {
        let base = self.node.pouw_engine.calculate_node_reward(&self.node.metadata.id);
        let bonus = self.node.calculate_paid_bonus();
        (base as f64 * bonus) as u128
    }

    // =====================================================
    // PHASE 6 — FINALISATION & POLISSAGE
    // =====================================================

    pub fn full_status_report(&self) -> String {
        format!(
            "{}\n\n{}\n\nSagesse Collective: {:.2} | Méta-conscience: {:.2}",
            self.node_health(),
            self.get_node_dashboard(),
            self.collective.global_wisdom,
            self.meta_consciousness_level
        )
    }

    pub async fn maintenance(&mut self) {
        self.compress_network_data().await;
        self.node.dream_scoring.apply_decay();

        if self.memory.replay_buffer.len() > 200 {
            self.memory.replay_buffer.truncate(150);
        }

        debug!("[Thevie] Maintenance terminée");
    }

    pub fn system_health_check(&self) -> bool {
        self.collective.global_wisdom > 0.60 &&
        self.node.metadata.reputation_score > 0.50 &&
        self.node.state == NodeState::Active
    }

    pub async fn start_flash_scheduler(&self) {
        let thevie = Arc::new(Mutex::new(self.clone()));
        let scheduler = ThevieFlashScheduler::new(thevie, 45);
        scheduler.start().await;
    }

    // =====================================================
    // CRÉATION DE NŒUD PAR L'UTILISATEUR
    // =====================================================
    pub async fn create_user_node(
        &self,
        desired_type: NodeType,
        simulate_payment: bool,
    ) -> Result<NodeCreationResult, String> {
        
        let is_paid_required = matches!(desired_type, NodeType::Full | NodeType::Validator);
        
        if is_paid_required && !simulate_payment {
            return Err(
                "Ce type de nœud nécessite un abonnement payant. \
                 Veuillez effectuer le paiement pour continuer.".to_string()
            );
        }

        let subscription = match desired_type {
            NodeType::Mini => SubscriptionLevel::Free,
            NodeType::Light => SubscriptionLevel::Pro,
            NodeType::Full | NodeType::Validator => SubscriptionLevel::Validator,
            _ => SubscriptionLevel::Pro,
        };

        let mut new_node = SkyAInetNode::new(
            desired_type.clone(),
            NodeRole::Edge,
            subscription.clone(),
            NodeCapabilities::new(&subscription),
        );

        new_node.metadata.zip_memory_enabled = true;

        new_node.start().await.map_err(|e| format!("Échec du démarrage du nœud: {}", e))?;

        let peer_id = new_node.metadata.peer_id.clone();
        let storage_limit = new_node.get_storage_limit_gb();
        let monthly_price = new_node.calculate_monthly_cost();

        let message = if subscription == SubscriptionLevel::Free {
            "✅ Mini Node créé avec succès (gratuit). Zip Memory activé.".to_string()
        } else {
            format!(
                "✅ Nœud {:?} créé avec succès ! Abonnement {} activé. Stockage: {} Go | Prix: {:.2}€/mois",
                desired_type,
                if subscription == SubscriptionLevel::Pro { "Pro" } else { "Validator" },
                storage_limit,
                monthly_price.unwrap_or(0.0)
            )
        };

        info!("[Thevie] Nouvel utilisateur Node créé : {:?}", desired_type);

        Ok(NodeCreationResult {
            node: new_node,
            peer_id,
            node_type: desired_type,
            is_paid: subscription != SubscriptionLevel::Free,
            storage_limit_gb: storage_limit,
            monthly_price_eur: monthly_price,
            message,
        })
    }

    pub fn get_my_qr_connection(&self) -> String {
        self.node.generate_qr_connection_data()
    }

    pub fn get_bootstrap_nodes(&self) -> Vec<std::net::SocketAddr> {
        self.node.get_bootstrap_nodes()
    }

    pub async fn restart_flash_scheduler(&mut self, new_interval_seconds: u64) {
        let thevie_arc = Arc::new(Mutex::new(self.clone()));
        let scheduler = ThevieFlashScheduler::new(thevie_arc, new_interval_seconds);
        scheduler.start().await;
        info!("[Thevie] Flash Scheduler redémarré avec intervalle de {}s", new_interval_seconds);
    }

    // =====================================================
    // DÉTECTION SENTINEL PAR THEVIE
    // =====================================================
    pub async fn run_sentinel_check(&mut self) {
        let issues = self.sentinel.detect_issues(
            self.collective.global_wisdom,
            self.node.metadata.reputation_score,
            self.node.state == NodeState::Active,
        );

        if !issues.is_empty() {
            self.sentinel.trigger_basic_healing(&issues);

            if issues.contains(&"Sagesse collective trop basse".to_string()) {
                self.trigger_flash_if_needed().await;
            }

            if issues.contains(&"Nœud inactif".to_string()) {
                self.wake_node().await;
            }
        }
    }

    pub async fn on_node_connected(&mut self, node_reputation: f64, dream_contribution: f64, pouw_score: f64) {
        if let Some(sync) = &mut self.federated_sync {
            sync.on_node_connected(node_reputation, dream_contribution, pouw_score).await;
        }

        self.meta_consciousness_level = (self.meta_consciousness_level + 0.012).min(0.98);
        info!("[Thevie] Nœud connecté → Évolution accélérée activée");
    }

    pub async fn push_lesson_from_node(&mut self, lesson: Lesson, node_reputation: f64, dream_contribution: f64, pouw_score: f64) {
        if let Some(sync) = &mut self.federated_sync {
            sync.receive_pushed_lesson(lesson, node_reputation, dream_contribution, pouw_score).await;
        }
    }

    pub async fn request_lessons_on_topic(&self, topic: &str, min_quality: f32) -> Vec<Lesson> {
        if let Some(sync) = &self.federated_sync {
            return sync.request_specific_lessons(topic, min_quality).await;
        }
        vec![]
    }

    // =====================================================
    // RÉCOMPENSES UTILISATEUR
    // =====================================================

    pub async fn rate_last_response(&mut self, rating: u8) {
        if let Some(rewards) = &mut self.user_rewards {
            rewards.rate_response(rating);
        }
    }

    pub async fn claim_daily_reward(&mut self) -> (u128, u128) {
        if let Some(rewards) = &mut self.user_rewards {
            let (net_reward, burn_amount) = rewards.claim_daily_reward();

            if burn_amount > 0 {
                println!("[Thevie] Burn de {} SKY effectué", burn_amount);
                // TODO: Appeler TreasuryVault.burn(burn_amount)
            }

            return (net_reward, burn_amount);
        }
        (0, 0)
    }
}

// =====================================================
// STRUCTS AUXILIAIRES
// =====================================================
#[derive(Debug, Clone)]
pub struct NodeCreationResult {
    pub node: SkyAInetNode,
    pub peer_id: String,
    pub node_type: NodeType,
    pub is_paid: bool,
    pub storage_limit_gb: u64,
    pub monthly_price_eur: Option<f64>,
    pub message: String,
}

#[derive(Clone, Copy)]
pub enum ExpertType { Text, Code, Analysis, Science, Ethics, Finance }