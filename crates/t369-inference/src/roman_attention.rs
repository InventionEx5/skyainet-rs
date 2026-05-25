use crate::roman_diffusion::RomanDiffusion;

pub struct RomanAttention {
    hidden_size: usize,
    roman_diffusion: RomanDiffusion,
}

impl RomanAttention {
    pub fn new(hidden_size: usize) -> Self {
        Self {
            hidden_size,
            roman_diffusion: RomanDiffusion::new(),
        }
    }

    pub fn forward(&mut self, hidden: &[f32]) -> Result<Vec<f32>, String> {
        // Version simplifi�e mais avec diffusion romaine
        let mut output = Vec::with_capacity(hidden.len());

        for (i, &val) in hidden.iter().enumerate() {
            // Attention simul�e + diffusion romaine
            let diffused = self.roman_diffusion.apply(&[val], i)[0];
            output.push(diffused * 0.8 + val * 0.2); // skip connection l�g�re
        }

        Ok(output)
    }
}