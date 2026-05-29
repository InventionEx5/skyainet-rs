// crates/secure/src/metrics/mod.rs
// =====================================================
// Metrics Module — SkyAInet Secure Transport
// Version 6.1
// =====================================================

pub mod red_team;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX
// =====================================================

pub use red_team::{
    RedTeamClassifier,
    CoverageMetrics,
    RedTeamReport,
    StealthProfile,
};