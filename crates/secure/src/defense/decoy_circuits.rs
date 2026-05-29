// crates/secure/src/defense/decoy_circuits.rs
// =====================================================
// Decoy Circuits v5.1 — Strong Edition
// Faux Circuits DiamantRoots — Deception Layer
// SkyAInet × Nikola T369
// =====================================================

use rand::Rng;
use std::net::SocketAddr;
use tracing::{info, debug, warn};

use crate::crypto::roman_t369::{RomanT369, GematriaMode};
use crate::crypto::sha_fips::hkdf_sha256;

#[derive(Debug, Clone)]
pub struct DecoyCircuit {
    pub id: u32,
    pub fake_nodes: Vec<SocketAddr>,
    pub fake_shared_secrets: Vec<[u8; 32]>,
    pub created_at: u64,
    pub realism_score: f32, // 0.0 → 1.0
    pub latency_ms: u16,
    pub node_reputation: f32,
}

pub struct DecoyCircuitManager {
    decoys: Vec<DecoyCircuit>,
    max_decoys: usize,
    roman: RomanT369,
}

impl DecoyCircuitManager {
    pub fn new(max_decoys: usize) -> Self {
        let roman = RomanT369::new([0x42u8; 32], [0u8; 12], GematriaMode::Hyper256);

        Self {
            decoys: Vec::new(),
            max_decoys,
            roman,
        }
    }

    /// Génère des circuits leurres réalistes et crédibles
    pub fn generate_decoy_circuits(&mut self, count: usize) -> Vec<DecoyCircuit> {
        let mut new_decoys = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..count.min(self.max_decoys - self.decoys.len()) {
            let node_count = rng.gen_range(3..=6);
            let mut fake_nodes = Vec::new();
            let mut fake_secrets = Vec::new();

            for _ in 0..node_count {
                // Adresses plus réalistes (simule des nœuds réels)
                let ip_octet = rng.gen_range(10..=200);
                let port = rng.gen_range(40000..=65000);
                let addr: SocketAddr = format!("{}.{}.{}.{}:{}", 
                    ip_octet, rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), port)
                    .parse()
                    .unwrap();

                fake_nodes.push(addr);

                // Secret partagé plus crédible (dérivé via RomanT369)
                let mut base = [0u8; 32];
                rng.fill(&mut base);
                let secret = self.roman.encrypt(&base);
                let mut final_secret = [0u8; 32];
                hkdf_sha256(&secret, Some(b"DECOY"), b"shared-secret", &mut final_secret);
                fake_secrets.push(final_secret);
            }

            let realism = self.calculate_realism_score(node_count, &fake_nodes);

            let circuit = DecoyCircuit {
                id: rng.gen(),
                fake_nodes,
                fake_shared_secrets: fake_secrets,
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                realism_score: realism,
                latency_ms: rng.gen_range(45..=180),
                node_reputation: rng.gen_range(0.75..=0.96),
            };

            self.decoys.push(circuit.clone());
            new_decoys.push(circuit);
        }

        info!(
            "[DecoyCircuitManager] {} circuits leurres générés (total: {})",
            new_decoys.len(),
            self.decoys.len()
        );

        new_decoys
    }

    fn calculate_realism_score(&self, node_count: usize, nodes: &[SocketAddr]) -> f32 {
        let mut score = 0.75;

        // Bonus pour variété de ports
        let unique_ports: std::collections::HashSet<_> = nodes.iter().map(|a| a.port()).collect();
        if unique_ports.len() > 2 {
            score += 0.08;
        }

        // Bonus pour nombre de nœuds cohérent
        if node_count >= 4 {
            score += 0.07;
        }

        score.min(0.98)
    }

    /// Retourne un circuit leurre aléatoire
    pub fn get_random_decoy(&self) -> Option<&DecoyCircuit> {
        if self.decoys.is_empty() {
            return None;
        }
        let idx = rand::thread_rng().gen_range(0..self.decoys.len());
        Some(&self.decoys[idx])
    }

    /// Retourne plusieurs leurres pour injection dans un vrai circuit
    pub fn get_decoy_batch(&self, count: usize) -> Vec<&DecoyCircuit> {
        let mut rng = rand::thread_rng();
        let mut result = Vec::new();

        for _ in 0..count.min(self.decoys.len()) {
            let idx = rng.gen_range(0..self.decoys.len());
            result.push(&self.decoys[idx]);
        }
        result
    }

    pub fn cleanup_old_decoys(&mut self, max_age_seconds: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let before = self.decoys.len();
        self.decoys.retain(|d| now - d.created_at < max_age_seconds);

        if before != self.decoys.len() {
            debug!(
                "[DecoyCircuitManager] {} leurres expirés nettoyés (restants: {})",
                before - self.decoys.len(),
                self.decoys.len()
            );
        }
    }

    pub fn total_decoys(&self) -> usize {
        self.decoys.len()
    }
}