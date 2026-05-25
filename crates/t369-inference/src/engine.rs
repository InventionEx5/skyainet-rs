use crate::model::T369Model;
use crate::roman_diffusion::RomanDiffusion;
use crate::neural_mesh::NeuralMesh;

pub struct T369InferenceEngine {
    model: T369Model,
    roman_diffusion: RomanDiffusion,
    neural_mesh: NeuralMesh,
    max_context: usize,
}

impl T369InferenceEngine {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let model = T369Model::load(model_path)?;
        let roman_diffusion = RomanDiffusion::new();
        let neural_mesh = NeuralMesh::new();

        Ok(Self {
            model,
            roman_diffusion,
            neural_mesh,
            max_context: 4096,
        })
    }

    pub fn generate(&self, prompt: &str, max_new_tokens: usize) -> Result<String, String> {
        let mut tokens = self.model.tokenize(prompt);
        
        for _ in 0..max_new_tokens {
            if tokens.len() >= self.max_context {
                break;
            }

            // Forward pass avec Roman Neural Inference
            let mut hidden = self.model.forward(&tokens)?;
            
            // === La magie : Diffusion Romaine ===
            hidden = self.roman_diffusion.apply(&hidden, tokens.len());
            
            // Intégration Neural Mesh (sagesse collective)
            hidden = self.neural_mesh.process(hidden);
            
            let next_token = self.model.predict_next_token(&hidden);
            tokens.push(next_token);

            if self.model.is_eos(next_token) {
                break;
            }
        }

        Ok(self.model.detokenize(&tokens))
    }
}