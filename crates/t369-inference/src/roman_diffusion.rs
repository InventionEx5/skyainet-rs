pub struct RomanDiffusion;
impl RomanDiffusion {
    pub fn new() -> Self { Self }
    pub fn apply_ultra(&self, h: &[f32], _p: usize, _l: usize, _lat: Option<&[f32]>) -> Vec<f32> { h.to_vec() }
    pub fn reset(&mut self) {}
}
