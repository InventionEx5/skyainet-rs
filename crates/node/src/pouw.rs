// crates/node/src/pouw.rs
// =====================================================
// PoUWEngine v4.0 — Proof of Useful Work Avancé
// Gematria Flash + ZipMemory + Thevie Orchestration + Rewards Dynamiques
// =====================================================

use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};

use crate::dream_scoring::DreamScoring;
use skyainet_memory::zip_memory::ZipMemory;
use crate::rewards::UserRewards;

/// Preuve de contribution utile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionProof {
    pub node_id: [u8; 32],
    pub contribution_type: String,      // "dream_cycle", "validation", "storage", "flash_gematria", etc.
    pub score: f64,
    pub timestamp: DateTime<Utc>,
    pub proof_hash: String,
    pub epoch: u64,
    pub metadata: Option<String>,
    pub thevie_boost: f64,              // Bonus accordé par Thevie
    pub compressed_size: u64,           // Taille après ZipMemory
}

/// Statistiques globales du PoUW
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoUWStats {
    pub total_contributions: u64,
    pub total_rewards_distributed: u128,
    pub average_score: f64,
    pub top_contributors: Vec<([u8; 32], f64)>,
    pub current_epoch: u64,
    pub active_nodes: usize,
}

pub struct PoUWEngine {
    contributions: HashMap<[u8; 32], Vec<ContributionProof>>,
    total_rewards_pool: u128,
    current_epoch: u64,
    epoch_rewards: BTreeMap<u64, u128>,
    thevie_boosts: HashMap<[u8; 32], f64>,

    // Cache optimisé
    stats_cache: Option<PoUWStats>,
    last_stats_update: DateTime<Utc>,

    // ZipMemory pour compresser les preuves
    proof_storage: ZipMemory,
}

impl PoUWEngine {
    pub fn new() -> Self {
        Self {
            contributions: HashMap::new(),
            total_rewards_pool: 0,
            current_epoch: 0,
            epoch_rewards: BTreeMap::new(),
            thevie_boosts: HashMap::new(),
            stats_cache: None,
            last_stats_update: Utc::now(),
            proof_storage: ZipMemory::new("./data/pouw_proofs"),
        }
    }

    /// Enregistre une contribution avec compression intelligente
    pub async fn record_contribution(
        &mut self,
        node_id: [u8; 32],
        contribution_type: &str,
        score: f64,
        metadata: Option<String>,
        raw_data: Option<&[u8]>,
        rewards: &mut UserRewards,
    ) -> ContributionProof {
        let score = score.clamp(0.0, 1.0);
        let thevie_boost = *self.thevie_boosts.get(&node_id).unwrap_or(&0.0);

        let compressed_size = if let Some(data) = raw_data {
            if let Ok(compressed) = self.proof_storage.compress(data).await {
                compressed.len() as u64
            } else {
                data.len() as u64
            }
        } else {
            0
        };

        let proof = ContributionProof {
            node_id,
            contribution_type: contribution_type.to_string(),
            score,
            timestamp: Utc::now(),
            proof_hash: format!("pouw:{}", uuid::Uuid::new_v4()),
            epoch: self.current_epoch,
            metadata,
            thevie_boost,
            compressed_size,
        };

        self.contributions.entry(node_id).or_default().push(proof.clone());
        self.stats_cache = None;

        // Récompense utilisateur
        let base_reward = (score * 12.0) as u128;
        rewards.add_reward(crate::rewards::RewardReason::PoUWContribution, base_reward);

        debug!(
            "PoUW Contribution | Node: {} | Type: {} | Score: {:.3} | Boost: {:.2}x",
            hex::encode(&node_id[0..8]),
            contribution_type,
            score,
            thevie_boost
        );

        proof
    }

    /// Thevie accorde un boost exceptionnel
    pub fn apply_thevie_boost(&mut self, node_id: [u8; 32], boost: f64) {
        let clamped = boost.clamp(0.0, 2.5);
        self.thevie_boosts.insert(node_id, clamped);

        info!("Thevie Boost applied to node {} → +{:.2}x", hex::encode(&node_id[0..8]), clamped);
    }

    /// Calcul de récompense avec fidélité et boost Thevie
    pub fn calculate_node_reward(&self, node_id: &[u8; 32]) -> u128 {
        let node_contribs = self.contributions.get(node_id).map_or(0.0, |contribs| {
            contribs.iter()
                .map(|p| p.score * (1.0 + p.thevie_boost))
                .sum()
        });

        let global_total: f64 = self.contributions.values()
            .flat_map(|v| v.iter().map(|p| p.score * (1.0 + p.thevie_boost)))
            .sum();

        if global_total < 0.0001 {
            return 0;
        }

        let base = (node_contribs / global_total) * self.total_rewards_pool as f64;
        let loyalty = self.calculate_loyalty_bonus(node_id);

        (base * loyalty) as u128
    }

    fn calculate_loyalty_bonus(&self, node_id: &[u8; 32]) -> f64 {
        if let Some(contribs) = self.contributions.get(node_id) {
            if contribs.len() < 8 {
                return 1.0;
            }
            let oldest = contribs.iter().map(|c| c.timestamp).min().unwrap();
            let age_days = (Utc::now() - oldest).num_days() as f64;
            1.0 + (age_days / 420.0).min(0.65)   // Bonus max +65%
        } else {
            1.0
        }
    }

    pub fn add_to_rewards_pool(&mut self, amount: u128) {
        self.total_rewards_pool += amount;
        self.epoch_rewards.insert(self.current_epoch, amount);
    }

    pub fn on_new_epoch(&mut self, epoch: u64) {
        self.current_epoch = epoch;
        self.stats_cache = None;
        info!("New PoUW Epoch started: {}", epoch);
    }

    pub fn get_total_score(&self) -> f64 {
        self.contributions.values()
            .flat_map(|v| v.iter().map(|p| p.score * (1.0 + p.thevie_boost)))
            .sum()
    }

    pub fn get_global_stats(&mut self) -> PoUWStats {
        if let Some(cache) = &self.stats_cache {
            if (Utc::now() - self.last_stats_update).num_minutes() < 3 {
                return cache.clone();
            }
        }

        let total_contribs: u64 = self.contributions.values().map(|v| v.len() as u64).sum();
        let total_score = self.get_total_score();
        let avg_score = if total_contribs > 0 { total_score / total_contribs as f64 } else { 0.0 };

        let mut top: Vec<([u8; 32], f64)> = self.contributions.iter()
            .map(|(id, contribs)| {
                let score: f64 = contribs.iter().map(|p| p.score * (1.0 + p.thevie_boost)).sum();
                (*id, score)
            })
            .collect();

        top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        top.truncate(8);

        let stats = PoUWStats {
            total_contributions: total_contribs,
            total_rewards_distributed: self.total_rewards_pool,
            average_score: avg_score,
            top_contributors: top,
            current_epoch: self.current_epoch,
            active_nodes: self.contributions.len(),
        };

        self.stats_cache = Some(stats.clone());
        self.last_stats_update = Utc::now();

        stats
    }

    pub fn prune_old_contributions(&mut self, max_age_days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(max_age_days);

        for contribs in self.contributions.values_mut() {
            contribs.retain(|p| p.timestamp > cutoff);
        }

        self.contributions.retain(|_, v| !v.is_empty());
        self.stats_cache = None;

        debug!("Old PoUW contributions pruned (> {} days)", max_age_days);
    }
}

impl Default for PoUWEngine {
    fn default() -> Self {
        Self::new()
    }
}