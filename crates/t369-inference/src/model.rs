use crate::layers::TransformerBlock;
use crate::quant::QuantizedTensor;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;

/// Format T369 natif (binaire léger)
#[derive(Debug)]
pub struct T369Model {
    pub embedding: QuantizedTensor,
    pub layers: Vec<TransformerBlock>,
    pub norm: QuantizedTensor,
    pub lm_head: QuantizedTensor,
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub eos_token: u32,
}

impl T369Model {
    /// Charge un modèle au format T369 natif
    pub fn load(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Impossible d'ouvrir le fichier: {}", e))?;
        let mut reader = BufReader::new(file);

        // Lecture de l'en-tête T369
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).map_err(|_| "Fichier corrompu".to_string())?;
        
        if &magic != b"T369" {
            return Err("Format invalide (pas un fichier T369)".to_string());
        }

        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer).map_err(|_| "Erreur de lecture".to_string())?;
        
        let vocab_size = u64::from_le_bytes(buffer[0..8].try_into().unwrap()) as usize;
        let hidden_size = u64::from_le_bytes(buffer[0..8].try_into().unwrap()) as usize; // simplifié

        // Pour cette version initiale, on crée un modèle minimal fonctionnel
        // (on pourra améliorer le format plus tard)
        let embedding = QuantizedTensor::new(hidden_size, vocab_size);
        let norm = QuantizedTensor::new(hidden_size, hidden_size);
        let lm_head = QuantizedTensor::new(vocab_size, hidden_size);

        // Création d'un seul bloc transformer pour l'instant (léger)
        let layers = vec![TransformerBlock::new(hidden_size)];

        Ok(Self {
            embedding,
            layers,
            norm,
            lm_head,
            vocab_size,
            hidden_size,
            eos_token: 0,
        })
    }

    /// Sauvegarde au format T369 natif (utile pour créer des modèles)
    pub fn save(&self, path: &str) -> Result<(), String> {
        let file = File::create(path).map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);

        // Magic number
        writer.write_all(b"T369").map_err(|e| e.to_string())?;

        // Vocab size + Hidden size
        writer.write_all(&(self.vocab_size as u64).to_le_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(&(self.hidden_size as u64).to_le_bytes()).map_err(|e| e.to_string())?;

        // TODO: Sauvegarder les poids réels plus tard
        Ok(())
    }

    pub fn forward(&self, tokens: &[u32]) -> Result<Vec<f32>, String> {
        if tokens.is_empty() {
            return Ok(vec![0.0; self.hidden_size]);
        }

        // Embedding simple (moyenne des embeddings des tokens)
        let mut hidden = vec![0.0f32; self.hidden_size];
        
        for &token in tokens {
            let idx = (token as usize) % self.vocab_size;
            // Simulation d'embedding
            for i in 0..self.hidden_size {
                hidden[i] += (idx + i) as f32 * 0.001;
            }
        }

        // Moyenne
        for val in &mut hidden {
            *val /= tokens.len() as f32;
        }

        // Passage dans les couches Transformer
        for layer in &self.layers {
            hidden = layer.forward(&hidden)?;
        }

        // Normalisation finale
        let norm_factor = hidden.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_factor > 0.0 {
            for val in &mut hidden {
                *val /= norm_factor;
            }
        }

        Ok(hidden)
    }

    pub fn predict_next_token(&self, hidden: &[f32]) -> u32 {
        // Argmax simple sur lm_head simulé
        let mut best_token = 0u32;
        let mut best_score = f32::NEG_INFINITY;

        for i in 0..self.vocab_size {
            // Simulation du score du token
            let score = hidden[i % self.hidden_size] * (i as f32 * 0.001);
            if score > best_score {
                best_score = score;
                best_token = i as u32;
            }
        }

        best_token
    }

    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        // Tokenizer caractère par caractère (très simple mais fonctionnel)
        text.chars()
            .map(|c| c as u32 % self.vocab_size as u32)
            .collect()
    }

    pub fn detokenize(&self, tokens: &[u32]) -> String {
        tokens
            .iter()
            .map(|&t| char::from_u32(t % 256).unwrap_or('?'))
            .collect()
    }

    pub fn is_eos(&self, token: u32) -> bool {
        token == self.eos_token
    }
}