// crates/model/src/thevie/flash_scheduler.rs
// =====================================================
// Thevie Flash Scheduler v2.0 — Orchestrateur Intelligent
// Déclenche Flash Gematria selon sagesse collective + rythme d'activité
// =====================================================

use tokio::time::{interval, Duration};
use tracing::{info, debug, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::thevie::Thevie;

/// Scheduler intelligent qui gère les Flash Gematria
pub struct ThevieFlashScheduler {
    thevie: Arc<Mutex<Thevie>>,
    interval_seconds: u64,
    tick_count: u64,
}

impl ThevieFlashScheduler {
    pub fn new(thevie: Arc<Mutex<Thevie>>, interval_seconds: u64) -> Self {
        Self {
            thevie,
            interval_seconds: interval_seconds.max(15), // Minimum 15 secondes
            tick_count: 0,
        }
    }

    /// Démarre le scheduler en tâche de fond
    pub async fn start(&self) {
        let thevie = self.thevie.clone();
        let interval_sec = self.interval_seconds;

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(interval_sec));
            let mut tick_count: u64 = 0;

            info!("[FlashScheduler] Démarré avec intervalle de {} secondes", interval_sec);

            loop {
                ticker.tick().await;
                tick_count += 1;

                let mut thevie_guard = thevie.lock().await;

                // Décision intelligente de déclenchement
                let should_flash = 
                    thevie_guard.collective.global_wisdom < 0.76 ||           // Sagesse trop basse
                    tick_count % 47 == 0 ||                                   // Rythme naturel
                    thevie_guard.total_queries_processed % 53 == 0;           // Basé sur activité réelle

                if should_flash {
                    thevie_guard.node.trigger_flash_gematria().await;
                    
                    info!(
                        "[FlashScheduler] ⚡ Flash Gematria déclenché | Sagesse: {:.3} | Requêtes: {} | Tick: {}",
                        thevie_guard.collective.global_wisdom,
                        thevie_guard.total_queries_processed,
                        tick_count
                    );
                } else if thevie_guard.collective.global_wisdom < 0.82 {
                    debug!(
                        "[FlashScheduler] Sagesse modérée ({:.3}) → Flash non déclenché",
                        thevie_guard.collective.global_wisdom
                    );
                }
            }
        });
    }
}