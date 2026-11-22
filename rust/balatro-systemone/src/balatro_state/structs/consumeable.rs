use mlua::Table;

use super::*;

pub enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
}

impl Consumable {
    pub fn get_name(&self) -> &str {
        match self {
            Consumable::Tarot(t) => &t.get_name(),
            Consumable::Planet(p) => &p.get_name(),
            Consumable::Spectral(s) => &s.get_name(),
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
            Consumable::Tarot(t) => &t.get_desc(),
            Consumable::Planet(p) => &p.get_desc(),
            Consumable::Spectral(s) => &s.get_desc(),
        }
    }

    pub fn get_sell_value(&self) -> i16 {
        match self {
            Consumable::Tarot(t) => t.get_sell_val(),
            Consumable::Planet(p) => p.get_sell_val(),
            Consumable::Spectral(s) => s.get_sell_val(),
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
