// crates/secure/src/protocol/handshake.rs
// =====================================================
// Handshake Hybride v6.1 — Négociation Intelligente
// Compatible Contact v6.2 + DID + RomanT369 + GroupManager
// SkyAInet × Nikola T369
// =====================================================

use crate::crypto::hybrid::{HybridTransport, HybridMode};
use crate::crypto::kem_t369::{KemT369, KemPublicKey};
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::contacts::contact::Contact;

use blake3::Hasher;
use serde::{Serialize, Deserialize};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug, warn};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HandshakeMessage {
    pub version: u8,
    pub x25519_public: [u8; 32],
    pub ml_kem_public: Vec<u8>,
    pub is_1024: bool,
    pub node_id: [u8; 32],
    pub node_role: NodeRole,
    pub supported_suites: Vec<CryptoSuite>,
    pub preferred_hybrid_mode: HybridMode,
    pub timestamp: u64,
    pub signature: Vec<u8>,
    pub did: Option<String>,                    // ← Nouveau : DID pour identité décentralisée
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Core,
    Edge,
    Validator,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoSuite {
    KemT369,
    RomanT369,
    HybridFlash,
    PostQuantumHybrid,
}

pub struct Handshake {
    pub local_secret: EphemeralSecret,
    pub local_kem: KemT369,
    pub transcript: Hasher,
    pub local_role: NodeRole,
    pub hybrid_engine: HybridTransport,
    pub chosen_mode: Option<HybridMode>,
    pub roman: RomanT369,                       // ← Nouveau
}

impl Handshake {
    pub fn new(local_role: NodeRole) -> Self {
        let hybrid_mode = match local_role {
            NodeRole::Core => HybridMode::KemT369Core,
            NodeRole::Edge => HybridMode::FullGematria,
            NodeRole::Validator => HybridMode::KemT369Core,
        };

        Self {
            local_secret: EphemeralSecret::random_from_rng(rand::thread_rng()),
            local_kem: KemT369::new(false),
            transcript: Hasher::new(),
            local_role,
            hybrid_engine: HybridTransport::new(false),
            chosen_mode: Some(hybrid_mode),
            roman: RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256),
        }
    }

    /// Crée le message initial du handshake (avec DID optionnel)
    pub fn create_initial_message(&mut self, node_id: [u8; 32], contact: Option<&Contact>) -> HandshakeMessage {
        let x25519_public = X25519PublicKey::from(&self.local_secret).to_bytes();
        let (kem_public, _) = self.local_kem.generate_keypair();

        self.transcript.update(&x25519_public);
        self.transcript.update(&kem_public.ml_kem_public);
        self.transcript.update(&[self.local_role as u8]);

        let preferred_mode = match self.local_role {
            NodeRole::Core => HybridMode::KemT369Core,
            NodeRole::Edge => HybridMode::FullGematria,
            NodeRole::Validator => HybridMode::KemT369Core,
        };

        let did = contact.and_then(|c| c.get_did_string());

        HandshakeMessage {
            version: 0x06,
            x25519_public,
            ml_kem_public: kem_public.ml_kem_public,
            is_1024: kem_public.is_1024,
            node_id,
            node_role: self.local_role,
            supported_suites: vec![
                CryptoSuite::KemT369,
                CryptoSuite::RomanT369,
                CryptoSuite::HybridFlash,
                CryptoSuite::PostQuantumHybrid,
            ],
            preferred_hybrid_mode: preferred_mode,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signature: vec![],
            did,
        }
    }

    /// Traite la réponse et négocie le mode hybride
    pub fn process_response(
        &mut self,
        msg: &HandshakeMessage,
    ) -> Result<(HybridMode, [u8; 32]), String> {
        let chosen_mode = self.negotiate_hybrid_mode(msg.preferred_hybrid_mode);
        self.chosen_mode = Some(chosen_mode);
        self.hybrid_engine.set_mode(chosen_mode);

        info!("[Handshake] Mode hybride négocié : {:?}", chosen_mode);

        self.transcript.update(&msg.x25519_public);
        self.transcript.update(&msg.ml_kem_public);
        self.transcript.update(&[msg.node_role as u8]);

        let transcript_hash = self.transcript.finalize();

        let x25519_shared = self.local_secret.diffie_hellman(
            &X25519PublicKey::from(msg.x25519_public),
        );

        let mut shared_secret = [0u8; 32];
        shared_secret.copy_from_slice(&x25519_shared.to_bytes());

        // Dérivation finale renforcée avec RomanT369
        let final_key = self.derive_final_key(&shared_secret, &transcript_hash);

        Ok((chosen_mode, final_key))
    }

    fn negotiate_hybrid_mode(&self, remote_preferred: HybridMode) -> HybridMode {
        match (self.local_role, remote_preferred) {
            (NodeRole::Edge, _) => HybridMode::FullGematria,
            (NodeRole::Core, HybridMode::FullGematria) => HybridMode::FlashGematria,
            _ => HybridMode::KemT369Core,
        }
    }

    /// Dérivation finale avec RomanT369 (plus forte que HKDF seul)
    fn derive_final_key(&self, shared_secret: &[u8; 32], transcript: &[u8; 32]) -> [u8; 32] {
        let mut input = [0u8; 64];
        input[..32].copy_from_slice(shared_secret);
        input[32..].copy_from_slice(transcript);

        // Utilisation de RomanT369 pour la dérivation finale
        let encrypted = self.roman.encrypt(&input);
        let mut final_key = [0u8; 32];
        final_key.copy_from_slice(&encrypted[..32]);
        final_key
    }

    pub fn chosen_mode(&self) -> Option<HybridMode> {
        self.chosen_mode
    }
}