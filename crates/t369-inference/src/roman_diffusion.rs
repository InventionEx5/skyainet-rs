// crates/t369-inference/src/roman_diffusion.rs
// =====================================================
// RomanDiffusion v4.0 — ULTRA-PUISSANTE
// S-Box Romaine + Diffusion Multi-Phase + Adaptive Weights + Latent Integration
// =====================================================

use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct RomanDiffusion {
    // Poids romains adaptatifs (I, V, X, L, C, D, M)
    base_weights: [f32; 7],
    phase: f32,
    layer_factor: f32,
    depth_boost: f32,           // Boost pour couches profondes
    chaos_intensity: f32,       // Intensité du mode chaotique
    latent_influence: f32,      // Influence de l'espace latent (MHLA)
}

impl RomanDiffusion {
    pub fn new() -> Self {
        Self {
            base_weights: [1.0, 5.0, 10.0, 50.0, 100.0, 200.0, 250.0],
            phase: 0.0,
            layer_factor: 1.0,
            depth_boost: 1.0,
            chaos_intensity: 0.012,
            latent_influence: 0.38,
        }
    }

    /// Version **ULTRA-PUISSANTE** (recommandée)
    pub fn apply_ultra(
        &mut self,
        hidden: &[f32],
        position: usize,
        layer: usize,
        latent_context: Option<&[f32]>, // Optionnel : contexte latent (MHLA)
    ) -> Vec<f32> {
        let mut output = Vec::with_capacity(hidden.len());
        self.phase += 0.009;
        self.layer_factor = 1.0 + (layer as f32 * 0.028);
        self.depth_boost = if layer >= 8 { 1.018 } else { 1.0 };

        for (i, &value) in hidden.iter().enumerate() {
            let roman_idx = (i + position + layer) % 7;
            let mut weight = self.base_weights[roman_idx] * self.layer_factor;

            // === Influence latente (MHLA) ===
            if let Some(latent) = latent_context {
                let latent_val = latent[i % latent.len()];
                weight += latent_val * self.latent_influence * 0.1;
            }

            // === Roman S-Box Ultra ===
            let sboxed = self.roman_sbox_ultra(value, weight, i + position + layer);

            // === Diffusion Multi-Opération Romaine ===
            let diffused = match (i + position + layer) % 9 {
                0 => sboxed - weight * 0.011,                              // Soustractif pur
                1 => sboxed + weight * 0.011,                              // Additif pur
                2 => self.roman_xor_ultra(sboxed, weight),                 // XOR romain renforcé
                3 => sboxed * (1.0 + weight * 0.0009),                     // Scaling romain
                4 => self.roman_rotate_ultra(sboxed, weight as i32),       // Rotation romaine
                5 => self.roman_hybrid_ultra(sboxed, weight, self.phase),  // Hybride phase
                6 => self.roman_chaotic_ultra(sboxed, weight, i),          // Chaotique renforcé
                7 => self.roman_spiral(sboxed, weight, position),          // Mode spirale romaine
                _ => self.roman_quantum(sboxed, weight, layer),            // Mode "quantique" romain
            };

            // === Normalisation + Clamping + Depth Boost ===
            let normalized = (diffused * self.depth_boost).clamp(-14.0, 14.0) * 0.97;
            output.push(normalized);
        }

        output
    }

    // =====================================================
    // FONCTIONS ULTRA (améliorées)
    // =====================================================

    #[inline(always)]
    fn roman_sbox_ultra(&self, value: f32, weight: f32, seed: usize) -> f32 {
        let x = value + (weight * 0.0012);
        let sin1 = (x * 4.1 + seed as f32 * 0.37).sin() * 0.18;
        let sin2 = (x * 1.9).sin() * 0.09;
        let cos1 = (x * 2.7).cos() * 0.07;
        x + sin1 + sin2 + cos1
    }

    #[inline(always)]
    fn roman_xor_ultra(&self, value: f32, weight: f32) -> f32 {
        let bits = value.to_bits();
        let w = (weight * 1371.0) as u32;
        let xored = bits ^ w ^ (bits.rotate_right(7));
        (xored as f32) * 0.00000009 + value * 0.998
    }

    #[inline(always)]
    fn roman_rotate_ultra(&self, value: f32, shift: i32) -> f32 {
        let bits = value.to_bits();
        let rotated = bits.rotate_left((shift as u32 + 11) % 29);
        (rotated as f32) * 0.00000008 + value * 0.997
    }

    #[inline(always)]
    fn roman_hybrid_ultra(&self, value: f32, weight: f32, phase: f32) -> f32 {
        let phase_mod = ((phase * 1.3) + (weight * 0.013)).sin() * 0.6 + 0.4;
        value * (1.0 + weight * 0.0005 * phase_mod) 
            + (weight * 0.0028) * phase_mod 
            + (value * 0.0003).sin() * 0.4
    }

    #[inline(always)]
    fn roman_chaotic_ultra(&self, value: f32, weight: f32, seed: usize) -> f32 {
        let chaos = ((seed as f32 * 0.41).sin() * self.chaos_intensity) 
                  + ((seed as f32 * 0.19).cos() * self.chaos_intensity * 0.7);
        value + chaos - (value * 0.0012)
    }

    #[inline(always)]
    fn roman_spiral(&self, value: f32, weight: f32, position: usize) -> f32 {
        let spiral = ((position as f32 * 0.27).sin() * 0.5 + 0.5) * weight * 0.0006;
        value * (1.0 + spiral) + (value * 0.0008).cos() * 0.3
    }

    #[inline(always)]
    fn roman_quantum(&self, value: f32, weight: f32, layer: usize) -> f32 {
        let q = (layer as f32 * 0.11).sin() * 0.4 + 0.6;
        value * q + (weight * 0.0018) * (1.0 - q)
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.layer_factor = 1.0;
        self.depth_boost = 1.0;
    }
}