use std::fmt::Display;

use indexmap::IndexMap;

pub const TOTAL_ITEMS: usize = 9;

pub const TOTAL_UNARY_ITEMS: usize = 7;

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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Item::NotAdreneline(not_adreneline) => match not_adreneline {
                NotAdreneline::UnaryItem(unary_item) => match unary_item {
                    UnaryItem::Remote => "Remote",
                    UnaryItem::Phone => "Phone",
                    UnaryItem::Inverter => "Inverter",
                    UnaryItem::MagnifyingGlass => "Magnifying Glass",
                    UnaryItem::Cigarettes => "Cigarettes",
                    UnaryItem::Handsaw => "Handsaw",
                    UnaryItem::Beer => "Beer",
                },
                NotAdreneline::Jammer => "Jammer",
            },
            Item::Adreneline => "Adreneline",
        };

        write!(f, "{}", str)
    }
}

pub static ALL_ITEMS: [Item; TOTAL_ITEMS] = [
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Remote)),
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Phone)),
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Inverter)),
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::MagnifyingGlass)),
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Cigarettes)),
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Handsaw)),
    Item::NotAdreneline(NotAdreneline::UnaryItem(UnaryItem::Beer)),
    Item::NotAdreneline(NotAdreneline::Jammer),
    Item::Adreneline,
];

pub fn initialize_item_count_map() -> IndexMap<Item, usize> {
    let mut map = IndexMap::with_capacity(TOTAL_ITEMS);
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
    assert!(map.len() == TOTAL_UNARY_ITEMS);
    map.insert(Item::Adreneline, 0);
    map.insert(Item::NotAdreneline(NotAdreneline::Jammer), 0);

    assert!(map.len() == TOTAL_ITEMS);
    map
}

pub fn global_item_limit(item: Item) -> usize {
    match item {
        Item::NotAdreneline(not_adreneline) => match not_adreneline {
            NotAdreneline::UnaryItem(unary_item) => match unary_item {
                UnaryItem::Remote => 2,
                UnaryItem::Phone
                | UnaryItem::Inverter
                | UnaryItem::MagnifyingGlass
                | UnaryItem::Cigarettes
                | UnaryItem::Handsaw
                | UnaryItem::Beer => 32,
            },
            NotAdreneline::Jammer => 1,
        },
        Item::Adreneline => 32,
    }
}

pub fn player_item_limit(item: Item) -> usize {
    match item {
        Item::NotAdreneline(not_adreneline) => match not_adreneline {
            NotAdreneline::UnaryItem(unary_item) => match unary_item {
                UnaryItem::Remote | UnaryItem::Cigarettes => 1,
                UnaryItem::MagnifyingGlass | UnaryItem::Handsaw => 2,
                UnaryItem::Inverter => 4,
                UnaryItem::Phone | UnaryItem::Beer => 8,
            },
            NotAdreneline::Jammer => 1,
        },
        Item::Adreneline => 4,
    }
}
