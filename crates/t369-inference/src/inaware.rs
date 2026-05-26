pub struct InAware;
impl InAware {
    pub fn new() -> Self { Self }
    pub fn generate_with_awareness(&self, _l: &[f32], _p: &str, _m: usize) -> crate::model::AwareResponse {
        crate::model::AwareResponse { text: "test".to_string(), confidence: 0.8, uncertainty: 0.2, entropy: 1.0, tokens_used: 10 }
    }
}
