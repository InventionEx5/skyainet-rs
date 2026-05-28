// crates/t369-inference/src/kv_cache.rs
// =====================================================
// KVCache v3.0 — High-Performance Key-Value Cache
// Ultra-optimized for autoregressive generation
// =====================================================

use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct KVCache {
    /// Keys per layer: [layer][seq_len][num_heads * head_dim]
    pub keys: Vec<Vec<Vec<f32>>>,
    /// Values per layer: [layer][seq_len][num_heads * head_dim]
    pub values: Vec<Vec<Vec<f32>>>,
    pub num_layers: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub max_seq_len: usize,
    pub current_seq_len: usize,
}

impl KVCache {
    pub fn new(
        num_layers: usize,
        num_heads: usize,
        head_dim: usize,
        max_seq_len: usize,
    ) -> Self {
        let head_size = num_heads * head_dim;

        let keys = vec![vec![vec![0.0; head_size]; max_seq_len]; num_layers];
        let values = vec![vec![vec![0.0; head_size]; max_seq_len]; num_layers];

        Self {
            keys,
            values,
            num_layers,
            num_heads,
            head_dim,
            max_seq_len,
            current_seq_len: 0,
        }
    }

    /// Append new key and value for current position
    #[inline]
    pub fn append(&mut self, layer: usize, key: &[f32], value: &[f32]) {
        if layer >= self.num_layers {
            return;
        }

        if self.current_seq_len >= self.max_seq_len {
            // Simple overflow handling (can be improved with ring buffer later)
            return;
        }

        let pos = self.current_seq_len;
        let head_size = self.num_heads * self.head_dim;

        // Copy key
        for i in 0..head_size.min(key.len()) {
            self.keys[layer][pos][i] = key[i];
        }

        // Copy value
        for i in 0..head_size.min(value.len()) {
            self.values[layer][pos][i] = value[i];
        }

        if layer == self.num_layers - 1 {
            self.current_seq_len += 1;
        }
    }

    /// Get keys and values up to current position for a layer
    #[inline]
    pub fn get_layer(&self, layer: usize) -> Option<(&[Vec<f32>], &[Vec<f32>])> {
        if layer >= self.num_layers {
            return None;
        }

        Some((
            &self.keys[layer][0..self.current_seq_len],
            &self.values[layer][0..self.current_seq_len],
        ))
    }

    /// Reset cache (for new generation)
    pub fn clear(&mut self) {
        self.current_seq_len = 0;
    }

    /// Resize cache if needed (for longer context)
    pub fn resize(&mut self, new_max_seq_len: usize) {
        if new_max_seq_len <= self.max_seq_len {
            return;
        }

        let head_size = self.num_heads * self.head_dim;

        for layer in 0..self.num_layers {
            self.keys[layer].resize(new_max_seq_len, vec![0.0; head_size]);
            self.values[layer].resize(new_max_seq_len, vec![0.0; head_size]);
        }

        self.max_seq_len = new_max_seq_len;
    }

    /// Get current sequence length
    pub fn len(&self) -> usize {
        self.current_seq_len
    }

    pub fn is_empty(&self) -> bool {
        self.current_seq_len == 0
    }
}