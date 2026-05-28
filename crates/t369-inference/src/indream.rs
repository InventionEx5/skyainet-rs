// crates/t369-inference/src/indream.rs
// =====================================================
// InDream v2.0 — Roman Dream Attention + Dream Cycle
// Version Ultra-Puissante (utilise RomanDiffusion Ultra)
// =====================================================

use crate::roman_diffusion::RomanDiffusion;
use tracing::info;

pub struct InDream {
    pub diffusion: RomanDiffusion,
    pub dream_cycles: u64,
    pub creativity_boost: f32,
}

impl InDream {
    pub fn new() -> Self {
        Self {
            diffusion: RomanDiffusion::new(),
            dream_cycles: 0,
            creativity_boost: 0.42,
        }
    }

    /// Applique la diffusion romaine ultra-puissante
    pub fn dream_forward(
        &mut self,
        hidden: &[f32],
        position: usize,
        layer: usize,
        latent_context: Option<&[f32]>,
    ) -> Vec<f32> {
        self.diffusion.apply_ultra(hidden, position, layer, latent_context)
    }

    /// Lance un vrai Dream Cycle (réflexion créative)
    pub fn run_dream_cycle(&mut self, input: &[f32]) -> Vec<f32> {
        self.dream_cycles += 1;

        let mut dreamed = input.to_vec();

        for i in 0..dreamed.len() {
            dreamed[i] = (dreamed[i] * 1.12).sin() * self.creativity_boost
                + dreamed[i] * (1.0 - self.creativity_boost);
        }

        // Renforcement créatif
        for val in &mut dreamed {
            *val = (*val * 1.03).clamp(-8.0, 8.0);
        }

        info!("[InDream] Dream Cycle #{} terminé", self.dream_cycles);
        dreamed
    }

    pub fn reset(&mut self) {
        self.diffusion.reset();
        self.dream_cycles = 0;
    }
}