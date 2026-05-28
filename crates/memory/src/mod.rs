// crates/memory/src/mod.rs
// =====================================================
// SkyAInet Memory Module — Déclaration des sous-modules
// =====================================================

pub mod zip_memory;
pub mod ipfs;
pub mod vector_store;

// =====================================================
// RÉ-EXPORTS PUBLICS (pour simplicité d'utilisation)
// =====================================================

pub use zip_memory::ZipMemory;
pub use ipfs::IpfsStorage;
pub use vector_store::{VectorStore, VectorEntry, VectorMetadata};

// =====================================================
// Version du module
// =====================================================

pub const MODULE_VERSION: &str = "5.0.0";