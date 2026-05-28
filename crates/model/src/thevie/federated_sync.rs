// crates/model/src/thevie/federated_sync.rs
// =====================================================
// Federated Sync v6.0 — Synchronisation Fédérée Intelligente
// Propagation sécurisée des leçons + Apprentissage collectif
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
    pub failed_broadcasts: u64,
    pub last_sync: Option<u64>,
}

pub struct FederatedSync {
    mesh: Arc<Mutex<NeuralMesh>>,
    collective: Arc<Mutex<CollectiveConsciousness>>,
    
    sync_interval: u64,
    min_quality_threshold: f32,
    min_reputation_threshold: f64,
    max_lessons_per_sync: usize,
    
    transport: Option<Arc<dyn Transport + Send + Sync>>,
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
            sync_interval: 280,                    // \~4.5 minutes
            min_quality_threshold: 0.83,
            min_reputation_threshold: 0.72,
            max_lessons_per_sync: 18,
            transport: None,
            roman: RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256),
            stats: FederatedSyncStats {
                total_syncs: 0,
                lessons_propagated: 0,
                failed_broadcasts: 0,
                last_sync: None,
            },
        }
    }

    pub fn with_transport(mut self, transport: Arc<dyn Transport + Send + Sync>) -> Self {
        self.transport = Some(transport);
        self
    }

    /// Synchronisation complète avec les pairs
    pub async fn sync_with_peers(&mut self) {
        let mesh = self.mesh.lock().await;
        let collective = self.collective.lock().await;

        if collective.global_wisdom < self.min_reputation_threshold {
            debug!("[FederatedSync] Sagesse collective trop faible pour synchronisation");
            return;
        }

        let high_quality_lessons: Vec<Lesson> = mesh
            .get_all_lessons()
            .into_iter()
            .filter(|l| l.quality >= self.min_quality_threshold)
            .take(self.max_lessons_per_sync)
            .collect();

        if high_quality_lessons.is_empty() {
            debug!("[FederatedSync] Aucune leçon qualifiée à propager");
            return;
        }

        info!("[FederatedSync] Propagation de {} leçons de haute qualité", high_quality_lessons.len());

        for lesson in high_quality_lessons {
            if let Err(e) = self.broadcast_lesson(&lesson).await {
                self.stats.failed_broadcasts += 1;
                warn!("[FederatedSync] Échec de broadcast : {}", e);
            } else {
                self.stats.lessons_propagated += 1;
            }
        }

        self.stats.total_syncs += 1;
        self.stats.last_sync = Some(crate::utils::now_millis());

        // Évolution passive de la sagesse collective
        let mut collective = self.collective.lock().await;
        collective.global_wisdom = (collective.global_wisdom + 0.003).min(0.98);
    }

    async fn broadcast_lesson(&self, lesson: &Lesson) -> Result<(), String> {
        let serialized = serde_json::to_vec(lesson)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        let encrypted = self.roman.encrypt(&serialized);

        if let Some(transport) = &self.transport {
            transport
                .broadcast("skyainet/lesson", &encrypted)
                .await
                .map_err(|e| format!("Broadcast failed: {}", e))?;

            debug!("📡 Leçon diffusée via transport sécurisé ({} octets)", encrypted.len());
        } else {
            debug!("[FederatedSync] [LOCAL MODE] Leçon prête pour diffusion");
        }

        Ok(())
    }

    /// Réception d'une leçon poussée par un autre nœud
    pub async fn receive_pushed_lesson(&mut self, lesson: Lesson, node_reputation: f64) {
        if lesson.quality < self.min_quality_threshold || node_reputation < self.min_reputation_threshold {
            debug!("[FederatedSync] Leçon rejetée (qualité ou réputation insuffisante)");
            return;
        }

        let mut mesh = self.mesh.lock().await;
        mesh.add_lesson_from_node(lesson.clone(), node_reputation);

        let mut collective = self.collective.lock().await;
        collective.global_wisdom = (collective.global_wisdom + 0.0025).min(0.98);

        info!("[FederatedSync] Leçon reçue et intégrée (qualité: {:.2})", lesson.quality);
    }

    /// Demande spécifique de leçons sur un sujet
    pub async fn request_specific_lessons(&self, topic: &str, min_quality: f32) -> Vec<Lesson> {
        let mesh = self.mesh.lock().await;
        let lessons = mesh.get_lessons_by_topic(topic)
            .into_iter()
            .filter(|l| l.quality >= min_quality)
            .collect();

        debug!("[FederatedSync] {} leçons trouvées pour le sujet '{}'", lessons.len(), topic);
        lessons
    }

    /// Démarrage de la synchronisation en arrière-plan
    pub fn start_background_sync(&self) {
        let mut sync = self.clone(); // Clone léger grâce à Arc
        let interval = self.sync_interval;

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(interval));
            info!("[FederatedSync] Synchronisation en arrière-plan activée ({}s)", interval);

            loop {
                ticker.tick().await;
                sync.sync_with_peers().await;
            }
        });
    }

    pub fn get_stats(&self) -> FederatedSyncStats {
        self.stats.clone()
    }
}

impl Clone for FederatedSync {
    fn clone(&self) -> Self {
        Self {
            mesh: Arc::clone(&self.mesh),
            collective: Arc::clone(&self.collective),
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