// crates/secure/src/roots/epoch_rekey.rs
// =====================================================
// EpochRekeyManager v6.1 — Rotation Sécurisée des Clés
// Compatible Contact v6.2 + DID + GroupManager + RomanT369
// DiamantRoots v2 — Post-Quantique + Double Ratchet
// SkyAInet × Nikola T369
// =====================================================

use crate::crypto::double_ratchet::{DoubleRatchet, EphemeralSecret};
use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::contacts::contact::Contact;
use crate::contacts::manager::ContactManager;

use hkdf::Hkdf;
use sha2::Sha256;
use tracing::{info, debug, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RekeyError {
    #[error("Rekeying failed: {0}")]
    RekeyFailed(String),
    #[error("Invalid circuit state")]
    InvalidCircuit,
    #[error("Contact not verified for rekey")]
    ContactNotVerified,
}

pub struct EpochRekeyManager {
    pub current_epoch: u64,
    pub rekey_interval: u64,           // en secondes
    pub last_rekey: u64,
    pub force_rekey_on_next: bool,
    roman: RomanT369,
}

impl EpochRekeyManager {
    pub fn new(rekey_interval: u64) -> Self {
        Self {
            current_epoch: 0,
            rekey_interval,
            last_rekey: Self::now(),
            force_rekey_on_next: false,
            roman: RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256),
        }
    }

    #[inline]
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Vérifie si un rekey est nécessaire
    pub fn should_rekey(&self) -> bool {
        if self.force_rekey_on_next {
            return true;
        }
        let now = Self::now();
        now - self.last_rekey > self.rekey_interval
    }

    /// Effectue la rotation des clés via Double Ratchet + RomanT369
    pub fn perform_rekey(
        &mut self,
        shared_secrets: &mut [[u8; 32]],
        contact: Option<&Contact>,
    ) -> Result<u64, RekeyError> {
        if shared_secrets.is_empty() {
            return Err(RekeyError::InvalidCircuit);
        }

        // Vérification DID si un contact est fourni
        if let Some(c) = contact {
            if !c.has_decentralized_identity() || c.verification_level < 2 {
                return Err(RekeyError::ContactNotVerified);
            }
        }

        let new_epoch = self.current_epoch + 1;

        for (i, secret) in shared_secrets.iter_mut().enumerate() {
            // Double Ratchet pour la rotation
            let ephemeral = EphemeralSecret::random_from_rng(rand_core::OsRng);
            let mut ratchet = DoubleRatchet::new(*secret, ephemeral);

            let new_key = ratchet.encrypt(b"epoch_rekey_v6");

            if new_key.len() >= 32 {
                // Renforcement final avec RomanT369
                let reinforced = self.roman.encrypt(&new_key[0..32]);
                *secret = reinforced[0..32].try_into().unwrap();
            }

            debug!("[EpochRekey] Clé {} rekeyée → Epoch {}", i, new_epoch);
        }

        self.current_epoch = new_epoch;
        self.last_rekey = Self::now();
        self.force_rekey_on_next = false;

        info!("[EpochRekey] Rekey terminé avec succès → Nouvel epoch: {}", new_epoch);
        Ok(new_epoch)
    }

    /// Force un rekey au prochain appel
    pub fn force_rekey(&mut self) {
        self.force_rekey_on_next = true;
        warn!("[EpochRekey] Rekey forcé pour le prochain cycle");
    }

    /// Temps restant avant le prochain rekey (en secondes)
    pub fn time_until_next_rekey(&self) -> u64 {
        let now = Self::now();
        let elapsed = now.saturating_sub(self.last_rekey);
        self.rekey_interval.saturating_sub(elapsed)
    }
}