// crates/t369-inference/src/moe.rs
// =====================================================
// Mixture of Experts (MoE) v2.0 — Roman Sparse MoE
// 8 Experts + Top-2 Routing + Roman Router
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct MoEConfig {
    pub num_experts: usize,      // 8 experts
    pub top_k: usize,            // Top-2 routing
    pub hidden_size: usize,
    pub intermediate_size: usize,
}

impl Default for MoEConfig {
    fn default() -> Self {
        Self {
            num_experts: 8,
            top_k: 2,
            hidden_size: 2048,
            intermediate_size: 8192,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoELayer {
    pub config: MoEConfig,
    pub router: Vec<Vec<f32>>,           // Router weights
    pub experts: Vec<ExpertFFN>,         // 8 experts
}

#[derive(Debug, Clone)]
pub struct ExpertFFN {
    pub up: Vec<Vec<f32>>,
    pub gate: Vec<Vec<f32>>,
    pub down: Vec<Vec<f32>>,
}

impl MoELayer {
    pub fn new(config: MoEConfig) -> Self {
        let mut router = vec![vec![0.0; config.hidden_size]; config.num_experts];
        for i in 0..config.num_experts {
            for j in 0..config.hidden_size {
                router[i][j] = (i as f32 * 0.017 + j as f32 * 0.013).sin() * 0.1;
            }
        }

        let mut experts = Vec::with_capacity(config.num_experts);
        for _ in 0..config.num_experts {
            experts.push(ExpertFFN {
                up: vec![vec![0.0; config.intermediate_size]; config.hidden_size],
                gate: vec![vec![0.0; config.intermediate_size]; config.hidden_size],
                down: vec![vec![0.0; config.hidden_size]; config.intermediate_size],
            });
        }

        Self { config, router, experts }
    }

    /// Forward MoE avec Top-K routing
    pub fn forward(&self, hidden: &[f32]) -> Vec<f32> {
        let hidden_size = self.config.hidden_size;
        let top_k = self.config.top_k;

        // 1. Calcul des scores du router
        let mut scores = vec![0.0; self.config.num_experts];
        for e in 0..self.config.num_experts {
            for d in 0..hidden_size {
                scores[e] += hidden[d] * self.router[e][d];
            }
            scores[e] = scores[e].tanh();
        }

        // 2. Sélection des Top-K experts
        let mut expert_indices: Vec<(usize, f32)> = scores.iter().enumerate().map(|(i, &s)| (i, s)).collect();
        expert_indices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let selected: Vec<_> = expert_indices.into_iter().take(top_k).collect();

        // 3. Exécution des experts sélectionnés
        let mut output = vec![0.0; hidden_size];
        let weight_sum: f32 = selected.iter().map(|(_, w)| w).sum();

        for (expert_id, weight) in selected {
            let expert = &self.experts[expert_id];
            let mut expert_out = vec![0.0; hidden_size];

            for i in 0..hidden_size {
                let mut gate_val = 0.0;
                let mut up_val = 0.0;

                for j in 0..self.config.intermediate_size {
                    let idx = i * self.config.intermediate_size + j;
                    gate_val += hidden[i] * expert.gate[i][j];
                    up_val += hidden[i] * expert.up[i][j];
                }

                let activated = gate_val * (up_val / (1.0 + (-up_val).exp()));
                for j in 0..self.config.intermediate_size {
                    expert_out[i] += activated * expert.down[i][j];
                }
            }

            // Pondération par le score du router
            for i in 0..hidden_size {
                output[i] += expert_out[i] * (weight / weight_sum);
            }
        }

        debug!("[MoE] Top-{} experts activés", top_k);
        output
    }
}