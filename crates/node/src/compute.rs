// crates/node/src/compute.rs
// =====================================================
// ComputeNode v5.0 — Nœud de Calcul Haute Performance Souverain
// TFLOPS Dynamique + Location Marketplace + Intégration Rewards & Evolution
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

use skyainet_core::node_types::{NodeCapabilities, NodeState, SubscriptionLevel, NodeType};
use crate::marketplace::ComputeMarketplace;
use crate::rewards::UserRewards;
use skyainet_memory::zip_memory::ZipMemory;

/// Nœud de Calcul Avancé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeNode {
    pub node_id: String,
    pub sovereign_alias: String,
    pub capabilities: NodeCapabilities,
    pub current_state: NodeState,

    // Puissance de calcul
    pub total_tfops: u32,
    pub available_tfops: u32,
    pub current_tasks: u32,
    pub max_concurrent_tasks: u32,
    pub total_tasks_completed: u64,

    // Location Marketplace
    pub is_rented: bool,
    pub rental_price_per_tfop: f64,      // en SKY par heure
    pub current_renter: Option<String>,

    // Infrastructure partagée
    pub zip_memory: Option<Arc<Mutex<ZipMemory>>>,

    // Métriques & Historique
    pub last_task_time: Option<DateTime<Utc>>,
    pub total_compute_time_seconds: u64,
}

impl ComputeNode {
    pub fn new(sovereign_alias: &str, tfops: u32, subscription: SubscriptionLevel) -> Self {
        let mut capabilities = NodeCapabilities::new(&subscription);
        capabilities.compute_power = 0.92;

        Self {
            node_id: format!("compute-{}", sovereign_alias.to_lowercase()),
            sovereign_alias: sovereign_alias.to_string(),
            capabilities,
            current_state: NodeState::Active,
            total_tfops: tfops,
            available_tfops: tfops,
            current_tasks: 0,
            max_concurrent_tasks: 12,
            total_tasks_completed: 0,
            is_rented: false,
            rental_price_per_tfop: 0.095,
            current_renter: None,
            zip_memory: None,
            last_task_time: None,
            total_compute_time_seconds: 0,
        }
    }

    /// Initialise ZipMemory pour ce nœud
    pub fn init_zip_memory(&mut self, base_path: &str) {
        let path = format!("{}/compute_{}", base_path, self.sovereign_alias);
        self.zip_memory = Some(Arc::new(Mutex::new(ZipMemory::new(&path))));
        info!("[ComputeNode] ZipMemory initialisé pour {}", self.node_id);
    }

    /// Accepte une tâche avec vérification intelligente
    pub async fn accept_task(
        &mut self,
        required_tfops: u32,
        task_id: &str,
        rewards: &mut UserRewards,
    ) -> Result<String, String> {
        if self.current_tasks >= self.max_concurrent_tasks {
            return Err("Nombre maximum de tâches concurrentes atteint".to_string());
        }

        if self.available_tfops < required_tfops {
            return Err(format!(
                "Puissance insuffisante (disponible: {} TFLOPS, requis: {})",
                self.available_tfops, required_tfops
            ));
        }

        self.available_tfops -= required_tfops;
        self.current_tasks += 1;
        self.last_task_time = Some(Utc::now());

        // Récompense pour contribution compute
        rewards.add_reward(crate::rewards::RewardReason::ComputeContribution, 
                          (required_tfops as f64 * 0.8) as u128);

        debug!("[ComputeNode] Tâche {} acceptée | TFLOPS utilisés: {}", task_id, required_tfops);

        Ok(format!("task-accepted:{}", task_id))
    }

    /// Termine une tâche et libère les ressources
    pub async fn complete_task(&mut self, tfops_used: u32, task_duration_seconds: u64) {
        self.available_tfops += tfops_used;
        self.current_tasks = self.current_tasks.saturating_sub(1);
        self.total_tasks_completed += 1;
        self.total_compute_time_seconds += task_duration_seconds;

        if let Some(zip) = &self.zip_memory {
            let mut z = zip.lock().await;
            let _ = z.compress_inactive_data().await;
        }

        debug!("[ComputeNode] Tâche terminée | Durée: {}s", task_duration_seconds);
    }

    /// Met le nœud en location sur le Marketplace
    pub fn put_for_rent(&mut self, price_per_tfop: f64) {
        self.is_rented = true;
        self.rental_price_per_tfop = price_per_tfop.clamp(0.05, 0.45);
        info!("[ComputeNode] Nœud mis en location à {} SKY/TFLOP/h", price_per_tfop);
    }

    pub fn stop_rental(&mut self) {
        self.is_rented = false;
        self.current_renter = None;
        info!("[ComputeNode] Location arrêtée");
    }

    pub fn get_utilization_rate(&self) -> f64 {
        if self.total_tfops == 0 {
            0.0
        } else {
            ((self.total_tfops - self.available_tfops) as f64 / self.total_tfops as f64) * 100.0
        }
    }

    pub fn health_report(&self) -> String {
        format!(
            "ComputeNode {} | TFLOPS: {} / {} | Tasks: {}/{} | Utilization: {:.1}% | Rented: {}",
            self.sovereign_alias,
            self.available_tfops,
            self.total_tfops,
            self.current_tasks,
            self.max_concurrent_tasks,
            self.get_utilization_rate(),
            if self.is_rented { "YES" } else { "NO" }
        )
    }
}