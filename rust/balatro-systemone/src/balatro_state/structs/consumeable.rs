use super::*;

/* This will represent a Planet card */
pub struct Planet {
    name: String,
    desc: String,
    sell_value: i16,
    hand: HandType,
}

impl Planet {
    pub fn new(name: String, desc: String, sell_value: i16, hand: HandType) -> Self {
        Planet {
            name,
            desc,
            sell_value,
            hand,
        }
    }
}

/* This will represent a Tarot card */
pub struct Tarot {
    name: String,
    desc: String,
    sell_value: i16,
}

impl Tarot {
    pub fn new(name: String, desc: String, sell_value: i16) -> Self {
        Tarot {
            name,
            desc,
            sell_value,
        }
    }
}

/* This will represent a spectral card */
pub struct Spectral {
    name: String,
    desc: String,
    sell_value: i16,
}

impl Spectral {
    pub fn new(name: String, desc: String, sell_value: i16) -> Self {
        Spectral {
            name,
            desc,
            sell_value,
        }
    }
}

pub enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
    Other,
}
