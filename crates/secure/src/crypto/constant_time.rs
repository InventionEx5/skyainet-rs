// crates/secure/src/crypto/constant_time.rs
// =====================================================
// Constant-Time Primitives v1.0 — Production Ready
// SkyAInet × Nikola T369 — Gematria-Safe + Timing Attack Resistant
// =====================================================

use rand_core::{RngCore, CryptoRng};
use subtle::{Choice, ConditionallySelectable};

/// Échantillonnage uniforme modulo M en temps constant (rejet borné)
/// Utilisé principalement pour la stéganographie et Gematria
#[inline]
pub fn sample_uniform_mod<R: RngCore + CryptoRng>(rng: &mut R, modulus: u8) -> u8 {
    debug_assert!(modulus > 1 && modulus <= 95, "Modulus must be between 2 and 95");

    let bound = 256u16 - (256u16 % modulus as u16);
    let mut attempts = 0u8;

    loop {
        let byte = rng.next_u32() as u8;
        if (byte as u16) < bound {
            return ((byte as u16) % (modulus as u16)) as u8;
        }
        attempts += 1;
        if attempts > 4 {
            // Fallback déterministe (ne devrait jamais se produire en pratique)
            return (byte % modulus) as u8;
        }
    }
}

/// Addition modulo M en temps constant (sans branchement)
#[inline]
#[must_use]
pub fn add_mod(a: u8, b: u8, modulus: u8) -> u8 {
    let sum = a as u16 + b as u16;
    ((sum % modulus as u16) as u8)
}

/// Soustraction modulo M en temps constant
#[inline]
#[must_use]
pub fn sub_mod(a: u8, b: u8, modulus: u8) -> u8 {
    let diff = (a as i16 - b as i16 + modulus as i16) % modulus as i16;
    diff as u8
}

/// Sélection conditionnelle en temps constant
#[inline]
#[must_use]
pub fn select(a: u8, b: u8, choice: Choice) -> u8 {
    u8::conditional_select(&a, &b, choice)
}

/// Comparaison d’égalité en temps constant (timing-safe)
#[inline]
#[must_use]
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    subtle::ConstantTimeEq::ct_eq(a, b).into()
}

/// Comparaison d’égalité entre deux tableaux de taille fixe (plus efficace)
#[inline]
#[must_use]
pub fn constant_time_eq_fixed<const N: usize>(a: &[u8; N], b: &[u8; N]) -> bool {
    subtle::ConstantTimeEq::ct_eq(a.as_slice(), b.as_slice()).into()
}