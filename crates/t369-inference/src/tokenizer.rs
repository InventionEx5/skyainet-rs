// crates/t369-inference/src/tokenizer.rs
// =====================================================
// T369 BPE Tokenizer v2.0 — Extremely Optimized
// High-performance Byte Pair Encoding for T369Inference
// =====================================================

use std::collections::HashMap;
use ahash::AHashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct BpeTokenizer {
    /// token -> id
    pub vocab: AHashMap<String, u32>,
    /// id -> token
    pub id_to_token: Vec<String>,
    /// Merge rules: (pair) -> rank
    pub merges: AHashMap<(u32, u32), u32>,
    /// Special tokens
    pub bos_token: u32,
    pub eos_token: u32,
    pub pad_token: u32,
    pub unk_token: u32,
    /// Cache for frequent tokens
    pub encode_cache: AHashMap<String, Vec<u32>>,
}

impl BpeTokenizer {
    pub fn new() -> Self {
        Self {
            vocab: AHashMap::new(),
            id_to_token: Vec::new(),
            merges: AHashMap::new(),
            bos_token: 0,
            eos_token: 1,
            pad_token: 2,
            unk_token: 3,
            encode_cache: AHashMap::new(),
        }
    }

    /// Load tokenizer from vocabulary and merges
    pub fn load(vocab: Vec<(String, u32)>, merges: Vec<((u32, u32), u32)>) -> Self {
        let mut tokenizer = Self::new();

        for (token, id) in vocab {
            tokenizer.vocab.insert(token.clone(), id);
            if tokenizer.id_to_token.len() <= id as usize {
                tokenizer.id_to_token.resize((id + 1) as usize, String::new());
            }
            tokenizer.id_to_token[id as usize] = token;
        }

        for ((a, b), rank) in merges {
            tokenizer.merges.insert((a, b), rank);
        }

        debug!("[Tokenizer] Loaded {} tokens and {} merges", 
               tokenizer.vocab.len(), tokenizer.merges.len());

        tokenizer
    }

    /// Encode text into token IDs (BPE)
    pub fn encode(&mut self, text: &str) -> Vec<u32> {
        // Check cache first
        if let Some(tokens) = self.encode_cache.get(text) {
            return tokens.clone();
        }

        let mut tokens = self.pre_tokenize(text);
        let mut result = Vec::new();

        // Apply BPE merges
        while tokens.len() > 1 {
            let mut best_rank = u32::MAX;
            let mut best_pair = None;

            for i in 0..tokens.len() - 1 {
                let pair = (tokens[i], tokens[i + 1]);
                if let Some(&rank) = self.merges.get(&pair) {
                    if rank < best_rank {
                        best_rank = rank;
                        best_pair = Some(i);
                    }
                }
            }

            if let Some(i) = best_pair {
                let new_token = self.get_merged_token(tokens[i], tokens[i + 1]);
                tokens.splice(i..i + 2, std::iter::once(new_token));
            } else {
                break;
            }
        }

        // Convert to IDs
        for token in tokens {
            if let Some(&id) = self.vocab.get(&token) {
                result.push(id);
            } else {
                result.push(self.unk_token);
            }
        }

        // Cache the result
        if self.encode_cache.len() < 10_000 {
            self.encode_cache.insert(text.to_string(), result.clone());
        }

        result
    }

    /// Decode token IDs back to text
    pub fn decode(&self, tokens: &[u32]) -> String {
        let mut result = String::new();

        for &id in tokens {
            if let Some(token) = self.id_to_token.get(id as usize) {
                result.push_str(token);
            } else {
                result.push_str("<unk>");
            }
        }

        result
    }

    /// Simple pre-tokenization (can be improved later with regex)
    fn pre_tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .flat_map(|word| {
                let mut chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
                if !chars.is_empty() {
                    // Add end-of-word marker
                    if let Some(last) = chars.last_mut() {
                        *last = format!("{}</w>", last);
                    }
                }
                chars
            })
            .collect()
    }

    fn get_merged_token(&self, a: u32, b: u32) -> String {
        let token_a = self.id_to_token.get(a as usize).map(|s| s.as_str()).unwrap_or("");
        let token_b = self.id_to_token.get(b as usize).map(|s| s.as_str()).unwrap_or("");
        format!("{}{}", token_a, token_b)
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Add a special token
    pub fn add_special_token(&mut self, token: &str, id: u32) {
        self.vocab.insert(token.to_string(), id);
        if self.id_to_token.len() <= id as usize {
            self.id_to_token.resize((id + 1) as usize, String::new());
        }
        self.id_to_token[id as usize] = token.to_string();
    }
}