use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Item {
    Remote,
    Phone,
    Inverter,
    MagnifyingGlass,
    Cigarettes,
    Adreneline,
    Knife,
    Beer,
    Handcuffs,
    Medication,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum InvalidItemUseError {
    #[error("The item is not present in the player's inventory")]
    NoItem,
    #[error("Cannot use adreneline to use adreneline")]
    DoubleAdreneline,
    #[error("Player is recovering from previous stun and cannot be restunned")]
    DoubleStun,
    #[error("Shotgun is already sawn")]
    DoubleSaw,
    #[error("Player targeted by stun is dead")]
    InvalidStunTarget,
}
