// crates/secure/src/crypto/double_ratchet.rs
// =====================================================
// Double Ratchet v3.0 — Version Finale Production
// Thevie × Nikola T369 — RomanT369 (Hyper256) + Post-Quantum Ready
// Compatible avec GematriaAead + KemT369
// =====================================================

use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use hkdf::Hkdf;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use std::collections::HashMap;
use thiserror::Error;

const MAX_SKIP: u32 = 1000;

#[derive(Error, Debug)]
pub enum DoubleRatchetError {
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Too many skipped messages")]
    TooManySkippedMessages,
    #[error("Invalid ratchet key")]
    InvalidRatchetKey,
}

pub struct DoubleRatchet {
    root_key: [u8; 32],
    send_chain_key: [u8; 32],
    recv_chain_key: [u8; 32],
    send_ratchet_key: EphemeralSecret,
    recv_ratchet_key: Option<X25519PublicKey>,
    send_message_number: u32,
    recv_message_number: u32,
    skipped_keys: HashMap<(u32, u32), [u8; 32]>,
}

impl DoubleRatchet {
    pub fn new(root_key: [u8; 32], send_ratchet_key: EphemeralSecret) -> Self {
        Self {
            root_key,
            send_chain_key: root_key,
            recv_chain_key: [0u8; 32],
            send_ratchet_key,
            recv_ratchet_key: None,
            send_message_number: 0,
            recv_message_number: 0,
            skipped_keys: HashMap::new(),
        }
    }

    /// Dérive une clé de message et avance la chaîne symétrique
    fn derive_message_key(chain_key: &mut [u8; 32]) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(None, chain_key);
        let mut message_key = [0u8; 32];
        hk.expand(b"T369-DR-MSG-KEY", &mut message_key).unwrap();

        // Avance la chaîne
        hk.expand(b"T369-DR-CHAIN", chain_key).unwrap();
        message_key
    }

    /// Chiffrement avec RomanT369 (Hyper256)
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let message_key = Self::derive_message_key(&mut self.send_chain_key);
        let nonce = self.derive_nonce();

        let roman = RomanT369::new(message_key, nonce, GematriaMode::Hyper256);
        let ciphertext = roman.encrypt(plaintext);

        self.send_message_number += 1;
        ciphertext
    }

    /// Déchiffrement avec gestion des messages en retard
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, DoubleRatchetError> {
        // Vérifie d'abord les clés sautées
        if let Some(key) = self.skipped_keys.remove(&(self.recv_message_number, 0)) {
            let roman = RomanT369::new(key, self.derive_nonce(), GematriaMode::Hyper256);
            return roman.decrypt(ciphertext).ok_or(DoubleRatchetError::DecryptionFailed);
        }

        if self.recv_chain_key == [0u8; 32] {
            return Err(DoubleRatchetError::DecryptionFailed);
        }

        let message_key = Self::derive_message_key(&mut self.recv_chain_key);
        let roman = RomanT369::new(message_key, self.derive_nonce(), GematriaMode::Hyper256);

        let plaintext = roman.decrypt(ciphertext).ok_or(DoubleRatchetError::DecryptionFailed)?;
        self.recv_message_number += 1;
        Ok(plaintext)
    }

    /// Nonce déterministe et unique par message (amélioré)
    fn derive_nonce(&self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        let mut hasher = Sha256::new();
        hasher.update(&self.send_message_number.to_le_bytes());
        hasher.update(&self.recv_message_number.to_le_bytes());
        hasher.update(&self.root_key[..8]);
        let hash = hasher.finalize();
        nonce.copy_from_slice(&hash[..12]);
        nonce
    }

    /// Ratchet de racine (changement de direction)
    pub fn ratchet(&mut self, their_ratchet_public: X25519PublicKey) -> Result<(), DoubleRatchetError> {
        if self.recv_ratchet_key.is_some() {
            return Err(DoubleRatchetError::InvalidRatchetKey);
        }

        let shared_secret = self.send_ratchet_key.diffie_hellman(&their_ratchet_public);

        let mut transcript = Vec::new();
        transcript.extend_from_slice(shared_secret.as_bytes());
        transcript.extend_from_slice(&self.root_key);

        let hk = Hkdf::<Sha256>::new(Some(b"T369-DR-ROOT"), &transcript);
        hk.expand(b"root", &mut self.root_key).unwrap();
        hk.expand(b"send-chain", &mut self.send_chain_key).unwrap();
        hk.expand(b"recv-chain", &mut self.recv_chain_key).unwrap();

        self.recv_ratchet_key = Some(their_ratchet_public);
        self.send_ratchet_key = EphemeralSecret::random_from_rng(rand::rngs::OsRng);
        self.send_message_number = 0;
        self.recv_message_number = 0;

        Ok(())
    }

    /// Permet de sauter des messages (pour les messages en retard)
    pub fn skip_message_keys(&mut self, until: u32) -> Result<(), DoubleRatchetError> {
        if until.saturating_sub(self.recv_message_number) > MAX_SKIP {
            return Err(DoubleRatchetError::TooManySkippedMessages);
        }

        while self.recv_message_number < until {
            let key = Self::derive_message_key(&mut self.recv_chain_key);
            self.skipped_keys.insert((self.recv_message_number, 0), key);
            self.recv_message_number += 1;
        }
        Ok(())
    }
}