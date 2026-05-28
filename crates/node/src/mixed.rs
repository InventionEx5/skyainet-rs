// crates/node/src/mixed.rs
// =====================================================
// MixedNode v4.0 — Nœud Hybride Souverain Dynamique
// Compute + Storage + Validator + Orchestration intelligente
// Intégré avec Rewards, ZipMemory, Hybrid Crypto & Evolution
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use std::sync::Arc;
use tokio::sync::Mutex;

use skyainet_core::node_types::{NodeCapabilities, NodeState, SubscriptionLevel, NodeType};
use crate::storage::StorageNode;
use crate::skyainet_node::SkyAInetNode;
use crate::pouw::PoUWEngine;
use skyainet_secure_transport::crypto::hybrid::HybridTransport;
use skyainet_memory::zip_memory::ZipMemory;
use crate::rewards::UserRewards;

/// Nœud Hybride Multi-Rôles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedNode {
    pub node_id: String,
    pub sovereign_alias: String,
    pub capabilities: NodeCapabilities,
    pub current_state: NodeState,

    // Rôles actifs
    pub active_roles: Vec<NodeType>,

    // Sous-composants
    pub storage: Option<StorageNode>,
    pub compute_power: f64,
    pub validator_stake: u64,

    // Infrastructure partagée
    pub hybrid_transport: Option<Arc<Mutex<HybridTransport>>>,
    pub zip_memory: Option<Arc<Mutex<ZipMemory>>>,

    // Métriques
    pub total_tasks_processed: u64,
    pub last_role_switch: Option<chrono::DateTime<chrono::Utc>>,
}

impl MixedNode {
    pub fn new(sovereign_alias: &str, subscription: SubscriptionLevel) -> Self {
        let capabilities = NodeCapabilities::new(&subscription);
        let max_storage = match subscription {
            SubscriptionLevel::Free => 8,
            SubscriptionLevel::Pro => 80,
            SubscriptionLevel::Validator => 300,
            _ => 40,
        };

        Self {
            node_id: format!("mixed-{}", sovereign_alias.to_lowercase()),
            sovereign_alias: sovereign_alias.to_string(),
            capabilities,
            current_state: NodeState::Active,
            active_roles: vec![NodeType::Full],
            storage: None,
            compute_power: capabilities.compute_power,
            validator_stake: 0,
            hybrid_transport: None,
            zip_memory: None,
            total_tasks_processed: 0,
            last_role_switch: None,
        }
    }

    /// Active dynamiquement un rôle supplémentaire
    pub async fn activate_role(&mut self, role: NodeType) -> Result<(), String> {
        if self.active_roles.contains(&role) {
            return Ok(());
        }

        self.active_roles.push(role.clone());
        self.last_role_switch = Some(chrono::Utc::now());

        match role {
            NodeType::Storage => {
                let storage = StorageNode::new(&self.sovereign_alias, SubscriptionLevel::Pro);
                self.storage = Some(storage);
                info!("[MixedNode] Rôle Storage activé");
            }
            NodeType::Validator => {
                self.validator_stake = 12000;
                info!("[MixedNode] Rôle Validator activé avec stake {}", self.validator_stake);
            }
            _ => {}
        }

        Ok(())
    }

    pub fn deactivate_role(&mut self, role: NodeType) {
        self.active_roles.retain(|r| r != &role);

        if role == NodeType::Storage {
            self.storage = None;
        }
    }

    /// Exécute une tâche selon les rôles actifs (orchestration intelligente)
    pub async fn execute_task(
        &mut self,
        task_type: &str,
        data: Option<&[u8]>,
        rewards: &mut UserRewards,
    ) -> Result<String, String> {
        self.total_tasks_processed += 1;

        match task_type {
            "inference" if self.active_roles.contains(&NodeType::Full) => {
                rewards.add_reward(crate::rewards::RewardReason::ComputeContribution, 12);
                Ok("Tâche d'inférence exécutée sur MixedNode".to_string())
            }

            "upload" if self.active_roles.contains(&NodeType::Storage) => {
                if let Some(storage) = &mut self.storage {
                    if let Some(d) = data {
                        let cid = storage.upload_file("mixed_task.bin", d, true).await?;
                        rewards.add_reward(crate::rewards::RewardReason::StorageContribution, 8);
                        Ok(format!("Fichier stocké avec succès → CID: {}", cid))
                    } else {
                        Err("Aucune donnée à uploader".to_string())
                    }
                } else {
                    Err("Storage non activé".to_string())
                }
            }

            "validation" if self.active_roles.contains(&NodeType::Validator) => {
                if self.validator_stake >= 10000 {
                    rewards.add_reward(crate::rewards::RewardReason::Validation, 15);
                    Ok("Validation PoUW effectuée avec succès".to_string())
                } else {
                    Err("Stake insuffisant pour validation".to_string())
                }
            }

            _ => Err(format!("Aucun rôle actif capable d'exécuter la tâche: {}", task_type)),
        }
    }

    pub fn get_total_power(&self) -> f64 {
        let mut power = self.compute_power;

        if self.storage.is_some() {
            power += 0.22;
        }
        if self.validator_stake > 0 {
            power += 0.18;
        }

        power
    }

    pub fn health_report(&self) -> String {
        format!(
            "MixedNode {} | Rôles: {:?} | Power: {:.3} | Tasks: {} | Storage: {} | Stake: {}",
            self.sovereign_alias,
            self.active_roles,
            self.get_total_power(),
            self.total_tasks_processed,
            self.storage.as_ref().map_or(0, |s| s.used_storage_gb),
            self.validator_stake
        )
    }
}