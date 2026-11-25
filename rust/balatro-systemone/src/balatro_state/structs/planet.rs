use mlua::Table;

use super::*;

pub enum PlanetType {
    Pluto,
    Mercury,
    Uranus,
    Venus,
    Saturn,
    Jupiter,
    Earth,
    Mars,
    Neptune,
    PlanetX,
    Ceres,
    Eris,
}

impl PlanetType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Pluto" => Self::Pluto,
            "Mercury" => Self::Mercury,
            "Uranus" => Self::Uranus,
            "Venus" => Self::Venus,
            "Saturn" => Self::Saturn,
            "Jupiter" => Self::Jupiter,
            "Earth" => Self::Earth,
            "Mars" => Self::Mars,
            "Neptune" => Self::Neptune,
            "Planet X" => Self::PlanetX,
            "Ceres" => Self::Ceres,
            "Eris" => Self::Eris,
            _ => panic!("Unknown planet card name: {s:?}"),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Pluto => "Pluto",
            Self::Mercury => "Mercury",
            Self::Uranus => "Uranus",
            Self::Venus => "Venus",
            Self::Saturn => "Saturn",
            Self::Jupiter => "Jupiter",
            Self::Earth => "Earth",
            Self::Mars => "Mars",
            Self::Neptune => "Neptune",
            Self::PlanetX => "Planet X",
            Self::Ceres => "Ceres",
            Self::Eris => "Eris",
        }
    }

    pub fn get_desc(&self) -> &'static str {
        match self {
            Self::Pluto => "Level up High Card, +1 Mult and +10 Chips",
            Self::Mercury => "Level up Pair, +1 Mult and +15 Chips",
            Self::Uranus => "Level up Two Pair, +1 Mult and +20 Chips",
            Self::Venus => "Level up Three of a Kind, +2 Mult and +20 Chips",
            Self::Saturn => "Level up Straight, +3 Mult and +30 Chips",
            Self::Jupiter => "Level up Flush, +2 Mult and +15 Chips",
            Self::Earth => "Level up Full House, +2 Mult and +25 Chips",
            Self::Mars => "Level up Four of a Kind, +3 Mult and +30 Chips",
            Self::Neptune => "Level up Straight Flush, +4 Mult and +40 Chips",
            Self::PlanetX => "Level up Five of a Kind, +3 Mult and +35 Chips",
            Self::Ceres => "Level up Flush House, +4 Mult and +40 Chips",
            Self::Eris => "Level up Flush Five, +3 Mult and +40 Chips",
        }
    }

    pub fn get_hand(&self) -> HandType {
        match self {
            Self::Pluto => HandType::HighCard,
            Self::Mercury => HandType::Pair,
            Self::Uranus => HandType::TwoPair,
            Self::Venus => HandType::ThreeOfAKind,
            Self::Saturn => HandType::Straight,
            Self::Jupiter => HandType::Flush,
            Self::Earth => HandType::FullHouse,
            Self::Mars => HandType::FourOfAKind,
            Self::Neptune => HandType::StraightFlush,
            Self::PlanetX => HandType::FiveOfAKind,
            Self::Ceres => HandType::FlushHouse,
            Self::Eris => HandType::FlushFive,
        }
    }
}

/* This will represent a Planet card */
pub struct Planet {
    planet_type: PlanetType,
    sell_value: i16,
    cost: i16,
}

impl Planet {
    pub fn get_name(&self) -> &'static str {
        self.planet_type.get_name()
    }

    pub fn get_desc(&self) -> &'static str {
        self.planet_type.get_desc()
    }

    pub fn get_sell_val(&self) -> i16 {
        self.sell_value
    }

    pub fn get_hand(&self) -> HandType {
        self.planet_type.get_hand()
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
        let cost: i16 = t
            .get("COST")
            .ok()
            .ok_or("Failed to parse cost while parsing Planet")?;

        Ok(Planet {
            planet_type: PlanetType::from_str(&name),
            sell_value,
            cost,
        })
    }
}

impl Buyable for Planet {
    fn get_item_name(&self) -> &str {
        self.get_name()
    }

    fn get_price(&self) -> i16 {
        self.cost
    }

    fn get_action(&self, i: i16) -> crate::Action {
        crate::Action::UseCons(i, (0, 0))
    }
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.get_name();
        let d = &self.get_desc();
        let h = &self.get_hand();

        write!(f, "{n} | {d}")
    }
}
