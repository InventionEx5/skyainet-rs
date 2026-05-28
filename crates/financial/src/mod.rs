// crates/financial/src/mod.rs
// =====================================================
// Financial Module — Déclaration et ré-exports
// Treasury + Liquidity Management
// =====================================================

pub mod treasury;
pub mod liquidity;

// =====================================================
// RÉ-EXPORTS PUBLICS
// =====================================================

pub use treasury::TreasuryManager;
pub use liquidity::LiquidityManager;

// =====================================================
// VERSION DU MODULE
// =====================================================

pub const MODULE_VERSION: &str = "5.0.0";