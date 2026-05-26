#[derive(Debug, Clone)]
pub struct QuantizedTensor {
    pub data: Vec<i8>,
    pub scale: f32,
    pub zero_point: i8,
    pub bits: u8,
    pub original_shape: (usize, usize),
}

impl QuantizedTensor {
    pub fn new(rows: usize, cols: usize, bits: u8) -> Self {
        let size = if bits == 4 { (rows * cols + 1) / 2 } else { rows * cols };
        Self {
            data: vec![0; size],
            scale: 1.0,
            zero_point: 0,
            bits,
            original_shape: (rows, cols),
        }
    }

    pub fn dequantize(&self) -> Vec<f32> {
        let mut result = Vec::with_capacity(self.original_shape.0 * self.original_shape.1);
        for &q in &self.data {
            result.push((q as f32 - self.zero_point as f32) * self.scale);
        }
        result
    }
}
