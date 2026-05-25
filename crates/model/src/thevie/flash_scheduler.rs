// crates/model/src/thevie/flash_scheduler.rs
// =====================================================
// Thevie Flash Scheduler v1.0 — Scheduler Intelligent Simplifié
// Remplace le Scheduler Poisson + Stéganographie Markov complexe
// =====================================================

use tokio::time::{interval, Duration};
use tracing::{info, debug};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::thevie::Thevie;

pub struct ThevieFlashScheduler {
    thevie: Arc<Mutex<Thevie>>,
    interval_seconds: u64,
}

impl ThevieFlashScheduler {
    pub fn new(thevie: Arc<Mutex<Thevie>>, interval_seconds: u64) -> Self {
        Self {
            thevie,
            interval_seconds,
        }
    }

    /// Démarre le scheduler intelligent
    pub async fn start(&self) {
        let thevie = self.thevie.clone();
        let interval_sec = self.interval_seconds;

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(interval_sec));
            loop {
                ticker.tick().await;

                let mut thevie = thevie.lock().await;

                // Thevie décide elle-même quand déclencher un Flash Gematria
                if thevie.collective.global_wisdom < 0.75 || 
                   thevie.total_queries_processed % 47 == 0 {
                    
                    thevie.node.trigger_flash_gematria().await;
                    info!("[FlashScheduler] Flash Gematria déclenché par Thevie (sagesse: {:.2})", 
                          thevie.collective.global_wisdom);
                }
            }
        });

        info!("[FlashScheduler] Thevie Flash Scheduler démarré (intervalle: {}s)", interval_sec);
    }
}