// crates/node/src/storage.rs
// =====================================================
// StorageNode v5.0 — Gestionnaire de Stockage Souverain
// ZipMemory + Chiffrement Hybride Post-Quantique + Facturation + Réplication
// =====================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};

use skyainet_core::node_types::{NodeCapabilities, NodeState, SubscriptionLevel};
use skyainet_secure_transport::crypto::{
    hybrid::HybridTransport,
    gematria_aead::GematriaAead,
    roman_t369::{RomanT369, GematriaMode}
};
use skyainet_memory::zip_memory::ZipMemory;
use crate::rewards::UserRewards;

/// Nœud de Stockage Avancé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageNode {
    pub node_id: String,
    pub sovereign_alias: String,
    pub capabilities: NodeCapabilities,
    pub current_state: NodeState,

    // === Stockage & Quota ===
    pub used_storage_gb: u64,
    pub reserved_gb: u64,
    pub max_storage_gb: u64,
    pub total_files: u32,

    // === Compression & Cache ===
    pub zip_memory: ZipMemory,
    pub hot_cache: HashMap<String, Vec<u8>>,

    // === Chiffrement ===
    pub hybrid: HybridTransport,
    pub encrypted_files: HashMap<String, String>, // filename → encrypted_cid

    // === Métriques & Facturation ===
    pub last_billing_update: u64,
    pub monthly_cost_sky: f64,
    pub storage_shield_enabled: bool, // Réplication renforcée (+0.2 SKY/GB)
}

impl StorageNode {
    pub fn new(sovereign_alias: &str, subscription: SubscriptionLevel) -> Self {
        let capabilities = NodeCapabilities::new(&subscription);
        let max_storage = match subscription {
            SubscriptionLevel::Free => 5,
            SubscriptionLevel::Pro => 50,
            SubscriptionLevel::Validator => 200,
            _ => 20,
        };

        Self {
            node_id: format!("storage-{}", sovereign_alias.to_lowercase()),
            sovereign_alias: sovereign_alias.to_string(),
            capabilities,
            current_state: NodeState::Active,
            used_storage_gb: 0,
            reserved_gb: 0,
            max_storage_gb: max_storage,
            total_files: 0,
            zip_memory: ZipMemory::new(&format!("./data/storage/{}_zip", sovereign_alias)),
            hot_cache: HashMap::new(),
            hybrid: HybridTransport::new(true), // Mode full post-quantum
            encrypted_files: HashMap::new(),
            last_billing_update: crate::utils::now_millis(),
            monthly_cost_sky: 0.0,
            storage_shield_enabled: false,
        }
    }

    /// Upload optimisé avec ZipMemory + chiffrement hybride
    pub async fn upload_file(&mut self, filename: &str, data: &[u8], rewards: &mut UserRewards) -> Result<String, String> {
        let raw_size_gb = (data.len() as f64 / (1024.0 * 1024.0 * 1024.0)) as u64;

        if self.used_storage_gb + raw_size_gb > self.max_storage_gb {
            return Err("Quota de stockage dépassé".to_string());
        }

        // === Compression intelligente ===
        let compressed = self.zip_memory.compress(data).await?;
        let compressed_size_gb = (compressed.len() as f64 / (1024.0 * 1024.0 * 1024.0)) as u64;

        // === Chiffrement Hybride ===
        let (key, nonce) = self.hybrid.derive_keys();
        let aead = GematriaAead::new(key, nonce);
        let encrypted = aead.encrypt(&compressed);

        let cid = format!("skn:{}", uuid::Uuid::new_v4());

        self.used_storage_gb += compressed_size_gb;
        self.total_files += 1;
        self.encrypted_files.insert(filename.to_string(), cid.clone());

        // Mise à jour coût mensuel
        self.update_monthly_cost();

        // Récompense légère pour contribution stockage
        rewards.add_reward(crate::rewards::RewardReason::StorageContribution, 3);

        debug!("[Storage] Fichier uploadé : {} → {} GB (compressé)", filename, compressed_size_gb);

        Ok(cid)
    }

    pub async fn download_file(&self, filename: &str) -> Option<Vec<u8>> {
        let cid = self.encrypted_files.get(filename)?;

        // Simulation récupération (à remplacer par vrai stockage décentralisé)
        let encrypted = vec![0u8; 1024]; // Placeholder

        let (key, nonce) = self.hybrid.derive_keys();
        let aead = GematriaAead::new(key, nonce);
        
        if let Some(decrypted) = aead.decrypt(&encrypted) {
            self.zip_memory.decompress(&decrypted).await.ok()
        } else {
            None
        }
    }

    pub fn delete_file(&mut self, filename: &str) -> bool {
        if let Some(_) = self.encrypted_files.remove(filename) {
            self.total_files = self.total_files.saturating_sub(1);
            // TODO: libérer l'espace réel dans ZipMemory
            true
        } else {
            false
        }
    }

    fn update_monthly_cost(&mut self) {
        let base_rate = if self.storage_shield_enabled { 0.7 } else { 0.5 }; // SKY/GB/mois
        self.monthly_cost_sky = (self.used_storage_gb as f64 * base_rate).max(0.5);
    }

    pub fn toggle_storage_shield(&mut self) {
        self.storage_shield_enabled = !self.storage_shield_enabled;
        self.update_monthly_cost();
        info!("[Storage] Storage Shield {}", if self.storage_shield_enabled { "activé" } else { "désactivé" });
    }

    pub fn get_storage_stats(&self) -> (u64, u64, f64, f64) {
        let usage_percent = (self.used_storage_gb as f64 / self.max_storage_gb as f64) * 100.0;
        (self.used_storage_gb, self.max_storage_gb, usage_percent, self.monthly_cost_sky)
    }

    pub fn enter_sleep_mode(&mut self) {
        self.current_state = NodeState::Sleeping;
        info!("[Storage] Nœud passé en mode veille");
    }

    pub fn health_report(&self) -> String {
        format!(
            "StorageNode {} | Used: {}GB / {}GB | Shield: {} | Files: {} | Cost: {:.2} SKY/mois",
            self.sovereign_alias,
            self.used_storage_gb,
            self.max_storage_gb,
            if self.storage_shield_enabled { "ON" } else { "OFF" },
            self.total_files,
            self.monthly_cost_sky
        )
    }
}