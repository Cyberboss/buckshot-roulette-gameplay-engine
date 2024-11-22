use std::collections::HashMap;

// https://github.com/thecatontheceiling/buckshotroulette_multiplayer/blob/aed4aecb7fd7f6cec14a7bd17239e736039915c0/global%20scripts/MP_MatchCustomization.gd#L18
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryItem {
    Remote,
    Phone,
    Inverter,
    MagnifyingGlass,
    Cigarettes,
    Handsaw,
    Beer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotAdreneline {
    UnaryItem(UnaryItem),
    Jammer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    NotAdreneline(NotAdreneline),
    Adreneline,
}

pub fn initialize_item_count_map() -> HashMap<Item, usize> {
    let mut map = HashMap::with_capacity(9);
    map.insert(Item::Adreneline, 0);
    map.insert(Item::NotAdreneline(NotAdreneline::Jammer), 0);
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Remote)),
        0,
    );
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Phone)),
        0,
    );
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Inverter)),
        0,
    );
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::MagnifyingGlass)),
        0,
    );
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Cigarettes)),
        0,
    );
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Handsaw)),
        0,
    );
    map.insert(
        Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Beer)),
        0,
    );

    map
}
