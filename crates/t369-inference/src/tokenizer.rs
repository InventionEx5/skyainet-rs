use std::collections::HashMap;

pub struct BpeTokenizer {
    pub vocab: HashMap<String, u32>,
    pub id_to_token: Vec<String>,
}

impl BpeTokenizer {
    pub fn new() -> Self {
        Self {
            vocab: HashMap::new(),
            id_to_token: Vec::new(),
        }
    }

    pub fn encode(&self, _text: &str) -> Vec<u32> {
        vec![0]
    }

    pub fn decode(&self, _tokens: &[u32]) -> String {
        "test".to_string()
    }
}
