//! Thevie - Living Collective Intelligence
//! Core module for ethical alignment and benevolent AI behavior

pub mod Thevie;
pub mod agent;
pub mod benchmark;
pub mod flash_scheduler;
pub mod inference;
pub mod lora_evolution;
pub mod memory;
pub mod model_registry;
pub mod moe;
pub mod neural_mesh;
pub mod neurone;
pub mod router_intelligent;
pub mod synapse;
pub mod personality;
pub mod replay_buffer;
pub mod collective_consciousness;
pub mod evolution;
pub mod dream_cycle;
pub mod federated_sync;
pub mod distillation_manager;
pub mod replication_manager;
pub mod migration_manager;
pub mod persistent_storage;

// Main re-export
pub use crate::thevie::Thevie::Thevie;