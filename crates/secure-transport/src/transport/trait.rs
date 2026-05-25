// crates/secure-transport/src/transport/trait.rs
// =====================================================
// Universal Transport Trait v3.0 — Gematria Flash Core
// SkyAInet × Nikola T369
// =====================================================

use async_trait::async_trait;
use std::net::SocketAddr;
use thiserror::Error;

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

/// Modes de chiffrement supportés par le système hybride
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HybridMode {
    /// Mode par défaut du CŒUR (95% du trafic)
    /// XChaCha20-Poly1305 + T369Kem (très rapide + post-quantique)
    BinaryPQ,

    /// Flash Gematria (5% du trafic dans le cœur)
    /// Petit paquet Gematria sur les métadonnées + headers
    FlashGematria,

    /// Mode complet pour les EXTRÉMITÉS
    /// Gematria Dynamic + Stéganographie Markov (plein régime)
    FullGematria,
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

/// Suite cryptographique utilisée
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptoSuite {
    BinaryXChaCha20Poly1305,
    Gematria95,
    HybridFlash,           // Nouveau : mode Flash Gematria
    PostQuantumHybrid,     // XChaCha20 + T369Kem
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
    // Méthodes optionnelles pour le mode hybride (Gematria Flash Core)
    // =====================================================

    /// Change le mode hybride (disponible sur les transports qui le supportent)
    async fn set_hybrid_mode(&mut self, mode: HybridMode) -> Result<(), TransportError> {
        // Par défaut : pas de support (implémenté seulement sur les transports concernés)
        Err(TransportError::InvalidModeForLayer)
    }

    /// Vérifie si ce transport supporte le mode Flash Gematria
    fn supports_flash_gematria(&self) -> bool {
        false
    }

    /// Retourne le mode hybride actuel (si applicable)
    fn current_hybrid_mode(&self) -> Option<HybridMode> {
        None
    }
}

/// Extension pour les transports qui supportent le mode hybride
#[async_trait]
pub trait HybridTransport: Transport {
    /// Envoi avec un mode hybride spécifique (plus flexible)
    async fn send_with_mode(
        &mut self,
        addr: SocketAddr,
        data: &[u8],
        mode: HybridMode,
    ) -> Result<(), TransportError>;

    /// Force le mode Flash Gematria (utile pour les tests et la gouvernance)
    async fn force_flash_gematria(&mut self) -> Result<(), TransportError>;
}