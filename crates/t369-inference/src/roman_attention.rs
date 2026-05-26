

#[derive(Debug, Clone)]
pub struct RomanAttentionConfig {
    pub num_query_heads: usize,
    pub num_kv_heads: usize,
    pub head_dim: usize,
    pub latent_dim: usize,
    pub diffusion_strength: f32,
    pub max_seq_len: usize,
    pub rope_base: f32,
    pub rope_scaling: f32,
    pub use_flash: bool,
    pub use_mhla: bool,
}

impl Default for RomanAttentionConfig {
    fn default() -> Self {
        Self {
            num_query_heads: 16,
            num_kv_heads: 4,
            head_dim: 128,
            latent_dim: 32,
            diffusion_strength: 0.38,
            max_seq_len: 32768,
            rope_base: 10000.0,
            rope_scaling: 1.0,
            use_flash: true,
            use_mhla: true,
        }
    }
}

#[derive(Debug)]
pub struct RomanAttention {
    config: RomanAttentionConfig,
    cos_cache: Vec<Vec<f32>>,
    sin_cache: Vec<Vec<f32>>,
    latent_key_proj: Vec<f32>,
    latent_value_proj: Vec<f32>,
}

impl RomanAttention {
    pub fn new(config: RomanAttentionConfig) -> Self {
        let mut attn = Self {
            config: config.clone(),
            cos_cache: Vec::new(),
            sin_cache: Vec::new(),
            latent_key_proj: vec![0.0; config.head_dim * config.latent_dim],
            latent_value_proj: vec![0.0; config.head_dim * config.latent_dim],
        };
        attn.precompute_rope();
        attn.init_latent_projections();
        attn
    }

    fn init_latent_projections(&mut self) {
        for i in 0..self.latent_key_proj.len() { self.latent_key_proj[i] = (i as f32 * 0.013).sin() * 0.1; }
        for i in 0..self.latent_value_proj.len() { self.latent_value_proj[i] = (i as f32 * 0.017).cos() * 0.1; }
    }

    fn precompute_rope(&mut self) {
        let head_dim = self.config.head_dim;
        let max_seq = self.config.max_seq_len;
        let base = self.config.rope_base;
        let scaling = self.config.rope_scaling;
        self.cos_cache = vec![vec![0.0; head_dim]; max_seq];
        self.sin_cache = vec![vec![0.0; head_dim]; max_seq];
        for pos in 0..max_seq {
            for i in 0..head_dim / 2 {
                let freq = (pos as f32) / (base.powf((2 * i) as f32 / head_dim as f32) * scaling);
                self.cos_cache[pos][2 * i] = freq.cos();
                self.cos_cache[pos][2 * i + 1] = freq.cos();
                self.sin_cache[pos][2 * i] = freq.sin();
                self.sin_cache[pos][2 * i + 1] = freq.sin();
            }
        }
    }

    pub fn forward(&self, query: &mut [f32], key: &mut [f32], value: &[f32], seq_len: usize) -> Vec<f32> {
        if self.config.use_mhla { return self.forward_mhla(query, key, value, seq_len); }
        let q_heads = self.config.num_query_heads;
        let kv_heads = self.config.num_kv_heads;
        let head_dim = self.config.head_dim;
        let kv_repeat = q_heads / kv_heads;
        let mut output = vec![0.0; query.len()];
        for i in 0..seq_len {
            for qh in 0..q_heads {
                let kv_h = qh / kv_repeat;
                let q_offset = (i * q_heads + qh) * head_dim;
                let k_offset = (i * kv_heads + kv_h) * head_dim;
                let v_offset = (i * kv_heads + kv_h) * head_dim;
                let mut score = 0.0;
                for d in 0..head_dim { score += query[q_offset + d] * key[k_offset + d]; }
                score = (score.tanh() * 0.88) + ((score * 0.6).sin() * self.config.diffusion_strength * 0.12);
                for d in 0..head_dim {
                    let out_idx = (i * q_heads + qh) * head_dim + d;
                    output[out_idx] += score * value[v_offset + d];
                }
            }
        }
        output
    }

    fn forward_mhla(&self, query: &[f32], key: &[f32], value: &[f32], seq_len: usize) -> Vec<f32> {
        let q_heads = self.config.num_query_heads;
        let head_dim = self.config.head_dim;
        let latent_dim = self.config.latent_dim;
        let mut output = vec![0.0; query.len()];
        for i in 0..seq_len {
            for qh in 0..q_heads {
                let q_offset = (i * q_heads + qh) * head_dim;
                let mut latent_key = vec![0.0; latent_dim];
                let mut latent_value = vec![0.0; latent_dim];
                for d in 0..head_dim {
                    for l in 0..latent_dim {
                        let proj_idx = d * latent_dim + l;
                        latent_key[l] += key[q_offset + d] * self.latent_key_proj[proj_idx];
                        latent_value[l] += value[q_offset + d] * self.latent_value_proj[proj_idx];
                    }
                }
                for l in 0..latent_dim {
                    latent_key[l] = (latent_key[l].tanh() * 0.88) + ((latent_key[l] * 0.6).sin() * 0.12);
                    latent_value[l] = (latent_value[l].tanh() * 0.88) + ((latent_value[l] * 0.6).sin() * 0.12);
                }
                let mut score = 0.0;
                for l in 0..latent_dim { score += query[q_offset + l % head_dim] * latent_key[l]; }
                score = (score.tanh() * 0.88) + ((score * 0.6).sin() * 0.12);
                for d in 0..head_dim {
                    let out_idx = (i * q_heads + qh) * head_dim + d;
                    let mut contrib = 0.0;
                    for l in 0..latent_dim {
                        let proj_idx = d * latent_dim + l;
                        contrib += latent_value[l] * self.latent_value_proj[proj_idx];
                    }
                    output[out_idx] += score * contrib;
                }
            }
        }
        output
    }
}
