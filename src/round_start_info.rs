use crate::round::Shell;

#[derive(Debug, Clone)]
pub struct RoundStartInfo {
    max_health: u8,
    initial_blank_rounds: u8,
    initial_live_rounds: u8,
    new_items: u8,
    multiplayer: bool,
}

impl RoundStartInfo {
    pub fn new(multiplayer: bool) -> Self {
        todo!("Impl new<RoundStartInfo>")
    }

    pub fn max_health(&self) -> u8 {
        self.max_health
    }

    pub fn generate_shells(&self) -> Vec<Shell> {
        todo!("generate shells")
    }
}
