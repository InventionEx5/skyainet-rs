// crates/node/src/node_communication.rs
// =====================================================
// NodeCommunication v4.0 — Réseau de Nœuds Vivant & Sécurisé
// HybridTransport + GematriaAead + GossipSub + Lesson Propagation
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

use skyainet_secure_transport::crypto::{
    hybrid::HybridTransport,
    gematria_aead::GematriaAead
};
use crate::pouw::ContributionProof;
use crate::skyainet_node::SkyAInetNode;

/// Message inter-nœuds standardisé
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMessage {
    pub from: String,
    pub to: Option<String>,           // None = broadcast
    pub message_type: String,         // "lesson", "flash_gematria", "sync_request", etc.
    pub payload: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<Vec<u8>>,   // Signature Dilithium5
}

/// Gestionnaire de communication entre nœuds
pub struct NodeCommunication {
    pub peer_id: String,
    pub hybrid_transport: Arc<Mutex<HybridTransport>>,
    pub last_broadcast: Option<DateTime<Utc>>,
    pub received_lessons: Vec<ContributionProof>,
    pub stats: CommunicationStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub lessons_propagated: u64,
    pub failed_broadcasts: u64,
    pub last_successful_sync: Option<DateTime<Utc>>,
}

impl NodeCommunication {
    pub fn new(peer_id: String, hybrid_transport: Arc<Mutex<HybridTransport>>) -> Self {
        Self {
            peer_id,
            hybrid_transport,
            last_broadcast: None,
            received_lessons: Vec::new(),
            stats: CommunicationStats {
                messages_sent: 0,
                messages_received: 0,
                lessons_propagated: 0,
                failed_broadcasts: 0,
                last_successful_sync: None,
            },
        }
    }

    // =====================================================
    // BROADCAST DE LEÇON (Version Optimisée)
    // =====================================================
    pub async fn broadcast_lesson(
        &mut self,
        lesson: &ContributionProof,
        quality_threshold: f32,
    ) -> Result<(), String> {
        if lesson.score < quality_threshold {
            return Ok(());
        }

        let lesson_data = serde_json::to_vec(lesson)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        // Chiffrement hybride (KemT369 + GematriaAead)
        let encrypted = {
            let transport = self.hybrid_transport.lock().await;
            transport.encrypt(&lesson_data)
                .map_err(|e| format!("Encryption failed: {}", e))?
        };

        // Publication via GossipSub (libp2p)
        let topic = "skyainet/lessons/v2";

        let transport = self.hybrid_transport.lock().await;
        transport.publish(topic, &encrypted).await
            .map_err(|e| format!("GossipSub publish failed: {}", e))?;

        self.stats.messages_sent += 1;
        self.stats.lessons_propagated += 1;
        self.last_broadcast = Some(Utc::now());

        debug!(
            "[NodeComm] Lesson broadcasted | Quality: {:.3} | Topic: {}",
            lesson.score, topic
        );

        Ok(())
    }

    // =====================================================
    // RÉCEPTION DE LEÇON
    // =====================================================
    pub async fn receive_remote_lesson(
        &mut self,
        encrypted_data: &[u8],
    ) -> Result<ContributionProof, String> {
        // Déchiffrement hybride
        let decrypted = {
            let transport = self.hybrid_transport.lock().await;
            transport.decrypt(encrypted_data)
                .map_err(|e| format!("Decryption failed: {}", e))?
        };

        let lesson: ContributionProof = serde_json::from_slice(&decrypted)
            .map_err(|e| format!("Deserialization failed: {}", e))?;

        self.received_lessons.push(lesson.clone());
        self.stats.messages_received += 1;

        debug!(
            "[NodeComm] Lesson received from network | Quality: {:.3}",
            lesson.score
        );

        Ok(lesson)
    }

    // =====================================================
    // COORDINATION FLASH GEMATRIA GLOBAL
    // =====================================================
    pub async fn coordinate_global_flash(&mut self) -> Result<(), String> {
        let signal = b"FLASH_GEMATRIA|GLOBAL|PRIORITY";

        let transport = self.hybrid_transport.lock().await;
        transport.publish("skyainet/signals/v2", signal).await
            .map_err(|e| format!("Global flash signal failed: {}", e))?;

        self.stats.messages_sent += 1;
        self.last_broadcast = Some(Utc::now());

        info!("Global Flash Gematria signal broadcasted successfully");
        Ok(())
    }

    // =====================================================
    // STATISTIQUES & MAINTENANCE
    // =====================================================
    pub fn get_stats(&self) -> CommunicationStats {
        self.stats.clone()
    }

    pub fn prune_old_lessons(&mut self, max_age_days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(max_age_days);
        self.received_lessons.retain(|l| l.timestamp > cutoff);
    }
}