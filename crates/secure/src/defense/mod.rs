// crates/secure/src/defense/mod.rs
// =====================================================
// Defense Module — SkyAInet Secure Transport
// Version 5.1 — Strong Edition
// Active Evasion + Deception Layer
// =====================================================

pub mod decoy_circuits;
pub mod canvas_blocker;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use decoy_circuits::{DecoyCircuitManager, DecoyCircuit};
pub use canvas_blocker::{CanvasBlocker, CanvasProtectionLevel};