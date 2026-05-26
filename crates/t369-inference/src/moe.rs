
#[derive(Debug, Clone)]
pub struct MoEConfig { pub num_experts: usize, pub top_k: usize, pub hidden_size: usize, pub intermediate_size: usize }
impl Default for MoEConfig { fn default() -> Self { Self { num_experts: 8, top_k: 2, hidden_size: 2048, intermediate_size: 8192 } } }
#[derive(Debug, Clone)]
pub struct MoELayer { pub config: MoEConfig, pub router: Vec<Vec<f32>>, pub experts: Vec<ExpertFFN> }
#[derive(Debug, Clone)]
pub struct ExpertFFN { pub up: Vec<Vec<f32>>, pub gate: Vec<Vec<f32>>, pub down: Vec<Vec<f32>> }
impl MoELayer {
    pub fn new(config: MoEConfig) -> Self {
        let mut router = vec![vec![0.0; config.hidden_size]; config.num_experts];
        for i in 0..config.num_experts { for j in 0..config.hidden_size { router[i][j] = (i as f32 * 0.017 + j as f32 * 0.013).sin() * 0.1; } }
        let mut experts = Vec::with_capacity(config.num_experts);
        for _ in 0..config.num_experts {
            experts.push(ExpertFFN { up: vec![vec![0.0; config.intermediate_size]; config.hidden_size], gate: vec![vec![0.0; config.intermediate_size]; config.hidden_size], down: vec![vec![0.0; config.hidden_size]; config.intermediate_size] });
        }
        Self { config, router, experts }
    }
    pub fn forward(&self, hidden: &[f32]) -> Vec<f32> { hidden.to_vec() }
}
