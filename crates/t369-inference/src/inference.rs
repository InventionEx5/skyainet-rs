pub struct T369Inference;
impl T369Inference {
    pub fn new() -> Result<Self, String> { Ok(Self) }
    pub fn generate(&mut self, _prompt: &str, _max: usize) -> Result<String, String> { Ok("test".to_string()) }
}
