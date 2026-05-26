pub struct KVCache {
    pub keys: Vec<Vec<Vec<f32>>>,
    pub values: Vec<Vec<Vec<f32>>>,
    pub num_layers: usize,
    pub num_kv_heads: usize,
    pub head_dim: usize,
    pub max_seq_len: usize,
    pub current_seq_len: usize,
}

impl KVCache {
    pub fn new(num_layers: usize, num_kv_heads: usize, head_dim: usize, max_seq_len: usize) -> Self {
        let head_size = num_kv_heads * head_dim;
        Self {
            keys: vec![vec![vec![0.0; head_size]; max_seq_len]; num_layers],
            values: vec![vec![vec![0.0; head_size]; max_seq_len]; num_layers],
            num_layers,
            num_kv_heads,
            head_dim,
            max_seq_len,
            current_seq_len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.current_seq_len = 0;
    }
}
