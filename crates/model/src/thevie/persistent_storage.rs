// crates/model/src/thevie/persistent_storage.rs
// =====================================================
// Persistent Storage v3.0 — Stockage Persistant Robuste
// Utilise sled (embeded KV) + Sérialisation JSON
// =====================================================

use sled::Db;
use std::path::Path;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug, error};

use super::synapse::Synapse;
use super::neural_mesh::Lesson;

/// Stockage persistant principal pour Thevie
pub struct PersistentStorage {
    db: Db,
}

impl PersistentStorage {
    /// Crée ou ouvre la base de données
    pub fn new(path: impl AsRef<Path>) -> Result<Self, String> {
        let db = sled::open(path.as_ref())
            .map_err(|e| format!("Impossible d'ouvrir la BDD sled : {}", e))?;

        info!("[PersistentStorage] Base de données ouverte : {:?}", path.as_ref());
        
        Ok(Self { db })
    }

    // =====================================================
    // NEURONES
    // =====================================================

    pub fn save_neuron(&self, neuron_id: u64, data: &[u8]) -> Result<(), String> {
        let key = format!("neuron:{}", neuron_id);
        self.db.insert(key.as_bytes(), data)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load_neuron(&self, neuron_id: u64) -> Result<Option<Vec<u8>>, String> {
        let key = format!("neuron:{}", neuron_id);
        self.db.get(key.as_bytes())
            .map_err(|e| e.to_string())?
            .map(|v| Some(v.to_vec()))
            .ok_or_else(|| None)
            .ok()
    }

    // =====================================================
    // SYNAPSES
    // =====================================================

    pub fn save_synapse(&self, synapse: &Synapse) -> Result<(), String> {
        let key = format!("synapse:{}:{}", synapse.from, synapse.to);
        let data = serde_json::to_vec(synapse)
            .map_err(|e| e.to_string())?;

        self.db.insert(key.as_bytes(), data)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load_synapse(&self, from: u64, to: u64) -> Result<Option<Synapse>, String> {
        let key = format!("synapse:{}:{}", from, to);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(data) => {
                let synapse: Synapse = serde_json::from_slice(&data)
                    .map_err(|e| e.to_string())?;
                Ok(Some(synapse))
            }
            None => Ok(None),
        }
    }

    // =====================================================
    // LEÇONS
    // =====================================================

    pub fn save_lesson(&self, lesson_id: &str, lesson: &Lesson) -> Result<(), String> {
        let key = format!("lesson:{}", lesson_id);
        let data = serde_json::to_vec(lesson)
            .map_err(|e| e.to_string())?;

        self.db.insert(key.as_bytes(), data)
            .map_err(|e| e.to_string())?;
        debug!("[PersistentStorage] Leçon sauvegardée : {}", lesson_id);
        Ok(())
    }

    pub fn load_lesson(&self, lesson_id: &str) -> Result<Option<Lesson>, String> {
        let key = format!("lesson:{}", lesson_id);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(data) => {
                let lesson: Lesson = serde_json::from_slice(&data)
                    .map_err(|e| e.to_string())?;
                Ok(Some(lesson))
            }
            None => Ok(None),
        }
    }

    // =====================================================
    // SNAPSHOTS & BACKUPS
    // =====================================================

    pub fn save_mesh_snapshot(&self, data: &[u8]) -> Result<(), String> {
        self.db.insert(b"mesh:snapshot", data)
            .map_err(|e| e.to_string())?;
        info!("[PersistentStorage] Snapshot du Neural Mesh sauvegardé");
        Ok(())
    }

    pub fn load_mesh_snapshot(&self) -> Result<Option<Vec<u8>>, String> {
        self.db.get(b"mesh:snapshot")
            .map_err(|e| e.to_string())?
            .map(|v| Some(v.to_vec()))
            .ok_or_else(|| None)
            .ok()
    }

    // =====================================================
    // UTILITAIRES
    // =====================================================

    pub fn flush(&self) -> Result<(), String> {
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<(), String> {
        self.db.clear().map_err(|e| e.to_string())?;
        self.flush()?;
        warn!("[PersistentStorage] Base de données entièrement vidée !");
        Ok(())
    }

    pub fn get_stats(&self) -> Result<String, String> {
        let len = self.db.len();
        let size = self.db.size_on_disk().map_err(|e| e.to_string())?;
        Ok(format!("PersistentStorage → {} entrées | {:.2} MB", len, size as f64 / 1_048_576.0))
    }
}

impl Drop for PersistentStorage {
    fn drop(&mut self) {
        let _ = self.flush();
        debug!("[PersistentStorage] Base de données fermée proprement");
    }
}