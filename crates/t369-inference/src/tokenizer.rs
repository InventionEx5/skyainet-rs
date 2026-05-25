pub struct SimpleTokenizer {
    vocab_size: usize,
}

impl SimpleTokenizer {
    pub fn new(vocab_size: usize) -> Self {
        Self { vocab_size }
    }

    pub fn encode(&self, text: &str) -> Vec<u32> {
        text.chars()
            .map(|c| (c as u32) % self.vocab_size as u32)
            .collect()
    }

    pub fn decode(&self, tokens: &[u32]) -> String {
        tokens.iter()
            .map(|&t| char::from_u32(t % 256).unwrap_or('?'))
            .collect()
    }
}