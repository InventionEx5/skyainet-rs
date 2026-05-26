pub struct InDream;
impl InDream {
    pub fn new() -> Self { Self }
    pub fn dream_forward(&self, h: &[f32], _p: usize, _l: usize, _lat: Option<&[f32]>) -> Vec<f32> { h.to_vec() }
}
