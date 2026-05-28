// crates/t369-inference/src/transformer_block.rs
// =====================================================
// TransformerBlock v4.0 — ULTRA-PUISSANT
// RMSNorm + Roman Dream Attention (GQA + MHLA) + MoE + RomanDiffusion Ultra
// =====================================================

use crate::roman_attention::{RomanAttention, RomanAttentionConfig};
use crate::moe::{MoELayer, MoEConfig};
use crate::roman_diffusion::RomanDiffusion;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct TransformerBlock {
    pub attention: RomanAttention,
    pub norm1: Vec<f32>,           // RMSNorm weights
    pub norm2: Vec<f32>,           // RMSNorm weights
    pub moe_layer: MoELayer,       // ← MoE (remplace SwiGLU)
    pub roman_diffusion: RomanDiffusion,
    pub hidden_size: usize,
}

impl TransformerBlock {
    pub fn new(hidden_size: usize, num_query_heads: usize, num_kv_heads: usize, head_dim: usize) -> Self {
        let attention = RomanAttention::new(RomanAttentionConfig {
            num_query_heads,
            num_kv_heads,
            head_dim,
            latent_dim: 32,
            diffusion_strength: 0.38,
            max_seq_len: 32768,
            rope_base: 10000.0,
            rope_scaling: 1.0,
            use_flash: true,
            use_mhla: true,
        });

        let moe_layer = MoELayer::new(MoEConfig {
            num_experts: 8,
            top_k: 2,
            hidden_size,
            intermediate_size: hidden_size * 4,
        });

        Self {
            attention,
            norm1: vec![1.0; hidden_size],
            norm2: vec![1.0; hidden_size],
            moe_layer,
            roman_diffusion: RomanDiffusion::new(),
            hidden_size,
        }
    }

    /// Forward pass ultra-puissant
    #[inline]
    pub fn forward(&mut self, hidden: &mut [f32], seq_len: usize, layer_idx: usize) {
        let hidden_size = self.hidden_size;

        // === 1. Pre-Norm + Roman Dream Attention (GQA + MHLA + RoPE) ===
        let mut normed = self.rms_norm(hidden);
        let attn_out = self.attention.forward(&normed, &normed, &normed, seq_len);

        // Residual
        for i in 0..hidden.len() {
            hidden[i] += attn_out[i];
        }

        // === 2. Pre-Norm + MoE (remplace SwiGLU) ===
        normed = self.rms_norm(hidden);
        let mlp_out = self.moe_layer.forward(&normed);

        // Residual
        for i in 0..hidden.len() {
            hidden[i] += mlp_out[i];
        }

        // === 3. RomanDiffusion Ultra (post-processing puissant) ===
        let diffused = self.roman_diffusion.apply_ultra(hidden, seq_len, layer_idx, None);

        for i in 0..hidden.len() {
            hidden[i] = diffused[i];
        }

        debug!("[TransformerBlock] Layer {} processed (MoE + RomanDiffusion Ultra)", layer_idx);
    }

    /// RMSNorm optimisé
    #[inline]
    fn rms_norm(&self, x: &[f32]) -> Vec<f32> {
        let eps = 1e-6;
        let mut normed = x.to_vec();
        let mut sum = 0.0;

        for &val in x.iter() {
            sum += val * val;
        }
        let rms = (sum / x.len() as f32 + eps).sqrt();

        for val in normed.iter_mut() {
            *val /= rms;
        }

        normed
    }
}