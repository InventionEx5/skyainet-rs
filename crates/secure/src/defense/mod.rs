// crates/secure/src/defense/mod.rs
// =====================================================
// Defense Module — SkyAInet Secure Transport
// Version 5.2
// =====================================================

pub mod canvas_blocker;
pub mod decoy_circuits;
pub mod anti_debug;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use canvas_blocker::{
    CanvasBlocker,
    CanvasProtectionLevel,
    CanvasError,
};

pub use decoy_circuits::{
    DecoyCircuitManager,
    DecoyCircuit,
};

pub use anti_debug::{
    AntiDebug,
    AntiDebugError,
};