// crates/secure/src/defense/canvas_blocker.rs
// =====================================================
// Canvas Fingerprinting Blocker v5.1 — Strong Edition
// SkyAInet × Nikola T369 — Active Evasion (AE)
// =====================================================

use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tracing::{debug, info};

use crate::crypto::roman_t369::{RomanT369, GematriaMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasProtectionLevel {
    Low,      // 35% de bruit
    Medium,   // 65% de bruit (par défaut)
    High,     // 82% de bruit
    Paranoid, // 95% de bruit + perturbation RomanT369
}

pub struct CanvasBlocker {
    noise_strength: f64,
    protection_level: CanvasProtectionLevel,
    injection_count: AtomicU64,
    total_bytes_modified: AtomicU64,
    roman: RomanT369,
}

impl CanvasBlocker {
    pub fn new(level: CanvasProtectionLevel) -> Self {
        let noise_strength = match level {
            CanvasProtectionLevel::Low => 0.35,
            CanvasProtectionLevel::Medium => 0.65,
            CanvasProtectionLevel::High => 0.82,
            CanvasProtectionLevel::Paranoid => 0.95,
        };

        Self {
            noise_strength,
            protection_level: level,
            injection_count: AtomicU64::new(0),
            total_bytes_modified: AtomicU64::new(0),
            roman: RomanT369::new([0xABu8; 32], [0xCDu8; 12], GematriaMode::Hyper256),
        }
    }

    /// Bloque et pollue activement le fingerprinting Canvas
    pub fn block_canvas_fingerprinting(&self, canvas_data: &mut [u8]) {
        let mut rng = rand::thread_rng();
        let mut modified = 0u64;

        for (i, byte) in canvas_data.iter_mut().enumerate() {
            if rng.gen_bool(self.noise_strength) {
                let noise = rng.gen_range(0..=12);

                // Bruit multi-couche
                let mut new_val = (*byte as u16 + noise) as u8;

                // Ajout de perturbation RomanT369 sur les niveaux élevés
                if self.protection_level == CanvasProtectionLevel::Paranoid {
                    let roman_noise = self.roman.encrypt(&[new_val])[0];
                    new_val = new_val.wrapping_add(roman_noise % 7);
                }

                // Variation selon la position (rendre le bruit moins uniforme)
                if i % 4 == 0 {
                    new_val = new_val.wrapping_add(3);
                }

                *byte = new_val % 255;
                modified += 1;
            }
        }

        self.injection_count.fetch_add(1, Ordering::Relaxed);
        self.total_bytes_modified.fetch_add(modified, Ordering::Relaxed);

        debug!(
            "[CanvasBlocker] Fingerprinting bloqué — {} octets modifiés (niveau: {:?})",
            modified, self.protection_level
        );
    }

    /// Génère un faux Canvas réaliste
    pub fn generate_fake_canvas(&self, width: u32, height: u32) -> Vec<u8> {
        let size = (width * height * 4) as usize;
        let mut fake = vec![0u8; size];
        let mut rng = rand::thread_rng();

        for (i, chunk) in fake.chunks_mut(4).enumerate() {
            // Couleurs plus naturelles avec légère variation
            let base = rng.gen_range(200..=245);
            chunk[0] = (base + rng.gen_range(0..=15)) as u8; // R
            chunk[1] = (base + rng.gen_range(0..=12)) as u8; // G
            chunk[2] = (base + rng.gen_range(0..=18)) as u8; // B
            chunk[3] = 255; // Alpha opaque

            // Ajout de bruit RomanT369 sur Paranoid
            if self.protection_level == CanvasProtectionLevel::Paranoid && i % 7 == 0 {
                let roman_val = self.roman.encrypt(&[chunk[0]])[0];
                chunk[0] = chunk[0].wrapping_add(roman_val % 5);
            }
        }

        info!(
            "[CanvasBlocker] Faux Canvas généré ({}x{}) — Niveau: {:?}",
            width, height, self.protection_level
        );

        fake
    }

    pub fn get_injection_count(&self) -> u64 {
        self.injection_count.load(Ordering::Relaxed)
    }

    pub fn get_total_bytes_modified(&self) -> u64 {
        self.total_bytes_modified.load(Ordering::Relaxed)
    }

    pub fn get_protection_level(&self) -> CanvasProtectionLevel {
        self.protection_level
    }
}

impl Default for CanvasBlocker {
    fn default() -> Self {
        Self::new(CanvasProtectionLevel::Medium)
    }
}