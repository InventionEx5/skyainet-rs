#[derive(Debug, Clone)]
pub struct QuantizedTensor {
    pub data: Vec<f32>,
    pub scale: f32,
    pub zero_point: i32,
}

impl QuantizedTensor {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            scale: 1.0,
            zero_point: 0,
        }
    }

    pub fn dequantize(&self) -> Vec<f32> {
        self.data.iter()
            .map(|&x| x * self.scale + self.zero_point as f32)
            .collect()
    }
}