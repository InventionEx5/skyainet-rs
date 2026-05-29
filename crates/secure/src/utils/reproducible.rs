// crates/secure/src/utils/reproducible.rs
// =====================================================
// Reproducible Utilities v6.1 — RNG et Hash Déterministes
// Compatible avec Contact, DID, Groupes et Tests
// SkyAInet × Nikola T369
// =====================================================

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::RngCore;
use tracing::debug;

/// Crée un générateur de nombres aléatoires **reproductible** à partir d’une graine
#[inline]
pub fn create_reproducible_rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

/// Calcule un hash **déterministe** (u64) à partir de données
#[inline]
pub fn deterministic_hash(data: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash_slice(data, &mut hasher);
    std::hash::Hasher::finish(&hasher)
}

/// Génère un tableau d’octets **déterministe** à partir d’une graine
pub fn generate_deterministic_bytes(seed: u64, length: usize) -> Vec<u8> {
    let mut rng = create_reproducible_rng(seed);
    let mut bytes = vec![0u8; length];
    rng.fill_bytes(&mut bytes);

    debug!(
        "[Reproducible] {} octets générés avec la graine {}",
        length, seed
    );

    bytes
}