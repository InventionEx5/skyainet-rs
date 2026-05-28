// crates/t369-inference/src/quant.rs
// =====================================================
// Quant v3.0 — 4-bit & 8-bit Quantization (GGUF-style)
// Ultra-optimisé + Compatible avec ton ancien QuantizedTensor
// =====================================================

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedTensor {
    pub data: Vec<i8>,           // Packed data (4-bit or 8-bit)
    pub scale: f32,
    pub zero_point: i8,
    pub bits: u8,                // 4 or 8
    pub original_shape: (usize, usize),
}

impl QuantizedTensor {
    /// Crée un nouveau tenseur quantifié
    pub fn new(rows: usize, cols: usize, bits: u8) -> Self {
        let size = if bits == 4 {
            (rows * cols + 1) / 2   // 2 valeurs par octet
        } else {
            rows * cols
        };

        Self {
            data: vec![0; size],
            scale: 1.0,
            zero_point: 0,
            bits,
            original_shape: (rows, cols),
        }
    }

    /// Quantize depuis des floats (8-bit ou 4-bit)
    pub fn quantize_from_f32(data: &[f32], bits: u8) -> Self {
        if data.is_empty() {
            return Self::new(0, 0, bits);
        }

        let min_val = data.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_val = data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let scale = if max_val != min_val {
            (max_val - min_val) / ((1 << bits) - 1) as f32
        } else {
            1.0
        };

        let zero_point = (-min_val / scale).round() as i8;

        let mut quantized = Vec::with_capacity(data.len());

        if bits == 8 {
            for &val in data {
                let q = ((val / scale) + zero_point as f32).round() as i8;
                quantized.push(q.clamp(-128, 127));
            }
        } else if bits == 4 {
            // 4-bit packing (2 valeurs par octet)
            for chunk in data.chunks(2) {
                let q1 = ((chunk[0] / scale) + zero_point as f32).round() as i8;
                let q2 = if chunk.len() > 1 {
                    ((chunk[1] / scale) + zero_point as f32).round() as i8
                } else {
                    0
                };

                let packed = ((q1 & 0x0F) | ((q2 & 0x0F) << 4)) as i8;
                quantized.push(packed);
            }
        }

        Self {
            data: quantized,
            scale,
            zero_point,
            bits,
            original_shape: (data.len(), 1),
        }
    }

    /// Déquantize vers des floats
    pub fn dequantize(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.original_shape.0 * self.original_shape.1);

        if self.bits == 8 {
            for &q in &self.data {
                result.push((q as f32 - self.zero_point as f32) * self.scale);
            }
        } else if self.bits == 4 {
            for &packed in &self.data {
                let q1 = (packed & 0x0F) as i8;
                let q2 = ((packed >> 4) & 0x0F) as i8;

                result.push((q1 as f32 - self.zero_point as f32) * self.scale);

                if result.len() < self.original_shape.0 * self.original_shape.1 {
                    result.push((q2 as f32 - self.zero_point as f32) * self.scale);
                }
            }
        }

        result
    }

    /// Version optimisée pour l'inférence (évite allocation)
    #[inline]
    pub fn dequantize_inplace(&self, output: &mut [f32]) {
        let len = output.len().min(self.original_shape.0 * self.original_shape.1);

        if self.bits == 8 {
            for i in 0..len {
                output[i] = (self.data[i] as f32 - self.zero_point as f32) * self.scale;
            }
        } else if self.bits == 4 {
            let mut out_idx = 0;
            for &packed in &self.data {
                if out_idx >= len { break; }
                output[out_idx] = ((packed & 0x0F) as f32 - self.zero_point as f32) * self.scale;
                out_idx += 1;

                if out_idx < len {
                    output[out_idx] = (((packed >> 4) & 0x0F) as f32 - self.zero_point as f32) * self.scale;
                    out_idx += 1;
                }
            }
        }
    }
}