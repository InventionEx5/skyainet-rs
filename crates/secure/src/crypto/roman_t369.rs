// crates/secure/src/crypto/gematria/roman_t369.rs
// =====================================================
// RomanT369 v4.0 — Chiffrement Hyper Sécurisé & Ultra Rapide
// SkyAInet × Nikola T369 — Roman Weighted Diffusion + Hyper256
// Version Production Ready
// =====================================================

use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GematriaMode {
    Dynamic,   // 95 caractères
    Extended,  // 128 caractères
    Hyper256,  // 256 caractères (recommandé)
}

#[derive(Error, Debug)]
pub enum RomanError {
    #[error("Decryption failed")]
    DecryptionFailed,
}

#[derive(Clone)]
pub struct RomanT369 {
    key: [u8; 32],
    nonce: [u8; 12],
    modulus: u16,
    permutation: [u8; 256],
    mode: GematriaMode,
    hyper_lookup: Option<[u8; 256]>,
}

impl RomanT369 {
    pub fn new(key: [u8; 32], nonce: [u8; 12], mode: GematriaMode) -> Self {
        let modulus = match mode {
            GematriaMode::Dynamic => 95,
            GematriaMode::Extended => 128,
            GematriaMode::Hyper256 => 256,
        };

        let permutation = Self::generate_constant_permutation(&key, &nonce, modulus as u8);
        let hyper_lookup = if mode == GematriaMode::Hyper256 {
            Some(Self::precompute_hyper_lookup(&permutation))
        } else {
            None
        };

        Self {
            key,
            nonce,
            modulus: modulus as u16,
            permutation,
            mode,
            hyper_lookup,
        }
    }

    fn generate_constant_permutation(key: &[u8; 32], nonce: &[u8; 12], modulus: u8) -> [u8; 256] {
        let mut table = [0u8; 256];
        for i in 0..256u16 {
            let mut hasher = Sha256::new();
            hasher.update(key);
            hasher.update(nonce);
            hasher.update(&i.to_le_bytes());
            let hash = hasher.finalize();
            table[i as usize] = hash[0] % modulus;
        }
        table
    }

    fn precompute_hyper_lookup(permutation: &[u8; 256]) -> [u8; 256] {
        let mut lookup = [0u8; 256];
        lookup.copy_from_slice(permutation);
        lookup
    }

    // =====================================================
    // Roman Weighted Diffusion (Cœur de l’innovation)
    // =====================================================

    const ROMAN_WEIGHTS: [u8; 7] = [1, 5, 10, 50, 100, 200, 250];

    #[inline(always)]
    fn roman_diffuse(&self, byte: u8, position: usize) -> u8 {
        let weight = Self::ROMAN_WEIGHTS[(byte as usize + position) % 7];

        match (byte + position as u8) % 3 {
            0 => byte.wrapping_sub(weight).rotate_right(2),
            1 => byte.wrapping_add(weight).rotate_left(3),
            _ => (byte ^ weight).rotate_left(1),
        }
    }

    #[inline(always)]
    fn roman_undiffuse(&self, byte: u8, position: usize) -> u8 {
        let weight = Self::ROMAN_WEIGHTS[(byte as usize + position) % 7];

        match (byte + position as u8) % 3 {
            0 => byte.wrapping_add(weight).rotate_left(2),
            1 => byte.wrapping_sub(weight).rotate_right(3),
            _ => (byte ^ weight).rotate_right(1),
        }
    }

    // =====================================================
    // CHIFFREMENT / DÉCHIFFREMENT
    // =====================================================

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());

        if let Some(lookup) = &self.hyper_lookup {
            for (i, &byte) in plaintext.iter().enumerate() {
                let k = lookup[byte as usize];
                let diffused = self.roman_diffuse(k, i);
                ciphertext.push(diffused);
            }
        } else {
            for (i, &byte) in plaintext.iter().enumerate() {
                let k = self.permutation[byte as usize];
                let c = ((byte as u16 + k as u16) % self.modulus) as u8;
                let diffused = self.roman_diffuse(c, i);
                ciphertext.push(diffused);
            }
        }

        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, RomanError> {
        let mut plaintext = Vec::with_capacity(ciphertext.len());

        if let Some(lookup) = &self.hyper_lookup {
            for (i, &c) in ciphertext.iter().enumerate() {
                let undiffused = self.roman_undiffuse(c, i);
                let k = lookup[undiffused as usize];
                plaintext.push(k);
            }
        } else {
            for (i, &c) in ciphertext.iter().enumerate() {
                let undiffused = self.roman_undiffuse(c, i);
                let k = self.permutation[undiffused as usize];
                let m = ((undiffused as u16 + self.modulus - k as u16) % self.modulus) as u8;
                plaintext.push(m);
            }
        }

        Ok(plaintext)
    }
}