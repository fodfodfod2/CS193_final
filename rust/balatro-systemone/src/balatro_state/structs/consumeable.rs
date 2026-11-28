use mlua::Table;

use super::*;

/*
 * This can represent any Consumable, either a planet, tarot,
 * or spectral card
 */
pub enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
}

impl Consumable {
    /* Get the name of the Consumable */
    pub fn get_name(&self) -> &str {
        match self {
            Consumable::Tarot(t) => t.get_name(),
            Consumable::Planet(p) => p.get_name(),
            Consumable::Spectral(s) => s.get_name(),
        }
    }

    /* Get the name of the type of the Consumable */
    pub fn get_type_name(&self) -> &str {
        match self {
            Consumable::Tarot(_) => "Tarot",
            Consumable::Planet(_) => "Planet",
            Consumable::Spectral(_) => "Spectral",
        }
    }

    /* Get the desc of the Consumable */
    pub fn get_desc(&self) -> &str {
        match self {
            Consumable::Tarot(t) => t.get_desc(),
            Consumable::Planet(p) => p.get_desc(),
            Consumable::Spectral(s) => s.get_desc(),
        }
    }

    /* get the sell value of a consumable */
    pub fn get_sell_value(&self) -> i16 {
        match self {
            Consumable::Tarot(t) => t.get_sell_val(),
            Consumable::Planet(p) => p.get_sell_val(),
            Consumable::Spectral(s) => s.get_sell_val(),
        }
    }

    /* get the whether you can play of a consumable */
    pub fn can_play(&self, ctx: &Ctx) -> bool {
        match self {
            Consumable::Tarot(t) => t.can_play(ctx),
            Consumable::Planet(p) => true,
            Consumable::Spectral(s) => s.can_play(ctx),
        }
    }


    /*
     * get the (min, max) amt of cards targeted by a
     * consumable
     */
    pub fn get_targets(&self) -> (i8, i8) {
        match self {
            Consumable::Tarot(t) => t.get_targets(),
            Consumable::Planet(p) => (0,0),
            Consumable::Spectral(s) => s.get_targets(),
        }
    }
}

impl Parsable for Consumable {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let type_str: String = t
            .get("TYPE")
            .ok()
            .ok_or("Failed to read type from Consumable")?;
        match type_str.as_str() {
            "TAROT" => Ok(Consumable::Tarot(Tarot::parse(t).unwrap())),
            "PLANET" => Ok(Consumable::Planet(Planet::parse(t).unwrap())),
            "SPECTRAL" => Ok(Consumable::Spectral(Spectral::parse(t).unwrap())),
            s => panic!("Expected consumable type string, got {s}"),
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
