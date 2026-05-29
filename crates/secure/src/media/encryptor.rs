// crates/secure/src/media/encryptor.rs
// =====================================================
// Media Encryptor v6.1 — Chiffrement Temps Réel (SRTP-like)
// Compatible Contact v6.2 + GroupManager v6.3 + DID
// SkyAInet × Nikola T369
// =====================================================

use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;
use crate::crypto::gematria_aead::GematriaAead;
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFrame {
    pub sequence_number: u32,
    pub timestamp: u64,
    pub payload_type: u8,
    pub ssrc: u32,
    pub contact_id: Option<[u8; 32]>,   // Pour traçabilité par contact
}

pub struct MediaEncryptor {
    gematria: GematriaAead,
    sequence_counter: u32,
    ssrc: u32,
    last_timestamp: u64,
    roman: RomanT369,
}

impl MediaEncryptor {
    /// Crée un encryptor média à partir d'une clé de contact ou de groupe
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self {
            gematria: GematriaAead::new(key, nonce),
            sequence_counter: 0,
            ssrc: rand::random::<u32>(),
            last_timestamp: Self::current_timestamp(),
            roman: RomanT369::new(key, nonce, GematriaMode::Hyper256),
        }
    }

    /// Chiffre une frame média (version contact ou groupe)
    pub fn encrypt_frame(
        &mut self,
        payload: &[u8],
        contact_id: Option<[u8; 32]>,
    ) -> (MediaFrame, Vec<u8>) {
        let now = Self::current_timestamp();

        let frame = MediaFrame {
            sequence_number: self.sequence_counter,
            timestamp: now,
            payload_type: 96,
            ssrc: self.ssrc,
            contact_id,
        };

        // En-tête SRTP-like + payload
        let mut packet = Vec::with_capacity(20 + payload.len());
        packet.extend_from_slice(&frame.sequence_number.to_le_bytes());
        packet.extend_from_slice(&frame.timestamp.to_le_bytes());
        packet.extend_from_slice(&frame.ssrc.to_le_bytes());
        if let Some(id) = contact_id {
            packet.extend_from_slice(&id);
        }
        packet.extend_from_slice(payload);

        let encrypted = self.gematria.encrypt(&packet);

        self.sequence_counter = self.sequence_counter.wrapping_add(1);
        self.last_timestamp = now;

        debug!(
            "[MediaEncryptor] Frame chiffrée | Seq: {} | Contact: {:?}",
            frame.sequence_number, contact_id.map(|id| hex::encode(&id[0..4]))
        );

        (frame, encrypted)
    }

    /// Déchiffre une frame média
    pub fn decrypt_frame(&self, encrypted: &[u8]) -> Option<(MediaFrame, Vec<u8>)> {
        let decrypted = self.gematria.decrypt(encrypted)?;

        if decrypted.len() < 16 {
            warn!("[MediaEncryptor] Frame trop courte");
            return None;
        }

        let sequence_number = u32::from_le_bytes(decrypted[0..4].try_into().unwrap());
        let timestamp = u64::from_le_bytes(decrypted[4..12].try_into().unwrap());
        let ssrc = u32::from_le_bytes(decrypted[12..16].try_into().unwrap());

        let (contact_id, payload_start) = if decrypted.len() >= 48 {
            let id: [u8; 32] = decrypted[16..48].try_into().unwrap();
            (Some(id), 48)
        } else {
            (None, 16)
        };

        let payload = decrypted[payload_start..].to_vec();

        let frame = MediaFrame {
            sequence_number,
            timestamp,
            payload_type: 96,
            ssrc,
            contact_id,
        };

        debug!("[MediaEncryptor] Frame déchiffrée | Seq: {}", sequence_number);
        Some((frame, payload))
    }

    /// Vérifie l'ordre (anti-replay)
    pub fn is_in_order(&self, sequence_number: u32) -> bool {
        let diff = sequence_number.wrapping_sub(self.sequence_counter);
        diff < 100 || diff > u32::MAX - 100
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Default for MediaEncryptor {
    fn default() -> Self {
        let key = [0x42u8; 32];
        let nonce = [0u8; 12];
        Self::new(key, nonce)
    }
}