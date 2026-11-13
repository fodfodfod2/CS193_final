use mlua::Table;

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

impl Parsable for Planet {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Planet")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Planet")?;
        let sell_value: i16 = t
            .get("SELL_VALUE")
            .ok()
            .ok_or("Failed to parse sell value while parsing Planet")?;
        let hand_id: String = t
            .get("HAND_ID")
            .ok()
            .ok_or("Failed to parse hand id while parsing Planet")?;
        let hand = HandType::from_str(&hand_id);

        Ok(Planet {
            name,
            desc,
            sell_value,
            hand,
        })
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

impl Parsable for Tarot {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Tarot")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Tarot")?;
        let sell_value: i16 = t
            .get("SELL_VALUE")
            .ok()
            .ok_or("Failed to parse sell value while parsing Tarot")?;

        Ok(Tarot {
            name,
            desc,
            sell_value,
        })
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

impl Parsable for Spectral {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Spectral")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Spectral")?;
        let sell_value: i16 = t
            .get("SELL_VALUE")
            .ok()
            .ok_or("Failed to parse sell value while parsing Spectral")?;

        Ok(Spectral {
            name,
            desc,
            sell_value,
        })
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
