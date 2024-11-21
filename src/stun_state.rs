#[derive(Debug, Clone, Copy)]
pub enum StunState {
    Unstunned,
    Stunned,
    Recovering,
}
