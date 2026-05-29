// crates/secure/src/transport/trait.rs
// =====================================================
// Universal Transport Trait v6.1 — Gematria Flash Core
// Compatible Contact v6.2 + DID + RomanT369 + GroupManager
// SkyAInet × Nikola T369
// =====================================================

use async_trait::async_trait;
use std::net::SocketAddr;
use thiserror::Error;

use crate::crypto::hybrid::HybridMode;
use crate::contacts::contact::Contact;

/// Type de couche de transport
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportLayer {
    /// Cœur du réseau (libp2p entre serveurs)
    /// Mode par défaut : BinaryPQ + Flash Gematria occasionnel
    Core,

    /// Extrémités (WebRTC, Mobile, Navigateur)
    /// Mode forcé : Full Gematria + Stéganographie
    Edge,
}

/// Suite cryptographique utilisée
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptoSuite {
    BinaryXChaCha20Poly1305,
    Gematria95,
    HybridFlash,
    PostQuantumHybrid,
}

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Transport not started")]
    NotStarted,
    #[error("Send failed: {0}")]
    SendFailed(String),
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    #[error("Invalid hybrid mode for this layer")]
    InvalidModeForLayer,
}

#[async_trait]
pub trait Transport: Send + Sync {
    /// Envoi de données (utilise le mode courant)
    async fn send(&self, addr: SocketAddr, data: &[u8]) -> Result<(), TransportError>;

    /// Réception de données
    async fn recv(&self) -> Result<(SocketAddr, Vec<u8>), TransportError>;

    /// Démarrage du transport
    async fn start(&mut self) -> Result<(), TransportError>;

    /// Arrêt propre du transport
    async fn stop(&mut self);

    /// Adresse locale (si applicable)
    fn local_addr(&self) -> Option<SocketAddr>;

    /// Mode de chiffrement actuel
    fn crypto_mode(&self) -> CryptoSuite;

    /// Couche du transport (Core ou Edge)
    fn layer(&self) -> TransportLayer;

    // =====================================================
    // Méthodes optionnelles pour le mode hybride
    // =====================================================

    async fn set_hybrid_mode(&mut self, mode: HybridMode) -> Result<(), TransportError> {
        Err(TransportError::InvalidModeForLayer)
    }

    fn supports_flash_gematria(&self) -> bool {
        false
    }

    fn current_hybrid_mode(&self) -> Option<HybridMode> {
        None
    }
}

/// Extension pour les transports qui supportent le mode hybride
#[async_trait]
pub trait HybridTransport: Transport {
    /// Envoi avec un mode hybride spécifique
    async fn send_with_mode(
        &mut self,
        addr: SocketAddr,
        data: &[u8],
        mode: HybridMode,
        contact: Option<&Contact>,
    ) -> Result<(), TransportError>;

    /// Force le mode Flash Gematria (pour tests et gouvernance)
    async fn force_flash_gematria(&mut self) -> Result<(), TransportError>;
}