#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryItem {
    Remote,
    Phone,
    Inverter,
    MagnifyingGlass,
    Cigarettes,
    Knife,
    Beer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotAdreneline {
    UnaryItem(UnaryItem),
    Jammer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    NotAdreneline(NotAdreneline),
    Adreneline,
}
