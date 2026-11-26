use mlua::Table;

use crate::{Action, balatro_state::SpectralType::Wraith};

use super::*;

/* This will represent the type of spectral card */
pub enum SpectralType {
    Familiar,
    Grim,
    Incantation,
    Talisman,
    Aura,
    Wraith,
    Sigil,
    Ouija,
    Ectoplasm,
    Immolate,
    Ankh,
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    TheSoul,
    BlackHole,
}

impl SpectralType {
    /* This will get the type of spectral from a string */
    pub fn from_str(s: &str) -> Self {
        match s {
            "Familiar" => Self::Familiar,
            "Grim" => Self::Grim,
            "Incantation" => Self::Incantation,
            "Talisman" => Self::Talisman,
            "Aura" => Self::Aura,
            "Wraith" => Self::Wraith,
            "Sigil" => Self::Sigil,
            "Ouija" => Self::Ouija,
            "Ectoplasm" => Self::Ectoplasm,
            "Immolate" => Self::Immolate,
            "Ankh" => Self::Ankh,
            "Deja Vu" => Self::DejaVu,
            "Hex" => Self::Hex,
            "Trance" => Self::Trance,
            "Medium" => Self::Medium,
            "Cryptid" => Self::Cryptid,
            "The Soul" => Self::TheSoul,
            "Black Hole" => Self::BlackHole,
            _ => panic!("Unknown spectral card name: {s:?}"),
        }
    }

    /* This will get the name of the spectral card */
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Familiar => "Familiar",
            Self::Grim => "Grim",
            Self::Incantation => "Incantation",
            Self::Talisman => "Talisman",
            Self::Aura => "Aura",
            Self::Wraith => "Wraith",
            Self::Sigil => "Sigil",
            Self::Ouija => "Ouija",
            Self::Ectoplasm => "Ectoplasm",
            Self::Immolate => "Immolate",
            Self::Ankh => "Ankh",
            Self::DejaVu => "Deja Vu",
            Self::Hex => "Hex",
            Self::Trance => "Trance",
            Self::Medium => "Medium",
            Self::Cryptid => "Cryptid",
            Self::TheSoul => "The Soul",
            Self::BlackHole => "Black Hole",
        }
    }

    /* This will get the desc of the ability of the spectral card */
    pub fn get_desc(&self) -> &'static str {
        match self {
            Self::Familiar => {
                "Destroy 1 random card in your hand, add 3 random Enhanced face cards to your hand"
            }
            Self::Grim => {
                "Destroy 1 random card in your hand, add 2 random Enhanced Aces to your hand"
            }
            Self::Incantation => {
                "Destroy 1 random card in your hand, add 4 random Enhanced numbered cards to your hand"
            }
            Self::Talisman => "Add a Gold Seal to 1 selected card in your hand",
            Self::Aura => "Add Foil, Holographic, or Polychrome effect to 1 selected card in hand",
            Self::Wraith => "Creates a random Rare Joker, sets money to $0",
            Self::Sigil => "Converts all cards in hand to a single random suit",
            Self::Ouija => "Converts all cards in hand to a single random rank, -1 hand size",
            Self::Ectoplasm => "Add Negative to a random Joker, -1 hand size",
            Self::Immolate => "Destroys 5 random cards in hand, gain $20",
            Self::Ankh => "Create a copy of a random Joker, destroy all other Jokers",
            Self::DejaVu => "Add a Red Seal to 1 selected card in your hand",
            Self::Hex => "Add Polychrome to a random Joker, destroy all other Jokers",
            Self::Trance => "Add a Blue Seal to 1 selected card in your hand",
            Self::Medium => "Add a Purple Seal to 1 selected card in your hand",
            Self::Cryptid => "Create 2 copies of 1 selected card in your hand",
            Self::TheSoul => "Creates a Legendary Joker (Must have room)",
            Self::BlackHole => "Upgrade every poker hand by 1 level",
        }
    }

    /* Can you currently play this card given this ctx? */
    pub fn can_play(&self, ctx: &Ctx) -> bool {
        match self {
            Self::Familiar => true,
            Self::Grim => true,
            Self::Incantation => true,
            Self::Talisman => true,
            Self::Aura => true,
            Self::Wraith => ctx.get_open_joker_slots() != 0,
            Self::Sigil => true,
            Self::Ouija => true,
            Self::Ectoplasm => ctx.has_open_joker_slot(),
            Self::Immolate => true,
            Self::Ankh => ctx.has_open_joker_slot(),
            Self::DejaVu => true,
            Self::Hex => ctx.has_open_joker_slot(),
            Self::Trance => true,
            Self::Medium => true,
            Self::Cryptid => true,
            Self::TheSoul => ctx.has_open_joker_slot(),
            Self::BlackHole => true,
        }
    }

    /* Get the (min, max) number of cards targeted by this card */
    pub fn get_targets(&self) -> (i8, i8) {
        match self {
            Self::Familiar => (0, 0),
            Self::Grim => (0, 0),
            Self::Incantation => (0, 0),
            Self::Talisman => (1, 1),
            Self::Aura => (1, 1),
            Self::Wraith => (0, 0),
            Self::Sigil => (0, 0),
            Self::Ouija => (0, 0),
            Self::Ectoplasm => (0, 0),
            Self::Immolate => (0, 0),
            Self::Ankh => (0, 0),
            Self::DejaVu => (1, 1),
            Self::Hex => (0, 0),
            Self::Trance => (1, 1),
            Self::Medium => (1, 1),
            Self::Cryptid => (0, 0),
            Self::TheSoul => (0, 0),
            Self::BlackHole => (0, 0),
        }
    }
}

/* This will represent a spectral card */
pub struct Spectral {
    spectral_type: SpectralType,
    sell_value: i16,
    cost: i16,
}

impl Spectral {
    /* This will get the name of the spectral card */
    pub fn get_name(&self) -> &'static str {
        self.spectral_type.get_name()
    }

    /* This will get the desc of the ability of the spectral card */
    pub fn get_desc(&self) -> &'static str {
        self.spectral_type.get_desc()
    }

    /* Get the sell value of the spectral card */
    pub fn get_sell_val(&self) -> i16 {
        self.sell_value
    }

    /* Can you play this card? */
    pub fn can_play(&self, ctx: &Ctx) -> bool {
        self.spectral_type.can_play(ctx)
    }

    /* Get the (min, max) number of cards targeted by this card */
    pub fn get_targets(&self) -> (i8, i8) {
        self.spectral_type.get_targets()
    }
}

impl Buyable for Spectral {
    fn get_item_name(&self) -> &str {
        self.get_name()
    }

    fn get_price(&self) -> i16 {
        self.cost
    }

    fn get_action(&self, i: i16) -> crate::Action {
        Action::BuyCard(i)
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
        let cost: i16 = t
            .get("COST")
            .ok()
            .ok_or("Failed to parse cost while parsing Spectral")?;

        Ok(Spectral {
            spectral_type: SpectralType::from_str(&name),
            sell_value,
            cost,
        })
    }
}

impl fmt::Display for Spectral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.get_name();
        let d = &self.get_desc();
        let sv = self.get_sell_val();

        write!(f, "{n} | use to {d}")
    }
}
