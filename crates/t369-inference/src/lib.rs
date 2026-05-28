// crates/t369-inference/src/lib.rs
// =====================================================
// T369Inference - Roman Neural Inference Engine
// =====================================================

pub mod inference;
pub mod model;
pub mod roman_attention;
pub mod moe;
pub mod kv_cache;
pub mod speculative;
pub mod parallel;
pub mod quant;
pub mod roman_diffusion;
pub mod tokenizer;
pub mod transformer_block;

pub use inference::T369Inference;
pub use model::{T369Model, ModelConfig};
pub use roman_diffusion::RomanDiffusion;
pub use moe::{MoELayer, MoEConfig};
pub use tokenizer::BpeTokenizer;
pub use transformer_block::TransformerBlock;
pub use roman_attention::{RomanAttention, RomanAttentionConfig};