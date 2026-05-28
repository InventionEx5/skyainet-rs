// crates/core/src/lib.rs
// =====================================================
// SkyAInet Core Crate
// Types fondamentaux et alignement éthique
// =====================================================

pub mod types;
pub mod error;
pub mod traits;
pub mod economics;
pub mod rewards;
pub mod node_types;
pub mod alignment_kernel;
pub mod constitution;

pub use types::*;
pub use error::SkyAInetError;
pub use traits::EthicalAgent;
pub use economics::*;
pub use rewards::*;
pub use node_types::*;
pub use alignment_kernel::*;
pub use constitution::*;