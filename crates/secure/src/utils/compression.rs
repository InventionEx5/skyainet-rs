// crates/secure/src/utils/compression.rs
// =====================================================
// Compression Utilities v6.1 — Zstd Haute Performance
// Compatible Contact v6.2 + DID + GroupManager
// SkyAInet × Nikola T369
// =====================================================

use std::io::{Read, Write};
use zstd::stream::{Decoder, Encoder};
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Data too small to decompress")]
    DataTooSmall,
}

pub struct Compression {
    pub level: i32,           // Niveau de compression (1 = rapide, 22 = maximum)
}

impl Compression {
    pub fn new(level: i32) -> Self {
        Self { level: level.clamp(1, 22) }
    }

    /// Compresse des données avec Zstd
    #[inline]
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        let mut encoder = Encoder::new(Vec::new(), self.level)
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;

        encoder.write_all(data)
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;

        let compressed = encoder.finish()
            .map_err(|e| CompressionError::CompressionFailed(e.to_string()))?;

        debug!(
            "[Compression] Données compressées : {} → {} octets ({:.1}% gain)",
            data.len(),
            compressed.len(),
            (1.0 - (compressed.len() as f64 / data.len() as f64)) * 100.0
        );

        Ok(compressed)
    }

    /// Décompresse des données avec Zstd
    #[inline]
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        if data.len() < 4 {
            return Err(CompressionError::DataTooSmall);
        }

        let mut decoder = Decoder::new(data)
            .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))?;

        debug!(
            "[Compression] Données décompressées : {} → {} octets",
            data.len(), decompressed.len()
        );

        Ok(decompressed)
    }
}

impl Default for Compression {
    fn default() -> Self {
        Self::new(3) // Niveau équilibré (rapide + bon ratio)
    }
}