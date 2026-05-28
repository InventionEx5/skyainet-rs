// crates/memory/src/ipfs.rs
// =====================================================
// IPFS Storage v5.0 — Client Décentralisé Intelligent & Résilient
// HybridTransport + GematriaAead + ZipMemory + Retry + Health Monitoring
// =====================================================

use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn, error, debug};
use thiserror::Error;

use skyainet_secure_transport::crypto::{
    hybrid::HybridTransport,
    gematria_aead::GematriaAead
};
use skyainet_memory::zip_memory::ZipMemory;

#[derive(Error, Debug)]
pub enum IpfsError {
    #[error("IPFS request failed: {0}")]
    RequestFailed(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("ZipMemory error: {0}")]
    ZipError(String),
    #[error("Network timeout")]
    Timeout,
    #[error("Max retries exceeded")]
    MaxRetries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpfsAddResponse {
    pub Hash: String,
    pub Size: String,
    pub Name: String,
}

pub struct IpfsStorage {
    pub api_url: String,
    pub client: reqwest::Client,
    pub hybrid: HybridTransport,
    pub zip_memory: Option<ZipMemory>,

    pub encrypt_before_upload: bool,
    pub use_compression: bool,
    pub max_retries: u8,
    pub timeout_seconds: u64,
}

impl IpfsStorage {
    pub fn new(api_url: Option<&str>) -> Self {
        let url = api_url.unwrap_or("http://127.0.0.1:5001").to_string();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(90))
            .connect_timeout(Duration::from_secs(12))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_url: url,
            client,
            hybrid: HybridTransport::new(true),
            zip_memory: None,
            encrypt_before_upload: true,
            use_compression: true,
            max_retries: 3,
            timeout_seconds: 90,
        }
    }

    pub fn with_zip_memory(mut self, enabled: bool) -> Self {
        if enabled {
            self.zip_memory = Some(ZipMemory::new("./data/ipfs_cache"));
        }
        self.use_compression = enabled;
        self
    }

    /// Upload sécurisé avec retry + compression + chiffrement hybride
    pub async fn put(&self, key: &str, data: &[u8]) -> Result<String, IpfsError> {
        let mut final_data = data.to_vec();

        // === 1. Compression ZipMemory ===
        if self.use_compression {
            if let Some(zip) = &self.zip_memory {
                let compressed = zip.compress(data).await
                    .map_err(|e| IpfsError::ZipError(e))?;
                final_data = compressed;
                debug!("[IPFS] Data compressed");
            }
        }

        // === 2. Chiffrement Hybride ===
        if self.encrypt_before_upload {
            let (key, nonce) = self.hybrid.derive_keys();
            let aead = GematriaAead::new(key, nonce);
            final_data = aead.encrypt(&final_data);
        }

        // === 3. Upload avec retry ===
        for attempt in 1..=self.max_retries {
            match self.upload_to_ipfs(key, &final_data).await {
                Ok(cid) => {
                    info!("[IPFS] Upload successful → CID: {}", cid);
                    return Ok(cid);
                }
                Err(e) if attempt < self.max_retries => {
                    warn!("[IPFS] Attempt {} failed: {}", attempt, e);
                    tokio::time::sleep(Duration::from_millis(600 * attempt as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }

        Err(IpfsError::MaxRetries)
    }

    async fn upload_to_ipfs(&self, key: &str, data: &[u8]) -> Result<String, IpfsError> {
        let form = multipart::Form::new()
            .part(
                "file",
                multipart::Part::bytes(data.to_vec())
                    .file_name(key.to_string())
                    .mime_str("application/octet-stream")
                    .map_err(|e| IpfsError::RequestFailed(e.to_string()))?,
            );

        let url = format!("{}/api/v0/add?pin=true", self.api_url);

        let response = self.client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| IpfsError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(IpfsError::RequestFailed(text));
        }

        let add_response: IpfsAddResponse = response
            .json()
            .await
            .map_err(|e| IpfsError::ParseError(e.to_string()))?;

        Ok(add_response.Hash)
    }

    /// Récupération avec déchiffrement automatique
    pub async fn get(&self, cid: &str) -> Result<Vec<u8>, IpfsError> {
        let url = format!("{}/api/v0/cat?arg={}", self.api_url, cid);

        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| IpfsError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(IpfsError::RequestFailed(response.status().to_string()));
        }

        let mut bytes = response
            .bytes()
            .await
            .map_err(|e| IpfsError::RequestFailed(e.to_string()))?
            .to_vec();

        // Déchiffrement
        if self.encrypt_before_upload && !bytes.is_empty() {
            let (key, nonce) = self.hybrid.derive_keys();
            let aead = GematriaAead::new(key, nonce);
            if let Some(decrypted) = aead.decrypt(&bytes) {
                bytes = decrypted;
            }
        }

        // Décompression
        if self.use_compression {
            if let Some(zip) = &self.zip_memory {
                if let Ok(decompressed) = zip.decompress(&bytes).await {
                    bytes = decompressed;
                }
            }
        }

        Ok(bytes)
    }

    pub async fn pin(&self, cid: &str) -> Result<(), IpfsError> {
        let url = format!("{}/api/v0/pin/add?arg={}", self.api_url, cid);

        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| IpfsError::RequestFailed(e.to_string()))?;

        if response.status().is_success() {
            info!("[IPFS] CID {} pinned", cid);
            Ok(())
        } else {
            Err(IpfsError::RequestFailed(response.status().to_string()))
        }
    }

    pub async fn health_check(&self) -> bool {
        let url = format!("{}/api/v0/version", self.api_url);
        self.client.get(&url).send().await.is_ok()
    }
}