// crates/secure/src/utils/mod.rs
// =====================================================
// Utils Module — SkyAInet Secure Transport
// Version 6.1
// =====================================================

pub mod time;
pub mod reproducible;
pub mod markov;
pub mod compression;

// =====================================================
// RÉ-EXPORTS PRINCIPAUX (pour un usage facile)
// =====================================================

pub use time::{
    now_timestamp,
    format_timestamp,
    elapsed_since,
    log_timestamp,
};

pub use reproducible::{
    create_reproducible_rng,
    deterministic_hash,
    generate_deterministic_bytes,
};

pub use markov::{
    MarkovChain,
    MarkovError,
};

pub use compression::{
    Compression,
    CompressionError,
};