// crates/model/src/thevie/persistent_storage.rs
// =====================================================
// Persistent Storage
// Stockage Persistant  Neural Mesh + Le�ons (sled + IPFS)
// =====================================================

use sled::Db;
use std::path::Path;
use tracing::{info, warn, error};

use super::synapse::Synapse;

pub struct PersistentStorage {
    db: Db,
}

impl PersistentStorage {
    pub fn new(path: &str) -> Result<Self, String> {
        let db = sled::open(path).map_err(|e| e.to_string())?;
        info!("[PersistentStorage] Base de donn�es ouverte : {}", path);
        Ok(Self { db })
    }

    /// Sauvegarde un neurone
    pub fn save_neuron(&self, neuron_id: u64, data: &[u8]) -> Result<(), String> {
        let key = format!("neuron:{}", neuron_id);
        self.db.insert(key.as_bytes(), data).map_err(|e| e.to_string())?;
        self.db.flush().ok();
        Ok(())
    }

    /// Charge un neurone
    pub fn load_neuron(&self, neuron_id: u64) -> Result<Option<Vec<u8>>, String> {
        let key = format!("neuron:{}", neuron_id);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(ivec) => Ok(Some(ivec.to_vec())),
            None => Ok(None),
        }
    }

    /// Supprime un neurone
    pub fn delete_neuron(&self, neuron_id: u64) -> Result<(), String> {
        let key = format!("neuron:{}", neuron_id);
        self.db.remove(key.as_bytes()).map_err(|e| e.to_string())?;
        self.db.flush().ok();
        Ok(())
    }

    /// Sauvegarde une synapse
    pub fn save_synapse(&self, from: u64, to: u64, data: &[u8]) -> Result<(), String> {
        let key = format!("synapse:{}:{}", from, to);
        self.db.insert(key.as_bytes(), data).map_err(|e| e.to_string())?;
        self.db.flush().ok();
        Ok(())
    }

    /// Charge toutes les synapses
    pub fn load_all_synapses(&self) -> Result<Vec<(String, Vec<u8>)>, String> {
        let mut results = Vec::new();
        for item in self.db.iter() {
            let (key, value) = item.map_err(|e| e.to_string())?;
            let key_str = String::from_utf8_lossy(&key).to_string();
            if key_str.starts_with("synapse:") {
                results.push((key_str, value.to_vec()));
            }
        }
        Ok(results)
    }

    /// Sauvegarde une le�on (haute qualit�)
    pub async fn save_lesson(&self, lesson_id: &str, data: &[u8]) -> Result<String, String> {
        let key = format!("lesson:{}", lesson_id);
        self.db.insert(key.as_bytes(), data).map_err(|e| e.to_string())?;
        Ok(format!("local:{}", lesson_id))
    }

    /// Charge une le�on
    pub fn load_lesson(&self, lesson_id: &str) -> Result<Option<Vec<u8>>, String> {
        let key = format!("lesson:{}", lesson_id);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(ivec) => Ok(Some(ivec.to_vec())),
            None => Ok(None),
        }
    }

    /// Sauvegarde l'�tat complet du mesh (backup)
    pub fn save_mesh_snapshot(&self, data: &[u8]) -> Result<(), String> {
        let key = "mesh:snapshot";
        self.db.insert(key.as_bytes(), data).map_err(|e| e.to_string())?;
        self.db.flush().ok();
        info!("[PersistentStorage] Snapshot du mesh sauvegard�");
        Ok(())
    }

    /// Charge le dernier snapshot du mesh
    pub fn load_mesh_snapshot(&self) -> Result<Option<Vec<u8>>, String> {
        let key = "mesh:snapshot";
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(ivec) => Ok(Some(ivec.to_vec())),
            None => Ok(None),
        }
    }

    /// Liste toutes les cl�s (debug)
    pub fn list_all_keys(&self) -> Result<Vec<String>, String> {
        let mut keys = Vec::new();
        for item in self.db.iter() {
            let (key, _) = item.map_err(|e| e.to_string())?;
            keys.push(String::from_utf8_lossy(&key).to_string());
        }
        Ok(keys)
    }

    /// Vide la base de donn�es (danger !)
    pub fn clear_all(&self) -> Result<(), String> {
        self.db.clear().map_err(|e| e.to_string())?;
        self.db.flush().ok();
        warn!("[PersistentStorage] Base de donn�es vid�e !");
        Ok(())
    }
}
    // =====================================================
    // Méthodes pour Synapse (nouveau type)
    // =====================================================

    /// Sauvegarde une synapse
    pub fn save_synapse(&self, synapse: &Synapse) -> Result<(), String> {
        let key = format!("synapse:{}:{}", synapse.from, synapse.to);
        let data = serde_json::to_vec(synapse)
            .map_err(|e| e.to_string())?;
        self.db.insert(key.as_bytes(), data).map_err(|e| e.to_string())?;
        self.db.flush().ok();
        Ok(())
    }

    /// Charge une synapse
    pub fn load_synapse(&self, from: u64, to: u64) -> Result<Option<Synapse>, String> {
        let key = format!("synapse:{}:{}", from, to);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(ivec) => {
                let synapse: Synapse = serde_json::from_slice(&ivec)
                    .map_err(|e| e.to_string())?;
                Ok(Some(synapse))
            }
            None => Ok(None),
        }
    }
}
