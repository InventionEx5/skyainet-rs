/// Roman Neural Inference - Version Finale Ultra-Puissante
/// Combine S-Box romaine, poids adaptatifs, multi-phase et diffusion par couche
pub struct RomanDiffusion {
    base_weights: [f32; 7],           // I, V, X, L, C, D, M
    phase: f32,
    layer_factor: f32,
}

impl RomanDiffusion {
    pub fn new() -> Self {
        Self {
            base_weights: [1.0, 5.0, 10.0, 50.0, 100.0, 200.0, 250.0],
            phase: 0.0,
            layer_factor: 1.0,
        }
    }

    /// Version ultra-puissante avec S-Box romaine + diffusion multi-opération
    pub fn apply(&mut self, hidden: &[f32], position: usize, layer: usize) -> Vec<f32> {
        let mut output = Vec::with_capacity(hidden.len());
        self.phase += 0.007;
        self.layer_factor = 1.0 + (layer as f32 * 0.03);

        for (i, &value) in hidden.iter().enumerate() {
            let roman_idx = (i + position + layer) % 7;
            let weight = self.base_weights[roman_idx] * self.layer_factor;

            // === Roman S-Box (transformation non-linéaire) ===
            let sboxed = self.roman_sbox(value, weight, i + position);

            // === Diffusion multi-opération romaine ===
            let diffused = match (i + position) % 7 {
                0 => sboxed - weight * 0.009,                    // Soustractif pur
                1 => sboxed + weight * 0.009,                    // Additif pur
                2 => self.roman_xor(sboxed, weight),             // XOR romain
                3 => sboxed * (1.0 + weight * 0.0007),           // Scaling romain
                4 => self.roman_rotate(sboxed, weight as i32),   // Rotation romaine
                5 => self.roman_hybrid(sboxed, weight, self.phase), // Hybride phase
                _ => self.roman_chaotic(sboxed, weight, i),      // Mode chaotique romain
            };

            // Normalisation + clamping
            let normalized = diffused.clamp(-12.0, 12.0) * 0.98;
            output.push(normalized);
        }

        output
    }

    /// Roman S-Box (transformation non-linéaire inspirée des chiffres romains)
    #[inline(always)]
    fn roman_sbox(&self, value: f32, weight: f32, seed: usize) -> f32 {
        let x = value + (weight * 0.001);
        let sin_component = (x * 3.7 + seed as f32).sin() * 0.15;
        let cos_component = (x * 2.3).cos() * 0.1;
        
        x + sin_component + cos_component
    }

    /// XOR romain (très bon pour la diffusion)
    #[inline(always)]
    fn roman_xor(&self, value: f32, weight: f32) -> f32 {
        let bits = value.to_bits();
        let weight_bits = (weight * 1000.0) as u32;
        let xored = bits ^ weight_bits;
        (xored as f32) * 0.0000001 + value * 0.999
    }

    /// Rotation romaine
    #[inline(always)]
    fn roman_rotate(&self, value: f32, shift: i32) -> f32 {
        let bits = value.to_bits();
        let rotated = if shift > 0 {
            bits.rotate_left(shift as u32 % 32)
        } else {
            bits.rotate_right((-shift) as u32 % 32)
        };
        (rotated as f32) * 0.0000001 + value * 0.999
    }

    /// Mode hybride avec phase romaine
    #[inline(always)]
    fn roman_hybrid(&self, value: f32, weight: f32, phase: f32) -> f32 {
        let phase_mod = (phase + weight * 0.01).sin() * 0.5 + 0.5;
        value * (1.0 + weight * 0.0004 * phase_mod) + (weight * 0.002) * phase_mod
    }

    /// Mode chaotique romain (très bon pour briser les patterns)
    #[inline(always)]
    fn roman_chaotic(&self, value: f32, weight: f32, seed: usize) -> f32 {
        let chaos = ((seed as f32 * 0.37).sin() * 0.5 + 0.5) * weight * 0.001;
        value + chaos - (value * 0.001)
    }

    /// Version pour les couches très profondes
    pub fn apply_deep(&mut self, hidden: &[f32], position: usize, layer: usize) -> Vec<f32> {
        let mut output = self.apply(hidden, position, layer);

        // Renforcement sur les couches profondes
        if layer >= 6 {
            for val in &mut output {
                *val = *val * 1.015 + (self.base_weights[layer % 7] * 0.0008);
            }
        }

        output
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.layer_factor = 1.0;
    }
}