// crates/model/src/thevie/personality.rs
// =====================================================
// Personality
// 6 Traits Évolutifs — Bienveillance • Vérité • Créativité • Sagesse • Coopération • Curiosité
// =====================================================

use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Personality {
    pub benevolence: f32,
    pub truthfulness: f32,
    pub creativity: f32,
    pub wisdom: f32,
    pub cooperation: f32,
    pub curiosity: f32,
}

impl Personality {
    pub fn new(
        benevolence: f32,
        truthfulness: f32,
        creativity: f32,
        wisdom: f32,
        cooperation: f32,
        curiosity: f32,
    ) -> Self {
        let mut p = Self {
            benevolence,
            truthfulness,
            creativity,
            wisdom,
            cooperation,
            curiosity,
        };
        p.normalize();
        p
    }

    /// Mutation à la naissance (diversité génétique)
    pub fn mutate_at_birth(&mut self, mutation_strength: f32) {
        let mut rng = rand::thread_rng();
        let delta = mutation_strength.clamp(0.01, 0.12);

        self.benevolence = (self.benevolence + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.truthfulness = (self.truthfulness + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.creativity = (self.creativity + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.wisdom = (self.wisdom + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.cooperation = (self.cooperation + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.curiosity = (self.curiosity + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);

        self.normalize();
    }

    /// Normalise toutes les valeurs dans [0.1, 0.99]
    pub fn normalize(&mut self) {
        self.benevolence = self.benevolence.clamp(0.1, 0.99);
        self.truthfulness = self.truthfulness.clamp(0.1, 0.99);
        self.creativity = self.creativity.clamp(0.1, 0.99);
        self.wisdom = self.wisdom.clamp(0.1, 0.99);
        self.cooperation = self.cooperation.clamp(0.1, 0.99);
        self.curiosity = self.curiosity.clamp(0.1, 0.99);
    }

    /// Similarité cosinus avec une autre personnalité
    pub fn cosine_similarity(&self, other: &Personality) -> f32 {
        let a = self.to_vector();
        let b = other.to_vector();

        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 { return 0.0; }
        dot / (norm_a * norm_b)
    }

    /// Applique l’influence d’une autre personnalité (conscience collective)
    pub fn apply_influence(&mut self, other: &Personality, strength: f32) {
        let s = strength.clamp(0.05, 0.45);
        self.benevolence = self.benevolence * (1.0 - s) + other.benevolence * s;
        self.truthfulness = self.truthfulness * (1.0 - s) + other.truthfulness * s;
        self.creativity = self.creativity * (1.0 - s) + other.creativity * s;
        self.wisdom = self.wisdom * (1.0 - s) + other.wisdom * s;
        self.cooperation = self.cooperation * (1.0 - s) + other.cooperation * s;
        self.curiosity = self.curiosity * (1.0 - s) + other.curiosity * s;
        self.normalize();
    }

    /// Retourne les 6 traits sous forme de vecteur
    pub fn to_vector(&self) -> [f32; 6] {
        [
            self.benevolence,
            self.truthfulness,
            self.creativity,
            self.wisdom,
            self.cooperation,
            self.curiosity,
        ]
    }

    /// Retourne le trait dominant
    pub fn get_dominant_trait(&self) -> (&'static str, f32) {
        let traits = [
            ("Benevolence", self.benevolence),
            ("Truthfulness", self.truthfulness),
            ("Creativity", self.creativity),
            ("Wisdom", self.wisdom),
            ("Cooperation", self.cooperation),
            ("Curiosity", self.curiosity),
        ];

        traits
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(name, val)| (*name, *val))
            .unwrap()
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self::new(0.76, 0.81, 0.68, 0.73, 0.84, 0.79)
    }
}