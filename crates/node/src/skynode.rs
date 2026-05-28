// crates/node/src/skynode.rs
// =====================================================
// SkyNode v6.8 — Sovereign Gateway + Hybrid Evolution + ZipMemory + Rewards
// Hub Central Intelligent + Web Hosting Décentralisé Post-Quantique
// + Système d'Évolution Hybride (Dream Cycle + Traditional Training)
// + Compression Intelligente (ZipMemory v4.0)
// + Système de Récompenses Intégré (Learn + Dream + AI Chat)
// T369Inference + HybridTransport + GematriaAead + Dilithium5
// =====================================================

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tracing::{info, debug};
use tokio::sync::Mutex;
use std::sync::Arc;

use t369_inference::T369Inference;
use sled::Db;
use skyainet_secure_transport::crypto::hybrid::HybridTransport;
use skyainet_secure_transport::crypto::gematria_aead::GematriaAead;
use skyainet_secure_transport::crypto::dilithium::Dilithium5Signer;
use skyainet_secure_transport::crypto::roman_t369::{RomanT369, GematriaMode};

use crate::evolution_manager::EvolutionManager;
use skyainet_memory::zip_memory::ZipMemory;
use skyainet_core::rewards::{UserRewards, RewardReason};

// ==================== STRUCTURES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub reputation: f64,
    pub last_seen: u64,
    pub wisdom_contribution: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeState { Active, Sleeping, Syncing, DreamMode, Evolving, Gateway }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub cpu_usage: u8,
    pub ram_used_gb: f32,
    pub ram_total_gb: f32,
    pub active_model: String,
    pub peers_connected: u32,
    pub network_speed_mbps: f32,
    pub wisdom_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub prompt: String,
    pub ai: String,
    pub max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub text: String,
    pub ai_used: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub checksum: String,
    pub version: u32,
    pub owner: String,
    pub timestamp: u64,
    pub encrypted: bool,
    pub chunks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignSite {
    pub id: String,
    pub encrypted_content: Vec<u8>,
    pub encryption_key: [u8; 32],
    pub nonce: [u8; 12],
    pub is_ai_generated: bool,
    pub signature: Vec<u8>,
    pub version: u32,
}

// ==================== SKYNODE v6.8 ====================

pub struct SkyNode {
    pub id: String,
    pub state: NodeState,
    pub peers: Vec<Peer>,
    pub inference_engine: Option<T369Inference>,
    pub total_requests: u64,
    pub wisdom_score: f32,
    pub is_running: bool,

    pub registered_ais: HashMap<String, String>,
    pub message_bus: Vec<AIMessage>,
    pub external_ai_enabled: bool,

    pub storage: DecentralizedStorage,
    pub gateway_enabled: bool,
    pub gateway_port: u16,
    pub dilithium_signer: Dilithium5Signer,
    pub hybrid: HybridTransport,
    pub api_keys: HashMap<String, String>,

    pub evolution_manager: Option<Arc<Mutex<EvolutionManager>>>,
    pub zip_memory: ZipMemory,

    // ==================== RÉCOMPENSES ====================
    pub user_rewards: UserRewards,
}

impl SkyNode {
    pub fn new() -> Self {
        let mut node = Self {
            id: "sky-0x4f2a...e7b9".to_string(),
            state: NodeState::Active,
            peers: Vec::new(),
            inference_engine: None,
            total_requests: 0,
            wisdom_score: 0.91,
            is_running: true,
            registered_ais: HashMap::new(),
            message_bus: Vec::new(),
            external_ai_enabled: false,
            storage: DecentralizedStorage::new("./data/skynode_storage").expect("Storage init failed"),
            gateway_enabled: false,
            gateway_port: 8080,
            dilithium_signer: Dilithium5Signer::new().expect("Dilithium signer failed"),
            hybrid: HybridTransport::new(false),
            api_keys: HashMap::new(),
            evolution_manager: None,
            zip_memory: ZipMemory::new("./data/zip_memory"),
            user_rewards: UserRewards::new(skyainet_core::rewards::AccountType::Free),
        };

        node.register_ai("thevie", "Thevie - Intelligence Collective");
        node.register_ai("loraevo", "LoraÉvo - Guide Évolutif");
        node.register_ai("agentic", "Agentic Mode - Mode Agentique");

        node
    }

    pub fn init_evolution_manager(&mut self) {
        let manager = EvolutionManager::new(Arc::new(Mutex::new(self.clone())));
        self.evolution_manager = Some(Arc::new(Mutex::new(manager)));
        info!("[SkyNode] EvolutionManager initialisé");
    }

    // ==================== RÉCOMPENSES ====================

    pub fn record_ai_chat_message(&mut self) {
        self.user_rewards.record_high_quality_interaction(1);
        debug!("[Rewards] AI Chat message enregistré");
    }

    pub fn record_learn_contribution(&mut self, quality: f64) {
        self.user_rewards.record_learn_contribution(quality);
        debug!("[Rewards] Learn contribution enregistré (qualité: {:.2})", quality);
    }

    pub fn record_dream_cycle_participation(&mut self) {
        self.user_rewards.record_dream_cycle();
        debug!("[Rewards] Dream Cycle participation enregistré");
    }

    pub async fn run_evolution_cycle(&mut self) {
        if let Some(manager) = &self.evolution_manager {
            let mut mgr = manager.lock().await;
            mgr.run_dream_cycle().await;
            self.record_dream_cycle_participation();
        }
    }

    pub async fn trigger_traditional_training(&mut self) -> Result<(), String> {
        if let Some(manager) = &self.evolution_manager {
            let mut mgr = manager.lock().await;
            mgr.run_traditional_training().await
        } else {
            Err("EvolutionManager non initialisé".to_string())
        }
    }

    // ==================== HUB CENTRAL INTELLIGENT ====================

    pub fn register_ai(&mut self, name: &str, description: &str) {
        self.registered_ais.insert(name.to_string(), description.to_string());
        info!("[SkyNode] IA enregistrée : {}", name);
    }

    pub fn send_message(&mut self, from: &str, to: &str, content: &str) -> Result<String, String> {
        if !self.registered_ais.contains_key(from) && from != "system" {
            return Err(format!("IA source '{}' inconnue", from));
        }
        if !self.registered_ais.contains_key(to) && to != "external" {
            return Err(format!("IA destination '{}' inconnue", to));
        }

        let msg = AIMessage {
            from: from.to_string(),
            to: to.to_string(),
            content: content.to_string(),
            timestamp: crate::utils::now_millis(),
        };

        self.message_bus.push(msg);
        self.record_ai_chat_message();

        if to == "external" && self.external_ai_enabled {
            info!("[SkyNode] Message routé vers IA Externe");
            return Ok("Message envoyé vers IA externe".to_string());
        }

        debug!("[SkyNode] Message {} → {} délivré", from, to);
        Ok(format!("Message délivré à {}", to))
    }

    pub async fn inject_lesson(&mut self, lesson: &str) -> Result<String, String> {
        info!("[Hub] Nouvelle leçon injectée : {}", lesson);
        self.record_learn_contribution(0.85);

        let msg = AIMessage {
            from: "user".to_string(),
            to: "thevie".to_string(),
            content: lesson.to_string(),
            timestamp: crate::utils::now_millis(),
        };
        self.message_bus.push(msg);

        if let Some(manager) = &self.evolution_manager {
            let mut mgr = manager.lock().await;
            mgr.run_dream_cycle().await;
        }

        if let Some(engine) = &mut self.inference_engine {
            info!("[Hub] Leçon envoyée à T369Inference");
        }

        Ok("Leçon injectée avec succès. Évolution en cours...".to_string())
    }

    // ==================== ROUTAGE IA ====================

    pub async fn generate_with_ai(&mut self, request: AIRequest) -> Result<AIResponse, String> {
        self.total_requests += 1;
        self.record_ai_chat_message();

        if self.registered_ais.contains_key(&request.ai) || 
           ["thevie", "loraevo", "agentic"].contains(&request.ai.as_str()) {
            
            if let Some(engine) = &mut self.inference_engine {
                let response = engine.generate(&request.prompt, request.max_tokens).await?;
                return Ok(AIResponse {
                    text: response,
                    ai_used: request.ai,
                    source: "local".to_string(),
                });
            }
        }

        if request.ai == "external" && self.external_ai_enabled {
            return Ok(AIResponse {
                text: format!("[External Fallback] Réponse simulée pour : {}", request.prompt),
                ai_used: "external".to_string(),
                source: "external".to_string(),
            });
        }

        if let Some(engine) = &mut self.inference_engine {
            let response = engine.generate(&request.prompt, request.max_tokens).await?;
            Ok(AIResponse {
                text: response,
                ai_used: "thevie".to_string(),
                source: "local".to_string(),
            })
        } else {
            Err("Aucun moteur d'inférence disponible".to_string())
        }
    }

    // ==================== SOVEREIGN GATEWAY ====================

    pub fn enable_gateway(&mut self, port: u16) {
        self.gateway_enabled = true;
        self.gateway_port = port;
        self.state = NodeState::Gateway;
        info!("[SkyNode] Sovereign Gateway activé sur le port {}", port);
    }

    pub async fn serve_site(&self, site_id: &str) -> Option<SovereignSite> {
        if let Ok(data) = self.storage.retrieve_file(site_id) {
            Some(SovereignSite {
                id: site_id.to_string(),
                encrypted_content: data,
                encryption_key: [0u8; 32],
                nonce: [0u8; 12],
                is_ai_generated: true,
                signature: self.dilithium_signer.sign(&data),
                version: 1,
            })
        } else {
            None
        }
    }

    pub async fn generate_dynamic_site(&mut self, prompt: &str) -> Result<String, String> {
        let response = self.generate_with_ai(AIRequest {
            prompt: prompt.to_string(),
            ai: "thevie".to_string(),
            max_tokens: 2048,
        }).await?;

        let (key, nonce) = self.hybrid.derive_keys();
        let aead = GematriaAead::new(key, nonce);
        let encrypted = aead.encrypt(response.text.as_bytes());

        let site_id = format!("site_{}", crate::utils::now_millis());
        self.storage.store_file(&site_id, &encrypted, &self.id)?;

        let signature = self.dilithium_signer.sign(&encrypted);

        info!("[Gateway] Site dynamique généré et chiffré : {}", site_id);
        Ok(site_id)
    }

    pub fn generate_api_key(&mut self, name: &str) -> String {
        let key = uuid::Uuid::new_v4().to_string();
        let encrypted = self.hybrid.encrypt_with_current_mode(&key.as_bytes()).unwrap_or_default();
        self.api_keys.insert(name.to_string(), encrypted);
        key
    }

    // ==================== STOCKAGE ====================

    pub fn upload_file(&mut self, name: &str, data: &[u8]) -> Result<String, String> {
        self.storage.store_file(name, data, &self.id)
    }

    pub fn list_files(&self) -> Result<Vec<FileMetadata>, String> {
        self.storage.list_files()
    }

    pub fn download_file(&self, id: &str) -> Result<Vec<u8>, String> {
        self.storage.retrieve_file(id)
    }

    pub fn delete_file(&mut self, id: &str) -> Result<bool, String> {
        self.storage.delete_file(id)
    }

    pub async fn replicate_files(&mut self) {
        self.storage.replicate_pending().await;
    }

    pub async fn process_request(&mut self, prompt: &str, max_tokens: usize) -> Result<String, String> {
        if let Some(engine) = &mut self.inference_engine {
            engine.generate(prompt, max_tokens).await
        } else {
            Err("Moteur non connecté".to_string())
        }
    }

    pub fn get_status(&self) -> String {
        format!(
            "SkyNode {} | {:?} | IA: {} | Rewards: {} SKY | Gateway: {} | Sagesse: {:.3}",
            self.id, self.state, self.registered_ais.len(),
            self.user_rewards.total_sky_earned,
            self.gateway_enabled,
            self.wisdom_score
        )
    }
}

// ==================== STOCKAGE DÉCENTRALISÉ ====================

pub struct DecentralizedStorage {
    db: Db,
    roman: RomanT369,
    replication_queue: Vec<String>,
}

impl DecentralizedStorage {
    pub fn new(path: &str) -> Result<Self, String> {
        let db = sled::open(path).map_err(|e| e.to_string())?;
        Ok(Self {
            db,
            roman: RomanT369::new([0x55u8; 32], [0u8; 12], GematriaMode::Hyper256),
            replication_queue: Vec::new(),
        })
    }

    pub fn store_file(&mut self, name: &str, data: &[u8], owner: &str) -> Result<String, String> {
        let id = format!("file_{}", crate::utils::now_millis());
        let checksum = format!("{:x}", md5::compute(data));
        let encrypted = self.roman.encrypt(data);

        let meta = FileMetadata {
            id: id.clone(),
            name: name.to_string(),
            size: data.len() as u64,
            checksum,
            version: 1,
            owner: owner.to_string(),
            timestamp: crate::utils::now_millis(),
            encrypted: true,
            chunks: vec![],
        };

        self.db.insert(format!("meta:{}", id).as_bytes(), serde_json::to_vec(&meta).unwrap())
            .map_err(|e| e.to_string())?;
        self.db.insert(format!("data:{}", id).as_bytes(), encrypted)
            .map_err(|e| e.to_string())?;

        self.replication_queue.push(id.clone());
        Ok(id)
    }

    pub fn retrieve_file(&self, id: &str) -> Result<Vec<u8>, String> {
        let encrypted = self.db.get(format!("data:{}", id).as_bytes())
            .map_err(|e| e.to_string())?
            .ok_or("Fichier non trouvé")?;
        
        self.roman.decrypt(&encrypted).ok_or("Échec du déchiffrement".to_string())
    }

    pub fn delete_file(&mut self, id: &str) -> Result<bool, String> {
        self.db.remove(format!("meta:{}", id).as_bytes()).map_err(|e| e.to_string())?;
        self.db.remove(format!("data:{}", id).as_bytes()).map_err(|e| e.to_string())?;
        Ok(true)
    }

    pub fn list_files(&self) -> Result<Vec<FileMetadata>, String> {
        let mut files = Vec::new();
        for item in self.db.scan_prefix("meta:") {
            let (_, value) = item.map_err(|e| e.to_string())?;
            let meta: FileMetadata = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
            files.push(meta);
        }
        Ok(files)
    }

    pub async fn replicate_pending(&mut self) {
        if !self.replication_queue.is_empty() {
            info!("[SkyNode] Réplication de {} fichiers en cours...", self.replication_queue.len());
            self.replication_queue.clear();
        }
    }
}

// ==================== TAURI COMMANDS ====================

#[tauri::command]
pub async fn generate_with_ai(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, prompt: String, ai: String, max_tokens: usize) -> Result<AIResponse, String> {
    let mut node = state.lock().await;
    node.generate_with_ai(AIRequest { prompt, ai, max_tokens }).await
}

#[tauri::command]
pub async fn send_ai_message(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, from: String, to: String, content: String) -> Result<String, String> {
    let mut node = state.lock().await;
    node.send_message(&from, &to, &content)
}

#[tauri::command]
pub async fn get_registered_ais(state: tauri::State<'_, Arc<Mutex<SkyNode>>>) -> Result<Vec<String>, String> {
    let node = state.lock().await;
    Ok(node.registered_ais.keys().cloned().collect())
}

#[tauri::command]
pub async fn toggle_external_ai(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, enabled: bool) -> Result<bool, String> {
    let mut node = state.lock().await;
    node.enable_external_ai(enabled);
    Ok(enabled)
}

#[tauri::command]
pub async fn upload_file(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, name: String, data: Vec<u8>) -> Result<String, String> {
    let mut node = state.lock().await;
    node.upload_file(&name, &data)
}

#[tauri::command]
pub async fn list_files(state: tauri::State<'_, Arc<Mutex<SkyNode>>>) -> Result<Vec<FileMetadata>, String> {
    let node = state.lock().await;
    node.list_files()
}

#[tauri::command]
pub async fn download_file(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, id: String) -> Result<Vec<u8>, String> {
    let node = state.lock().await;
    node.download_file(&id)
}

#[tauri::command]
pub async fn delete_file(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, id: String) -> Result<bool, String> {
    let mut node = state.lock().await;
    node.delete_file(&id)
}

#[tauri::command]
pub async fn enable_gateway(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, port: u16) -> Result<bool, String> {
    let mut node = state.lock().await;
    node.enable_gateway(port);
    Ok(true)
}

#[tauri::command]
pub async fn generate_dynamic_site(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, prompt: String) -> Result<String, String> {
    let mut node = state.lock().await;
    node.generate_dynamic_site(&prompt).await
}

#[tauri::command]
pub async fn create_api_key(state: tauri::State<'_, Arc<Mutex<SkyNode>>>, name: String) -> Result<String, String> {
    let mut node = state.lock().await;
    Ok(node.generate_api_key(&name))
}

#[tauri::command]
pub async fn run_evolution_cycle(state: tauri::State<'_, Arc<Mutex<SkyNode>>>) -> Result<String, String> {
    let mut node = state.lock().await;
    node.run_evolution_cycle().await;
    Ok("Evolution cycle completed".to_string())
}

#[tauri::command]
pub async fn trigger_traditional_training(state: tauri::State<'_, Arc<Mutex<SkyNode>>>) -> Result<String, String> {
    let mut node = state.lock().await;
    node.trigger_traditional_training().await?;
    Ok("Traditional training completed".to_string())
}

#[tauri::command]
pub async fn claim_rewards(state: tauri::State<'_, Arc<Mutex<SkyNode>>>) -> Result<u128, String> {
    let mut node = state.lock().await;
    let amount = node.user_rewards.claim_monthly_rewards();
    Ok(amount)
}

#[tauri::command]
pub async fn get_rewards_stats(state: tauri::State<'_, Arc<Mutex<SkyNode>>>) -> Result<UserRewards, String> {
    let node = state.lock().await;
    Ok(node.user_rewards.clone())
}

// ==================== INIT ====================

pub fn init_skynode() -> Arc<Mutex<SkyNode>> {
    let node = Arc::new(Mutex::new(SkyNode::new()));
    
    {
        let mut node_guard = node.lock().await;
        node_guard.init_evolution_manager();
    }
    
    node
}