// crates/memory/src/lib.rs
// =====================================================
// SkyAInet Memory Crate — v5.0
// Stockage Décentralisé Intelligent & Post-Quantique
// Zip Memory + IPFS + Vector Store + Hybrid Crypto
// =====================================================

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

//! # SkyAInet Memory
//!
//! Module central de stockage décentralisé du projet SkyAInet.
//! Fournit des outils performants, sécurisés et intelligents :
//! - **ZipMemory**: Compression intelligente avec cache LRU
//! - **IpfsStorage**: Client IPFS avec chiffrement hybride, retry et persistance
//! - **VectorStore**: Recherche sémantique avancée avec cache et boost qualité

pub mod zip_memory;
pub mod ipfs;
pub mod vector_store;

// =====================================================
// RÉ-EXPORTS PUBLICS
// =====================================================

pub use zip_memory::ZipMemory;
pub use ipfs::IpfsStorage;
pub use vector_store::{VectorStore, VectorEntry, VectorMetadata};

// =====================================================
// CONSTANTES & VERSION
// =====================================================

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Retourne les informations de version
pub fn version_info() -> String {
    format!("{} v{}", CRATE_NAME, VERSION)
}

// =====================================================
// FONCTIONS D'USINE (Factory)
// =====================================================

/// Crée un système de stockage complet prêt à l'emploi
pub fn create_full_storage(base_path: &str) -> (ZipMemory, IpfsStorage, VectorStore) {
    let zip = ZipMemory::new(&format!("{}/zip", base_path));
    
    let mut ipfs = IpfsStorage::new(Some("http://127.0.0.1:5001"));
    ipfs = ipfs.with_zip_memory(true);

    // Dimension standard pour les embeddings modernes (ex: text-embedding-3-large)
    let vector_store = VectorStore::new(1536);

    info!("[Memory] Full storage system initialized successfully");
    (zip, ipfs, vector_store)
}