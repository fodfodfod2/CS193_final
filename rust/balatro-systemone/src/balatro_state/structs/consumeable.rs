use mlua::Table;

use super::*;

/* This will represent a Planet card */
pub struct Planet {
    pub name: String,
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
        let sv = self.sell_value;
        let h = &self.hand;

        write!(f, "{n} | upgrades {h}")
    }
}

/* This will represent a Tarot card */
pub struct Tarot {
    pub name: String,
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
        let sv = self.sell_value;

        write!(f, "{n} | use to {d}")
    }
}

/* This will represent a spectral card */
pub struct Spectral {
    pub name: String,
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
        let sv = self.sell_value;

        write!(f, "{n} | use to {d}")
    }
}

pub enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
}

impl Consumable {
    pub fn get_name(&self) -> &str {
        match self {
            Consumable::Tarot(t) => &t.name,
            Consumable::Planet(p) => &p.name,
            Consumable::Spectral(s) => &s.name,
        }
    }

    pub fn get_type_name(&self) -> &str {
        match self {
            Consumable::Tarot(_) => "Tarot",
            Consumable::Planet(_) => "Planet",
            Consumable::Spectral(_) => "Spectral",
        }
    }

    pub fn get_desc(&self) -> &str {
        match self {
            Consumable::Tarot(t) => &t.desc,
            Consumable::Planet(p) => &p.desc,
            Consumable::Spectral(s) => &s.desc,
        }
    }

    pub fn get_sell_value(&self) -> i16 {
        match self {
            Consumable::Tarot(t) => t.sell_value,
            Consumable::Planet(p) => p.sell_value,
            Consumable::Spectral(s) => s.sell_value,
        }
    }
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
