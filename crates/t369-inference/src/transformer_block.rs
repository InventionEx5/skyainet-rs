use crate::layers::roman_attention::RomanAttention;
use crate::layers::feedforward::FeedForward;
use crate::roman_diffusion::RomanDiffusion;

pub struct TransformerBlock {
    pub attention: RomanAttention,
    pub feedforward: FeedForward,
    pub norm1: Vec<f32>,
    pub norm2: Vec<f32>,
    hidden_size: usize,
}

impl TransformerBlock {
    pub fn new(hidden_size: usize) -> Self {
        Self {
            attention: RomanAttention::new(hidden_size),
            feedforward: FeedForward::new(hidden_size),
            norm1: vec![1.0; hidden_size],
            norm2: vec![1.0; hidden_size],
            hidden_size,
        }
    }

    pub fn forward(&self, hidden: &[f32]) -> Result<Vec<f32>, String> {
        // Pre-norm + Roman Attention
        let normed = self.layer_norm(hidden, &self.norm1);
        let attn_out = self.attention.forward(&normed)?;
        
        let residual1: Vec<f32> = hidden.iter().zip(attn_out.iter())
            .map(|(a, b)| a + b)
            .collect();

        // Pre-norm + FeedForward
        let normed2 = self.layer_norm(&residual1, &self.norm2);
        let ff_out = self.feedforward.forward(&normed2)?;
        
        let output: Vec<f32> = residual1.iter().zip(ff_out.iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(output)
    }

    fn layer_norm(&self, input: &[f32], scale: &[f32]) -> Vec<f32> {
        let mean: f32 = input.iter().sum::<f32>() / input.len() as f32;
        let var: f32 = input.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / input.len() as f32;
        let std = (var + 1e-5).sqrt();

        input.iter().zip(scale.iter())
            .map(|(x, s)| (x - mean) / std * s)
            .collect()
    }
}