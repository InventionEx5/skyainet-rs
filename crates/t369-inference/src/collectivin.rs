pub struct CollectivIn;
impl CollectivIn {
    pub fn new() -> Self { Self }
    pub fn collective_reason(&self, h: &[f32], _p: usize, _l: usize) -> Vec<f32> { h.to_vec() }
}
