// crates/secure/src/utils/markov.rs
// =====================================================
// Markov Chain v6.1 — Stéganographie Textuelle Intelligente
// Compatible Contact v6.2 + DID + RomanT369
// SkyAInet × Nikola T369
// =====================================================

use std::collections::HashMap;
use rand::Rng;
use tracing::{debug, warn};
use thiserror::Error;

use crate::contacts::contact::Contact;

#[derive(Error, Debug)]
pub enum MarkovError {
    #[error("Not enough training data")]
    InsufficientData,
    #[error("Start word not found in chain")]
    StartWordNotFound,
}

pub struct MarkovChain {
    transitions: HashMap<String, HashMap<String, u32>>,
    total_transitions: usize,
}

impl MarkovChain {
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            total_transitions: 0,
        }
    }

    /// Entraîne la chaîne à partir d’un texte
    pub fn train(&mut self, text: &str) {
        let words: Vec<&str> = text.split_whitespace().collect();
        
        for i in 0..words.len().saturating_sub(1) {
            let current = words[i].to_string();
            let next = words[i + 1].to_string();

            self.transitions
                .entry(current)
                .or_default()
                .entry(next)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            self.total_transitions += 1;
        }

        debug!(
            "[MarkovChain] Entraînement terminé — {} transitions",
            self.total_transitions
        );
    }

    /// Entraîne à partir des notes d’un Contact (DID-friendly)
    pub fn train_from_contact(&mut self, contact: &Contact) {
        if let Some(notes) = &contact.notes {
            self.train(notes);
            debug!("[MarkovChain] Entraîné depuis les notes du contact {}", contact.name);
        }
    }

    /// Génère du texte de manière probabiliste (beaucoup plus naturel)
    pub fn generate(&self, start: &str, length: usize) -> Result<String, MarkovError> {
        if self.transitions.is_empty() {
            return Err(MarkovError::InsufficientData);
        }

        let mut current = start.to_string();
        let mut result = vec![current.clone()];
        let mut rng = rand::thread_rng();

        for _ in 0..length {
            if let Some(next_words) = self.transitions.get(&current) {
                // Sélection pondérée (plus réaliste que "toujours le plus fréquent")
                let total: u32 = next_words.values().sum();
                let mut r = rng.gen_range(0..total);
                
                let mut chosen = None;
                for (word, &count) in next_words {
                    if r < count {
                        chosen = Some(word.clone());
                        break;
                    }
                    r -= count;
                }

                if let Some(next) = chosen {
                    result.push(next.clone());
                    current = next;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if result.len() == 1 {
            return Err(MarkovError::StartWordNotFound);
        }

        Ok(result.join(" "))
    }

    /// Génère du texte à partir d’un mot aléatoire du corpus
    pub fn generate_random(&self, length: usize) -> Result<String, MarkovError> {
        if self.transitions.is_empty() {
            return Err(MarkovError::InsufficientData);
        }

        let start = self.transitions.keys().next().unwrap().clone();
        self.generate(&start, length)
    }

    pub fn is_trained(&self) -> bool {
        !self.transitions.is_empty()
    }
}

impl Default for MarkovChain {
    fn default() -> Self {
        Self::new()
    }
}