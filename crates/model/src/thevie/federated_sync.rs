// crates/model/src/thevie/federated_sync.rs
// =====================================================
// Federated Sync v5.4 — Version Finale Unifiée
// SkyAInet × Thevie — Synchronisation Fédérée Sécurisée
// =====================================================

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug, warn, error};

use crate::thevie::neural_mesh::{NeuralMesh, Lesson};
use crate::thevie::collective_consciousness::CollectiveConsciousness;
use skyainet_secure_transport::crypto::roman_t369::{RomanT369, GematriaMode};
use skyainet_secure_transport::transport::Transport;

#[derive(Debug, Clone)]
pub struct FederatedSyncStats {
    pub total_syncs: u64,
    pub lessons_propagated: u64,
    pub last_sync: Option<u64>,
    pub failed_broadcasts: u64,
}

pub struct FederatedSync {
    pub mesh: Arc<Mutex<NeuralMesh>>,
    pub collective: Arc<Mutex<CollectiveConsciousness>>,
    pub sync_interval: u64,
    pub min_quality_threshold: f32,
    pub min_reputation_threshold: f64,
    pub max_lessons_per_sync: usize,
    pub transport: Option<Arc<dyn Transport + Send + Sync>>,
    roman: RomanT369,
    stats: FederatedSyncStats,
}

impl FederatedSync {
    pub fn new(
        mesh: Arc<Mutex<NeuralMesh>>,
        collective: Arc<Mutex<CollectiveConsciousness>>,
    ) -> Self {
        Self {
            mesh,
            collective,
            sync_interval: 300,
            min_quality_threshold: 0.82,
            min_reputation_threshold: 0.75,
            max_lessons_per_sync: 20,
            transport: None,
            roman: RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256),
            stats: FederatedSyncStats {
                total_syncs: 0,
                lessons_propagated: 0,
                last_sync: None,
                failed_broadcasts: 0,
            },
        }
    }

    pub fn with_transport(mut self, transport: Arc<dyn Transport + Send + Sync>) -> Self {
        self.transport = Some(transport);
        self
    }

    pub async fn sync_with_peers(&mut self) {
        let mesh = self.mesh.lock().await;
        let collective = self.collective.lock().await;

        let mut high_quality_lessons: Vec<Lesson> = mesh
            .get_all_lessons()
            .into_iter()
            .filter(|lesson| lesson.quality >= self.min_quality_threshold)
            .collect();

        if high_quality_lessons.is_empty() {
            debug!("[FederatedSync] Aucune leçon de haute qualité à propager");
            return;
        }

        if collective.global_wisdom < self.min_reputation_threshold {
            warn!("[FederatedSync] Sagesse collective trop faible pour synchronisation");
            return;
        }

        if high_quality_lessons.len() > self.max_lessons_per_sync {
            high_quality_lessons.truncate(self.max_lessons_per_sync);
        }

        info!(
            "[FederatedSync] Propagation de {} leçons de haute qualité",
            high_quality_lessons.len()
        );

        for lesson in high_quality_lessons {
            match self.broadcast_lesson(&lesson).await {
                Ok(_) => {
                    self.stats.lessons_propagated += 1;
                }
                Err(e) => {
                    self.stats.failed_broadcasts += 1;
                    error!("[FederatedSync] Échec de propagation : {}", e);
                }
            }
        }

        self.stats.total_syncs += 1;
        self.stats.last_sync = Some(crate::utils::now_millis());

        collective.passive_evolution_tick();
    }

    async fn broadcast_lesson(&self, lesson: &Lesson) -> Result<(), String> {
        let serialized = serde_json::to_vec(lesson)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        let encrypted = self.roman.encrypt(&serialized);

        if let Some(transport) = &self.transport {
            transport
                .broadcast("skyainet/federated_sync", &encrypted)
                .await
                .map_err(|e| format!("Transport broadcast failed: {}", e))?;

            debug!(
                "[FederatedSync] Leçon diffusée via libp2p ({} octets, qualité: {:.2})",
                encrypted.len(),
                lesson.quality
            );
        } else {
            debug!("[FederatedSync] [LOCAL] Leçon prête ({} octets)", encrypted.len());
        }

        Ok(())
    }

    pub async fn on_node_connected(&mut self, node_reputation: f64, dream_contribution: f64, pouw_score: f64) {
        info!("[FederatedSync] Nœud connecté → Synchronisation renforcée activée");

        let contribution_score = self.calculate_contribution_score(node_reputation, dream_contribution, pouw_score);

        if contribution_score > 0.75 {
            self.sync_high_value_lessons().await;
        } else {
            self.sync_standard_lessons().await;
        }

        let mut collective = self.collective.lock().await;
        collective.global_wisdom = (collective.global_wisdom + 0.004).min(0.98);
    }

    fn calculate_contribution_score(&self, reputation: f64, dream: f64, pouw: f64) -> f64 {
        (reputation * 0.40) + (dream * 0.35) + (pouw * 0.25)
    }

    async fn sync_high_value_lessons(&self) {
        let mesh = self.mesh.lock().await;

        let lessons: Vec<Lesson> = mesh
            .get_all_lessons()
            .into_iter()
            .filter(|l| l.quality >= self.min_quality_threshold)
            .collect();

        for lesson in lessons {
            let encrypted = self.roman.encrypt(&serde_json::to_vec(&lesson).unwrap());
            if let Some(transport) = &self.transport {
                let _ = transport.broadcast("skyainet/federated_sync", &encrypted).await;
            }
        }
    }

    async fn sync_standard_lessons(&self) {
        let mesh = self.mesh.lock().await;

        let lessons: Vec<Lesson> = mesh
            .get_all_lessons()
            .into_iter()
            .filter(|l| l.quality >= 0.70)
            .take(10)
            .collect();

        for lesson in lessons {
            let encrypted = self.roman.encrypt(&serde_json::to_vec(&lesson).unwrap());
            if let Some(transport) = &self.transport {
                let _ = transport.broadcast("skyainet/federated_sync", &encrypted).await;
            }
        }
    }

    pub async fn receive_pushed_lesson(&mut self, lesson: Lesson, node_reputation: f64, dream_contribution: f64, pouw_score: f64) {
        let contribution = self.calculate_contribution_score(node_reputation, dream_contribution, pouw_score);

        if contribution < 0.60 || lesson.quality < self.min_quality_threshold {
            debug!("[FederatedSync] Leçon poussée rejetée (contribution insuffisante)");
            return;
        }

        let mut mesh = self.mesh.lock().await;
        mesh.add_lesson_from_node(lesson.clone(), contribution);

        info!("[FederatedSync] Leçon poussée acceptée (qualité: {:.2})", lesson.quality);

        let mut collective = self.collective.lock().await;
        collective.global_wisdom = (collective.global_wisdom + 0.002).min(0.98);
    }

    pub async fn request_specific_lessons(&self, topic: &str, min_quality: f32) -> Vec<Lesson> {
        let mesh = self.mesh.lock().await;

        let requested: Vec<Lesson> = mesh
            .get_lessons_by_topic(topic)
            .into_iter()
            .filter(|l| l.quality >= min_quality)
            .collect();

        info!(
            "[FederatedSync] Requête de leçons sur '{}' → {} résultats",
            topic,
            requested.len()
        );

        requested
    }

    pub async fn start_background_sync(&self) {
        let mut sync = self.clone();
        let interval = self.sync_interval;

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(interval));
            info!("[FederatedSync] Synchronisation fédérée activée (toutes les {}s)", interval);

            loop {
                ticker.tick().await;
                sync.sync_with_peers().await;
            }
        });
    }

    pub async fn force_sync(&mut self) {
        info!("[FederatedSync] Synchronisation forcée");
        self.sync_with_peers().await;
    }

    pub fn get_stats(&self) -> FederatedSyncStats {
        self.stats.clone()
    }
}

impl Clone for FederatedSync {
    fn clone(&self) -> Self {
        Self {
            mesh: self.mesh.clone(),
            collective: self.collective.clone(),
            sync_interval: self.sync_interval,
            min_quality_threshold: self.min_quality_threshold,
            min_reputation_threshold: self.min_reputation_threshold,
            max_lessons_per_sync: self.max_lessons_per_sync,
            transport: self.transport.clone(),
            roman: self.roman.clone(),
            stats: self.stats.clone(),
        }
    }
}