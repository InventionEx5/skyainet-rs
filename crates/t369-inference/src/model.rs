use crate::roman_attention::{RomanAttention, RomanAttentionConfig};
use crate::quant::QuantizedTensor;
use crate::moe::{MoELayer, MoEConfig};
use crate::kv_cache::KVCache;
use crate::roman_diffusion::RomanDiffusion;
use crate::collectivin::CollectivIn;
use crate::inself::InSelf;
use crate::inaware::InAware;
use crate::indream::InDream;
use crate::meshin::MeshIn;
use crate::tokenizer::BpeTokenizer;

#[derive(Debug, Clone)]
pub struct AwareResponse {
    pub text: String,
    pub confidence: f32,
    pub uncertainty: f32,
    pub entropy: f32,
    pub tokens_used: usize,
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_layers: usize,
    pub num_query_heads: usize,
    pub num_kv_heads: usize,
    pub head_dim: usize,
    pub max_seq_len: usize,
    pub rope_scaling: f32,
    pub bits: u8,
    pub use_moe: bool,
    pub num_experts: usize,
    pub top_k: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            vocab_size: 32000,
            hidden_size: 2048,
            num_layers: 24,
            num_query_heads: 16,
            num_kv_heads: 4,
            head_dim: 128,
            max_seq_len: 32768,
            rope_scaling: 1.0,
            bits: 4,
            use_moe: true,
            num_experts: 8,
            top_k: 2,
        }
    }
}

pub struct T369Model {
    pub config: ModelConfig,
    pub embedding: QuantizedTensor,
    pub layers: Vec<TransformerBlock>,
    pub final_norm: Vec<f32>,
    pub lm_head: QuantizedTensor,
    pub tokenizer: Option<BpeTokenizer>,
    pub kv_cache: Option<KVCache>,
    pub roman_diffusion: RomanDiffusion,
    pub collectiv_in: CollectivIn,
    pub in_self: InSelf,
    pub in_aware: InAware,
    pub in_dream: InDream,
    pub mesh_in: MeshIn,
}

#[derive(Debug)]
pub struct TransformerBlock {
    pub attention: RomanAttention,
    pub norm1: Vec<f32>,
    pub norm2: Vec<f32>,
    pub moe_layer: MoELayer,
}

impl T369Model {
    pub fn new(config: ModelConfig) -> Self {
        let bits = config.bits;
        let mut layers = Vec::with_capacity(config.num_layers);

        for _ in 0..config.num_layers {
            let attention = RomanAttention::new(RomanAttentionConfig {
                num_query_heads: config.num_query_heads,
                num_kv_heads: config.num_kv_heads,
                head_dim: config.head_dim,
                latent_dim: 32,
                diffusion_strength: 0.38,
                max_seq_len: config.max_seq_len,
                rope_base: 10000.0,
                rope_scaling: config.rope_scaling,
                use_flash: true,
                use_mhla: true,
            });

            let moe_layer = MoELayer::new(MoEConfig {
                num_experts: config.num_experts,
                top_k: config.top_k,
                hidden_size: config.hidden_size,
                intermediate_size: config.hidden_size * 4,
            });

            layers.push(TransformerBlock {
                attention,
                norm1: vec![1.0; config.hidden_size],
                norm2: vec![1.0; config.hidden_size],
                moe_layer,
            });
        }

        Self {
            config: config.clone(),
            embedding: QuantizedTensor::new(config.vocab_size, config.hidden_size, bits),
            layers,
            final_norm: vec![1.0; config.hidden_size],
            lm_head: QuantizedTensor::new(config.hidden_size, config.vocab_size, bits),
            tokenizer: None,
            kv_cache: None,
            roman_diffusion: RomanDiffusion::new(),
            collectiv_in: CollectivIn::new(),
            in_self: InSelf::new(),
            in_aware: InAware::new(),
            in_dream: InDream::new(),
            mesh_in: MeshIn::new(),
        }
    }

    pub fn init_kv_cache(&mut self) {
        if self.kv_cache.is_none() {
            self.kv_cache = Some(KVCache::new(
                self.config.num_layers,
                self.config.num_kv_heads,
                self.config.head_dim,
                self.config.max_seq_len,
            ));
        }
    }

    pub fn forward(&mut self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        let seq_len = tokens.len();
        if seq_len == 0 {
            return Err("Empty input".to_string());
        }

        let mut hidden = vec![0.0; seq_len * self.config.hidden_size];
        let emb_deq = self.embedding.dequantize();

        for (i, &token) in tokens.iter().enumerate() {
            let start = i * self.config.hidden_size;
            hidden[start..start + self.config.hidden_size]
                .copy_from_slice(&emb_deq[(token as usize) * self.config.hidden_size..]);
        }

        // === Version corrigée (sans borrow checker problem) ===
        for layer_idx in 0..self.layers.len() {
            self.apply_rms_norm(&mut hidden);

            let mut q = hidden.clone();
            let mut k = hidden.clone();
            let attn_out = self.layers[layer_idx].attention.forward(&mut q, &mut k, &hidden, seq_len);
            for i in 0..hidden.len() {
                hidden[i] += attn_out[i];
            }

            self.apply_rms_norm(&mut hidden);

            let mlp_out = self.layers[layer_idx].moe_layer.forward(&hidden);
            for i in 0..hidden.len() {
                hidden[i] += mlp_out[i];
            }

            let diffused = self.roman_diffusion.apply_ultra(&hidden, seq_len, layer_idx, None);
            for i in 0..hidden.len() {
                hidden[i] = diffused[i];
            }
        }

        self.apply_rms_norm(&mut hidden);

        let dreamed = self.in_dream.dream_forward(&hidden, seq_len, self.config.num_layers, None);
        for i in 0..hidden.len() {
            hidden[i] = dreamed[i];
        }

        let lm_deq = self.lm_head.dequantize();
        let mut logits = vec![0.0; self.config.vocab_size];

        for i in 0..self.config.hidden_size {
            for j in 0..self.config.vocab_size {
                logits[j] += hidden[(seq_len - 1) * self.config.hidden_size + i] * lm_deq[i * self.config.vocab_size + j];
            }
        }

        self.mesh_in.learn(&[1, 2, 3], 0.04);

        Ok(logits)
    }

    fn apply_rms_norm(&self, x: &mut [f32]) {
        let eps = 1e-6;
        let mut sum = 0.0;
        for &val in x.iter() {
            sum += val * val;
        }
        let rms = (sum / x.len() as f32 + eps).sqrt();
        for val in x.iter_mut() {
            *val /= rms;
        }
    }

    pub fn generate(&mut self, prompt_tokens: &[u32], max_new_tokens: usize) -> Result<Vec<u32>, String> {
        self.init_kv_cache();
        let mut tokens = prompt_tokens.to_vec();

        for _ in 0..max_new_tokens {
            let logits = self.forward(&tokens)?;
            let next_token = logits.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i, _)| i as u32).unwrap_or(0);
            tokens.push(next_token);
            if next_token == 1 { break; }
        }

        if self.in_self.is_evolving {
            self.in_self.evolve_self();
        }

        Ok(tokens)
    }

    pub fn set_tokenizer(&mut self, tokenizer: BpeTokenizer) {
        self.tokenizer = Some(tokenizer);
    }

    pub fn clear_kv_cache(&mut self) {
        if let Some(cache) = &mut self.kv_cache {
            cache.clear();
        }
    }
}
