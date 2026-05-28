// crates/node/src/skyainet_node.rs
// =====================================================
// SkyAInetNode v4.0 — Nœud Souverain Intelligent
// Core optimisé + ZipMemory + Hybrid Crypto + Évolution + Rewards
// =====================================================

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, warn, error};

use crate::pouw::PoUWEngine;
use crate::dream_scoring::DreamScoring;
use crate::zip_memory::ZipMemory;
use crate::node_types::{NodeType, NodeState, NodeRole, NodeCapabilities, SubscriptionLevel};
use skyainet_secure_transport::crypto::hybrid::HybridTransport;
use crate::rewards::UserRewards;
use crate::evolution_manager::EvolutionManager;

// =====================================================
// MÉTADONNÉES & ÉCONOMIE
// =====================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub id: [u8; 32],
    pub node_type: NodeType,
    pub node_role: NodeRole,
    pub subscription_level: SubscriptionLevel,
    pub peer_id: String,
    pub capabilities: NodeCapabilities,
    pub reputation_score: f64,
    pub last_active: DateTime<Utc>,
    pub dream_contributions: u64,
    pub total_pouw_score: f64,
    pub zip_memory_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub is_paid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalScore {
    pub benevolence: f64,
    pub truthfulness: f64,
    pub non_malice: f64,
    pub sovereignty: f64,
    pub overall: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEconomics {
    pub tier: NodeTier,
    pub is_renting_out: bool,
    pub monthly_earnings: u128,
    pub last_rent_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeTier {
    Mini = 0,
    Light = 1,
    Full = 2,
    DreamWeaver = 3,
}

impl NodeEconomics {
    pub fn new(tier: NodeTier) -> Self {
        Self {
            tier,
            is_renting_out: false,
            monthly_earnings: 0,
            last_rent_update: Utc::now(),
        }
    }

    pub fn estimated_monthly_earnings(&self) -> u128 {
        match self.tier {
            NodeTier::Mini => 180,
            NodeTier::Light => 950,
            NodeTier::Full => 2850,
            NodeTier::DreamWeaver => 7200,
        }
    }
}

// =====================================================
// NOYAU PRINCIPAL
// =====================================================

pub struct SkyAInetNode {
    pub metadata: NodeMetadata,
    pub ethical_score: EthicalScore,
    pub state: NodeState,
    pub pouw_engine: PoUWEngine,
    pub dream_scoring: DreamScoring,
    pub economics: NodeEconomics,
    pub rewards: UserRewards,

    pub zip_memory: Option<Arc<Mutex<ZipMemory>>>,
    pub hybrid_transport: Option<Arc<Mutex<HybridTransport>>>,
    pub evolution_manager: Option<Arc<Mutex<EvolutionManager>>>,

    pub communication: Option<Arc<Mutex<crate::node_communication::NodeCommunication>>>,

    pub total_messages_processed: u64,
    pub total_bytes_stored: u64,
    pub last_flash_gematria: Option<DateTime<Utc>>,
}

impl SkyAInetNode {
    pub fn new(
        node_type: NodeType,
        role: NodeRole,
        subscription: SubscriptionLevel,
        capabilities: NodeCapabilities,
    ) -> Self {
        let id: [u8; 32] = rand::random();
        let now = Utc::now();
        let is_paid = subscription != SubscriptionLevel::Free;

        Self {
            metadata: NodeMetadata {
                id,
                node_type,
                node_role: role,
                subscription_level: subscription,
                peer_id: format!("peer-{}", hex::encode(&id[0..8])),
                capabilities,
                reputation_score: if is_paid { 0.82 } else { 0.65 },
                last_active: now,
                dream_contributions: 0,
                total_pouw_score: 0.0,
                zip_memory_enabled: true,
                created_at: now,
                is_paid,
            },
            ethical_score: EthicalScore {
                benevolence: 0.97,
                truthfulness: 0.96,
                non_malice: 0.98,
                sovereignty: 0.95,
                overall: 0.965,
            },
            state: NodeState::Initializing,
            pouw_engine: PoUWEngine::new(),
            dream_scoring: DreamScoring::new(),
            economics: NodeEconomics::new(if is_paid { NodeTier::Full } else { NodeTier::Mini }),
            rewards: UserRewards::new(if is_paid { crate::rewards::AccountType::NodeOwner } else { crate::rewards::AccountType::Free }),

            zip_memory: None,
            hybrid_transport: Some(Arc::new(Mutex::new(HybridTransport::new(true)))),
            evolution_manager: None,

            communication: Some(Arc::new(Mutex::new(
                crate::node_communication::NodeCommunication::new(format!("peer-{}", hex::encode(&id[0..8])))
            ))),

            total_messages_processed: 0,
            total_bytes_stored: 0,
            last_flash_gematria: None,
        }
    }

    // =====================================================
    // INITIALISATION & CYCLE DE VIE
    // =====================================================

    pub async fn start(&mut self) -> Result<(), String> {
        if self.state == NodeState::Active {
            return Ok(());
        }

        // Initialisation ZipMemory
        if self.metadata.zip_memory_enabled {
            let path = format!("./data/node_{}", hex::encode(&self.metadata.id[0..8]));
            let zip = ZipMemory::new(&path);
            self.zip_memory = Some(Arc::new(Mutex::new(zip)));
        }

        self.state = NodeState::Active;
        self.metadata.last_active = Utc::now();

        info!(
            "🟢 SkyAInetNode démarré | Type: {:?} | Tier: {:?} | Zip: ON | Reputation: {:.3}",
            self.metadata.node_type, self.economics.tier, self.metadata.reputation_score
        );

        Ok(())
    }

    pub async fn sleep(&mut self) {
        if self.state == NodeState::Sleeping { return; }
        if let Some(zip) = &self.zip_memory {
            let mut z = zip.lock().await;
            let _ = z.compress_inactive_data().await;
        }
        self.state = NodeState::Sleeping;
        debug!("😴 Node entered sleep mode");
    }

    pub async fn wake(&mut self) {
        if self.state != NodeState::Sleeping { return; }
        if let Some(zip) = &self.zip_memory {
            let mut z = zip.lock().await;
            let _ = z.decompress_on_demand().await;
        }
        self.state = NodeState::Active;
        self.metadata.last_active = Utc::now();
        debug!("🌅 Node woke up");
    }

    // =====================================================
    // FLASH GEMATRIA + ÉVOLUTION
    // =====================================================

    pub async fn trigger_flash_gematria(&mut self) {
        if self.metadata.node_role == NodeRole::Edge { return; }

        if let Some(transport) = &self.hybrid_transport {
            let mut t = transport.lock().await;
            let _ = t.set_flash_mode().await;
        }

        self.last_flash_gematria = Some(Utc::now());
        info!("⚡ Flash Gematria activated");
    }

    // =====================================================
    // STOCKAGE & UPLOAD
    // =====================================================

    pub async fn upload_data(&mut self, data: &[u8], label: &str) -> Result<String, String> {
        if let Some(zip) = &self.zip_memory {
            let mut z = zip.lock().await;
            let compressed = z.compress(data).await?;
            self.total_bytes_stored += compressed.len() as u64;
        }

        Ok(format!("stored://{}", uuid::Uuid::new_v4()))
    }

    // =====================================================
    // MISE À JOUR & SANTÉ
    // =====================================================

    pub fn update_overall_score(&mut self) {
        let pouw = self.pouw_engine.get_total_score();
        let dream = self.dream_scoring.get_total_score();

        self.metadata.total_pouw_score = (pouw * 0.62) + (dream * 0.38);

        let decay = if self.metadata.is_paid { 0.9975 } else { 0.991 };
        self.metadata.reputation_score = (self.metadata.reputation_score * decay)
            + (self.metadata.total_pouw_score * 0.25)
            + (self.metadata.dream_contributions as f64 * 0.002);

        self.metadata.reputation_score = self.metadata.reputation_score.clamp(0.1, 1.0);
    }

    pub fn record_activity(&mut self, bytes: u64) {
        self.metadata.last_active = Utc::now();
        self.total_messages_processed += 1;
        self.total_bytes_stored += bytes;

        self.ethical_score.overall = (self.ethical_score.overall * 0.982) + 0.018;
    }

    pub fn health_report(&self) -> String {
        format!(
            "SkyAInetNode {} | Tier: {:?} | State: {:?} | Rep: {:.3} | Messages: {} | Storage: {} bytes",
            self.metadata.peer_id,
            self.economics.tier,
            self.state,
            self.metadata.reputation_score,
            self.total_messages_processed,
            self.total_bytes_stored
        )
    }
}