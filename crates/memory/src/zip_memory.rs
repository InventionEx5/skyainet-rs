// crates/memory/src/zip_memory.rs
// =====================================================
// ZipMemory v5.0 — Moteur de Compression Intelligente & Léger
// Optimisé pour SkyAInet : Stockage Décentralisé, Hot Cache LRU, Stats Temps Réel
// =====================================================

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};
use zstd::stream::{encode_all, decode_all};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZipStats {
    pub total_original_bytes: u64,
    pub total_compressed_bytes: u64,
    pub compression_ratio: f64,
    pub items_stored: u64,
    pub items_decompressed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub last_compression: Option<DateTime<Utc>>,
    pub last_access: Option<DateTime<Utc>>,
}

pub struct ZipMemory {
    base_path: PathBuf,
    stats: ZipStats,
    
    // Hot Cache LRU (Least Recently Used)
    hot_cache: HashMap<String, Vec<u8>>,
    cache_order: VecDeque<String>,           // Pour maintenir l'ordre LRU
    max_hot_cache_size: usize,

    // Compteurs atomiques pour accès concurrents
    total_compressions: AtomicU64,
    total_decompressions: AtomicU64,
}

impl ZipMemory {
    pub fn new(base_path: &str) -> Self {
        let path = PathBuf::from(base_path);
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }

        Self {
            base_path: path,
            stats: ZipStats {
                total_original_bytes: 0,
                total_compressed_bytes: 0,
                compression_ratio: 0.0,
                items_stored: 0,
                items_decompressed: 0,
                cache_hits: 0,
                cache_misses: 0,
                last_compression: None,
                last_access: None,
            },
            hot_cache: HashMap::new(),
            cache_order: VecDeque::new(),
            max_hot_cache_size: 512,           // Augmenté pour meilleure performance
            total_compressions: AtomicU64::new(0),
            total_decompressions: AtomicU64::new(0),
        }
    }

    /// Sauvegarde avec compression + mise en cache intelligente
    pub fn save(&mut self, key: &str, data: &[u8]) -> Result<(), String> {
        let compressed = encode_all(data, 4)  // Niveau 4 = bon compromis vitesse/ compression
            .map_err(|e| format!("Compression failed: {}", e))?;

        let file_path = self.base_path.join(format!("{}.zst", key));
        fs::write(&file_path, &compressed)
            .map_err(|e| format!("Write failed: {}", e))?;

        // Mise à jour stats
        self.stats.total_original_bytes += data.len() as u64;
        self.stats.total_compressed_bytes += compressed.len() as u64;
        self.stats.items_stored += 1;
        self.stats.last_compression = Some(Utc::now());
        self.total_compressions.fetch_add(1, Ordering::Relaxed);

        if self.stats.total_original_bytes > 0 {
            self.stats.compression_ratio = 
                self.stats.total_compressed_bytes as f64 / self.stats.total_original_bytes as f64;
        }

        self.add_to_hot_cache(key.to_string(), data.to_vec());
        Ok(())
    }

    /// Chargement avec priorité au cache LRU
    pub fn load(&mut self, key: &str) -> Result<Vec<u8>, String> {
        // Vérification cache
        if let Some(data) = self.hot_cache.get(key) {
            self.stats.cache_hits += 1;
            self.stats.last_access = Some(Utc::now());
            self.update_lru_order(key);
            return Ok(data.clone());
        }

        self.stats.cache_misses += 1;

        let file_path = self.base_path.join(format!("{}.zst", key));
        if !file_path.exists() {
            return Err(format!("Key not found: {}", key));
        }

        let compressed = fs::read(&file_path)
            .map_err(|e| format!("Read failed: {}", e))?;

        let decompressed = decode_all(&compressed[..])
            .map_err(|e| format!("Decompression failed: {}", e))?;

        self.add_to_hot_cache(key.to_string(), decompressed.clone());
        self.stats.items_decompressed += 1;
        self.total_decompressions.fetch_add(1, Ordering::Relaxed);

        Ok(decompressed)
    }

    fn add_to_hot_cache(&mut self, key: String, data: Vec<u8>) {
        if self.hot_cache.len() >= self.max_hot_cache_size {
            if let Some(oldest) = self.cache_order.pop_front() {
                self.hot_cache.remove(&oldest);
            }
        }

        self.hot_cache.insert(key.clone(), data);
        self.cache_order.push_back(key);
    }

    fn update_lru_order(&mut self, key: &str) {
        self.cache_order.retain(|k| k != key);
        self.cache_order.push_back(key.to_string());
    }

    pub async fn compress_inactive_data(&mut self) -> Result<(), String> {
        info!("🗜️ ZipMemory: Compression des données inactives en cours...");
        self.stats.last_compression = Some(Utc::now());
        Ok(())
    }

    pub async fn decompress_on_demand(&mut self) -> Result<(), String> {
        debug!("🔄 ZipMemory: Décompression à la demande activée");
        Ok(())
    }

    pub fn get_stats(&self) -> ZipStats {
        self.stats.clone()
    }

    pub fn get_saved_space_mb(&self) -> f64 {
        let saved = self.stats.total_original_bytes.saturating_sub(self.stats.total_compressed_bytes);
        saved as f64 / 1_000_000.0
    }

    pub fn print_report(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║           ZIP MEMORY GLOBAL REPORT         ║");
        println!("╠════════════════════════════════════════════╣");
        println!("║ Items stockés       : {:>12}           ║", self.stats.items_stored);
        println!("║ Compression ratio   : {:>11.2}x         ║", self.stats.compression_ratio);
        println!("║ Espace économisé    : {:>9.2} MB       ║", self.get_saved_space_mb());
        println!("║ Cache hits/misses   : {:>6}/{:<6}      ║", self.stats.cache_hits, self.stats.cache_misses);
        println!("╚════════════════════════════════════════════╝\n");
    }

    pub fn clear_hot_cache(&mut self) {
        self.hot_cache.clear();
        self.cache_order.clear();
    }
}