// crates/secure-transport/src/transport/mod.rs
// =====================================================
// Transport Module — Nikola T369
// =====================================================

pub mod trait;
pub mod libp2p_transport;
pub mod webrtc_transport; // Gardé pour compatibilité future

pub use trait::{Transport, TransportLayer, HybridMode};
pub use libp2p_transport::Libp2pTransportReal;