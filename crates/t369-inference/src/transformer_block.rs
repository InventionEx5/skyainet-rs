use crate::roman_attention::RomanAttention;
use crate::moe::MoELayer;

#[derive(Debug)]
pub struct TransformerBlock {
    pub attention: RomanAttention,
    pub norm1: Vec<f32>,
    pub norm2: Vec<f32>,
    pub moe_layer: MoELayer,
}
