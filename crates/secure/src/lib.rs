// crates/secure/src/lib.rs
// =====================================================
// SkyAInet Secure Transport Crate — Version 6.1
// RomanT369 + KemT369 + DID + Contact + Group + Post-Quantum
// SkyAInet × Nikola T369
// =====================================================

pub mod crypto;
pub mod protocol;
pub mod transport;
pub mod identity;
pub mod defense;
pub mod suites;
pub mod roots;
pub mod device;
pub mod group;
pub mod media;
pub mod metrics;
pub mod contacts;
pub mod utils;

// =====================================================
// EXPORTS PRINCIPAUX (API publique recommandée)
// =====================================================

pub use crypto::{
    RomanT369,
    GematriaMode,
    KemT369,
    HybridTransport,
    HybridMode,
    DoubleRatchet,
};

pub use suites::{
    GematriaSuite,
    PostQuantumSuite,
};

pub use contacts::{
    Contact,
    ContactManager,
    ContactManagerError,
};

pub use group::{
    GroupManager,
    Group,
    GroupError,
};

pub use identity::{
    Did,
    DidError,
};

pub use defense::{
    CanvasBlocker,
    CanvasProtectionLevel,
    DecoyCircuitManager,
    DecoyCircuit,
};

pub use metrics::{
    RedTeamClassifier,
    CoverageMetrics,
    StealthProfile,
};

pub use roots::{
    DiamantCircuitBuilder,
    PeerPool,
    PeerReputation,
    EpochRekeyManager,
    NodeAttestation,
};

// =====================================================
// RE-EXPORTS UTILES
// =====================================================

pub use crypto::hybrid::HybridError;
pub use crypto::double_ratchet::DoubleRatchetError;
pub use protocol::handshake::{Handshake, HandshakeMessage, NodeRole, CryptoSuite};
pub use utils::{
    MarkovChain,
    Compression,
};