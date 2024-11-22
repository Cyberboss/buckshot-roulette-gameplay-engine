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
