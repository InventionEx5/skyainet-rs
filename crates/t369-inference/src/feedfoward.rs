pub struct FeedForward {
    hidden_size: usize,
}

impl FeedForward {
    pub fn new(hidden_size: usize) -> Self {
        Self { hidden_size }
    }

    pub fn forward(&self, hidden: &[f32]) -> Result<Vec<f32>, String> {
        // MLP simplifi� (expansion + contraction)
        let mut expanded = Vec::with_capacity(hidden.len() * 2);

        for &val in hidden {
            expanded.push(val * 1.5);
            expanded.push(val * 0.8);
        }

        let mut output = Vec::with_capacity(hidden.len());
        for i in 0..hidden.len() {
            let sum = expanded[i * 2] + expanded[i * 2 + 1];
            output.push(sum * 0.5);
        }

        Ok(output)
    }
}