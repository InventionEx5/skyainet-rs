// crates/secure/src/utils/time.rs
// =====================================================
// Time Utilities v6.1 — Fonctions Temporelles Simples
// Compatible avec tout le projet (Contact, DID, Groupes)
// SkyAInet × Nikola T369
// =====================================================

use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

/// Retourne le timestamp Unix actuel en secondes
#[inline]
pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Convertit un timestamp en chaîne de caractères
#[inline]
pub fn format_timestamp(ts: u64) -> String {
    ts.to_string()
}

/// Calcule le temps écoulé depuis un timestamp (en secondes)
#[inline]
pub fn elapsed_since(timestamp: u64) -> u64 {
    now_timestamp().saturating_sub(timestamp)
}

/// Affiche un timestamp de manière lisible (pour les logs)
pub fn log_timestamp(ts: u64) {
    debug!("[Time] Timestamp: {} ({} secondes)", ts, elapsed_since(ts));
}