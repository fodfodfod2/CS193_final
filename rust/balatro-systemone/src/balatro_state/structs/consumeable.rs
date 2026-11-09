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

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        let sv = &self.sell_value;
        let h = &self.hand;
        write!(f, "{n} | upgrades {h} or can be sold for {sv}")
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

impl fmt::Display for Tarot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        let sv = &self.sell_value;
        write!(f, "{n} | use to {d} or can be sold for {sv}")
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

impl fmt::Display for Spectral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        let sv = &self.sell_value;
        write!(f, "{n} | use to {d} or can be sold for {sv}")
    }
}

pub enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
}

impl fmt::Display for Consumable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Consumable::Planet(p) => write!(f, "{p}"),
            Consumable::Tarot(t) => write!(f, "{t}"),
            Consumable::Spectral(s) => write!(f, "{s}"),
        }
    }
}
