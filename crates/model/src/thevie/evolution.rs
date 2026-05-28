// crates/model/src/thevie/evolution/evolution.rs
// =====================================================
// Evolution Engine v2.0 — Version Intensifiée
// Moteur d’Évolution Avancé + Émergence + Mutation
// =====================================================

use serde::{Serialize, Deserialize};
use tracing::{debug, info};

use super::neural_mesh::Personality;
use super::collective_consciousness::CollectiveConsciousness;

/// Moteur d’évolution principal (version intensifiée)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionEngine {
    pub global_evolution_rate: f32,
    pub total_evolutions: u64,
    pub last_global_tick: u64,
    pub mutation_rate: f32,           // Nouveau : Taux de mutation
    pub collective_boost: f32,        // Nouveau : Bonus quand la sagesse collective est haute
}

impl EvolutionEngine {
    pub fn new() -> Self {
        Self {
            global_evolution_rate: 0.042,
            total_evolutions: 0,
            last_global_tick: Self::now_millis(),
            mutation_rate: 0.012,
            collective_boost: 1.0,
        }
    }

    /// Évolution avancée de la personnalité (après chaque requête)
    pub fn evolve_personality(&mut self, personality: &mut Personality, quality: f32, collective_wisdom: f32) {
        // Ajustement dynamique du taux selon la qualité et la sagesse collective
        let dynamic_rate = self.global_evolution_rate * quality.clamp(0.5, 1.4) * self.collective_boost;

        // Évolution multi-dimensionnelle
        personality.wisdom = (personality.wisdom + dynamic_rate * 0.65).min(0.99);
        personality.truthfulness = (personality.truthfulness + dynamic_rate * 0.45).min(0.99);
        personality.cooperation = (personality.cooperation + dynamic_rate * 0.38).min(0.99);
        personality.curiosity = (personality.curiosity + dynamic_rate * 0.32).min(0.99);
        personality.creativity = (personality.creativity + dynamic_rate * 0.28).min(0.99);
        personality.ethics = (personality.ethics + dynamic_rate * 0.22).min(0.99);

        // Mutation légère (évolution imprévisible)
        if rand::random::<f32>() < self.mutation_rate {
            personality.creativity = (personality.creativity + 0.04).min(0.99);
            debug!(" Mutation créative appliquée !");
        }

        // Équilibre : trop de sagesse peut légèrement réduire la créativité
        if personality.wisdom > 0.90 {
            personality.creativity = (personality.creativity - 0.006).max(0.40);
        }

        // Bonus collectif
        if collective_wisdom > 0.85 {
            personality.wisdom = (personality.wisdom + 0.008).min(0.99);
            self.collective_boost = 1.15;
        } else {
            self.collective_boost = 1.0;
        }

        self.total_evolutions += 1;

        debug!(
            " Évolution personnalité | Qualité: {:.2} | Sagesse: {:.3} | Créativité: {:.3}",
            quality, personality.wisdom, personality.creativity
        );
    }

    /// Évolution d’un expert MoE (plus agressive)
    pub fn evolve_expert(&mut self, competence: &mut f32, current_level: &mut u32) {
        *competence = (*competence + 0.065).min(2.2);

        if *competence > 1.15 && *current_level < 12 {
            *current_level += 1;
            *competence = 0.78;
            info!(" Expert Level Up → Niveau {}", *current_level);
        }
    }

    /// Tick global intensifié (appelé régulièrement)
    pub fn global_tick(&mut self, collective: &mut CollectiveConsciousness) {
        if collective.global_wisdom < 0.94 {
            collective.global_wisdom = (collective.global_wisdom + 0.009).min(0.99);
        }

        // Accélération de l’émergence
        collective.emergent_intelligence = (collective.emergent_intelligence + 0.003).min(0.99);

        if collective.coherence_level > 0.70 {
            collective.coherence_level *= 0.985;
        }

        self.last_global_tick = Self::now_millis();
        debug!(" Tick global intensifié | Sagesse: {:.3} | Émergence: {:.3}", 
               collective.global_wisdom, collective.emergent_intelligence);
    }

    /// Évolution en batch sur plusieurs personnalités (plus efficace)
    pub fn batch_evolve_neurons(&mut self, personalities: &mut [Personality], collective_wisdom: f32) {
        for p in personalities.iter_mut() {
            p.wisdom = (p.wisdom + 0.006).min(0.99);
            p.cooperation = (p.cooperation + 0.004).min(0.99);
            p.creativity = (p.creativity + 0.003).min(0.99);

            if collective_wisdom > 0.82 {
                p.ethics = (p.ethics + 0.005).min(0.99);
            }
        }
        self.total_evolutions += personalities.len() as u64;
    }

    /// Évolution croisée (crossover entre deux personnalités)
    pub fn crossover(&mut self, p1: &mut Personality, p2: &mut Personality) {
        let avg_wisdom = (p1.wisdom + p2.wisdom) / 2.0;
        let avg_creativity = (p1.creativity + p2.creativity) / 2.0;

        p1.wisdom = (p1.wisdom * 0.6 + avg_wisdom * 0.4).min(0.99);
        p2.wisdom = (p2.wisdom * 0.6 + avg_wisdom * 0.4).min(0.99);

        p1.creativity = (p1.creativity * 0.55 + avg_creativity * 0.45).min(0.99);
        p2.creativity = (p2.creativity * 0.55 + avg_creativity * 0.45).min(0.99);

        self.total_evolutions += 2;
        debug!(" Crossover entre deux personnalités effectué");
    }

    fn now_millis() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}