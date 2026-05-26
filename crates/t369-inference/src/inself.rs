pub struct InSelf { pub is_evolving: bool }
impl InSelf {
    pub fn new() -> Self { Self { is_evolving: true } }
    pub fn evolve_self(&mut self) {}
}
