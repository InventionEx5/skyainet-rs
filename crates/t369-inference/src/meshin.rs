// crates/t369-inference/src/meshin.rs
// =====================================================
// MeshIn v2.0 — Evolving Neural Mesh
// Réseau neuronal dynamique auto-évolutif (Hebbian + Neurogenesis)
// =====================================================

use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Neuron {
    pub id: u64,
    pub wisdom: f32,
    pub activation: f32,
    pub connections: Vec<u64>,
    pub last_used: u64,
}

pub struct MeshIn {
    pub neurons: HashMap<u64, Neuron>,
    pub next_id: u64,
    pub total_synapses: usize,
    pub average_wisdom: f32,
}

impl MeshIn {
    pub fn new() -> Self {
        let mut mesh = Self {
            neurons: HashMap::new(),
            next_id: 1,
            total_synapses: 0,
            average_wisdom: 0.5,
        };
        // Création des neurones de base
        for _ in 0..64 {
            mesh.add_neuron(0.5);
        }
        mesh
    }

    pub fn add_neuron(&mut self, initial_wisdom: f32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.neurons.insert(id, Neuron {
            id,
            wisdom: initial_wisdom,
            activation: 0.0,
            connections: Vec::new(),
            last_used: 0,
        });

        debug!("[MeshIn] Neurone {} créé", id);
        id
    }

    /// Hebbian learning + Neurogenesis
    pub fn learn(&mut self, neuron_ids: &[u64], strength: f32) {
        for &id in neuron_ids {
            if let Some(neuron) = self.neurons.get_mut(&id) {
                neuron.wisdom = (neuron.wisdom + strength * 0.1).min(0.99);
                neuron.activation = (neuron.activation + strength).min(1.0);
                neuron.last_used = crate::utils::now_millis(); // placeholder
            }
        }

        // Neurogenesis : création de nouveaux neurones si sagesse élevée
        if self.average_wisdom > 0.85 && self.neurons.len() < 512 {
            self.add_neuron(0.6);
            self.total_synapses += 1;
        }

        self.update_average_wisdom();
    }

    fn update_average_wisdom(&mut self) {
        if self.neurons.is_empty() { return; }
        let sum: f32 = self.neurons.values().map(|n| n.wisdom).sum();
        self.average_wisdom = sum / self.neurons.len() as f32;
    }

    pub fn get_stats(&self) -> (usize, f32, usize) {
        (self.neurons.len(), self.average_wisdom, self.total_synapses)
    }
}