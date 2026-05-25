// crates/core/src/mod.rs
// =====================================================
// SkyAInet Core Module
// Shared core types and logic
// =====================================================

pub mod economics;
pub mod rewards;
pub mod node_types;
pub mod alignment_kernel;
pub mod constitution;

pub use economics::*;
pub use rewards::*;
pub use node_types::*;
pub use alignment_kernel::*;
pub use constitution::*;