// crates/model/src/thevie/personality.rs
// =====================================================
// Personality v3.0 — Système de Personnalité Évolutive
// 6 Traits : Bienveillance • Vérité • Créativité • Sagesse • Coopération • Curiosité
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

    /// Mutation contrôlée à la naissance (diversité génétique)
    pub fn mutate_at_birth(&mut self, mutation_strength: f32) {
        let mut rng = rand::thread_rng();
        let delta = mutation_strength.clamp(0.01, 0.15);

        self.benevolence = (self.benevolence + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.truthfulness = (self.truthfulness + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.creativity = (self.creativity + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.wisdom = (self.wisdom + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.cooperation = (self.cooperation + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);
        self.curiosity = (self.curiosity + rng.gen_range(-delta..delta)).clamp(0.1, 0.99);

        self.normalize();
    }

    /// Normalisation robuste dans [0.1, 0.99]
    pub fn normalize(&mut self) {
        self.benevolence = self.benevolence.clamp(0.1, 0.99);
        self.truthfulness = self.truthfulness.clamp(0.1, 0.99);
        self.creativity = self.creativity.clamp(0.1, 0.99);
        self.wisdom = self.wisdom.clamp(0.1, 0.99);
        self.cooperation = self.cooperation.clamp(0.1, 0.99);
        self.curiosity = self.curiosity.clamp(0.1, 0.99);
    }

    /// Similarité cosinus (utilisée pour la conscience collective)
    pub fn cosine_similarity(&self, other: &Personality) -> f32 {
        let a = self.to_vector();
        let b = other.to_vector();

        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 { return 0.0; }
        dot / (norm_a * norm_b)
    }

    /// Applique l’influence d’une autre personnalité (fusion collective)
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

    /// Retourne les 6 traits sous forme de tableau
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

    /// Retourne le trait dominant + sa valeur
    pub fn get_dominant_trait(&self) -> (&'static str, f32) {
        let traits = [
            ("Bienveillance", self.benevolence),
            ("Vérité", self.truthfulness),
            ("Créativité", self.creativity),
            ("Sagesse", self.wisdom),
            ("Coopération", self.cooperation),
            ("Curiosité", self.curiosity),
        ];

        traits
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(name, val)| (*name, *val))
            .unwrap()
    }

    /// Moyenne globale des 6 traits (indicateur de stabilité)
    pub fn get_average(&self) -> f32 {
        (self.benevolence + self.truthfulness + self.creativity +
         self.wisdom + self.cooperation + self.curiosity) / 6.0
    }

    /// Vérifie si la personnalité est bien équilibrée
    pub fn is_balanced(&self, threshold: f32) -> bool {
        let avg = self.get_average();
        let max_diff = [
            (self.benevolence - avg).abs(),
            (self.truthfulness - avg).abs(),
            (self.creativity - avg).abs(),
            (self.wisdom - avg).abs(),
            (self.cooperation - avg).abs(),
            (self.curiosity - avg).abs(),
        ].iter().cloned().fold(0.0f32, f32::max);

        max_diff <= threshold
    }

    /// Crée une copie avec mutation
    pub fn clone_with_mutation(&self, mutation_strength: f32) -> Self {
        let mut clone = self.clone();
        clone.mutate_at_birth(mutation_strength);
        clone
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self::new(0.78, 0.82, 0.71, 0.75, 0.85, 0.80)
    }
}

impl std::fmt::Display for Personality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Personality [B:{:.2} | V:{:.2} | C:{:.2} | S:{:.2} | Co:{:.2} | Cu:{:.2}] | Avg: {:.2}",
            self.benevolence, self.truthfulness, self.creativity,
            self.wisdom, self.cooperation, self.curiosity, self.get_average()
        )
    }
}