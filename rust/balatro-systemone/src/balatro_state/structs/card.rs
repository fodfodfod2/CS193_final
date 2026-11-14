use mlua::Table;

use super::*;

/* This will store a Seal applicable to cards */
pub struct Seal {
    name: String,
    desc: String,
}

impl Seal {
    pub fn new(name: String, desc: String) -> Self {
        Seal { name, desc }
    }
}

impl Parsable for Seal {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Seal")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Seal")?;

        Ok(Seal { name, desc })
    }
}

impl fmt::Display for Seal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "{n} Seal | Ability: {d}")
    }
}

/* This will store a Enhancement applicable to cards */
pub struct Enhancement {
    name: String,
    desc: String,
}

impl Enhancement {
    pub fn new(name: String, desc: String) -> Self {
        Enhancement { name, desc }
    }
}

impl Parsable for Enhancement {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Enhancement")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Enhancement")?;

        Ok(Enhancement { name, desc })
    }
}

impl fmt::Display for Enhancement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "{n} | Ability: {d}")
    }
}

/* This represents the possible ranks a card can be */
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    King,
    Queen,
    Ace,
}

impl Rank {
    pub fn from_int(int: i16) -> Self {
        match int {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            13 => Rank::King,
            12 => Rank::Queen,
            14 => Rank::Ace,
            n => panic!("Invalid rank {n}"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::King => write!(f, "King"),
            Rank::Queen => write!(f, "Queen"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}

/* This represents the possible suits a card can be */
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl Suit {
    pub fn from_str(string: &str) -> Self {
        match string {
            "Diamonds" => Suit::Diamonds,
            "Hearts" => Suit::Hearts,
            "Spades" => Suit::Spades,
            "Clubs" => Suit::Clubs,
            s => panic!("Unrecognized suit {s}"),
        }
    }
    pub fn from_int(int: u8) -> Self {
        match int {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            3 => Suit::Spades,
            i => panic!("Unrecognized suit {i}"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}

/* This will hold all of the pub structs and pub enums used for the game state */
pub struct Card {
    rank: Rank,
    suit: Suit,
    enhancement: Enhancement,
    seal: Seal,
    disabled: bool,
    face_down: bool,
}

impl Card {
    pub fn new(
        rank: Rank,
        suit: Suit,
        enhancement: Enhancement,
        seal: Seal,
        disabled: bool,
        face_down: bool,
    ) -> Self {
        Card {
            rank,
            suit,
            enhancement,
            seal,
            disabled,
            face_down,
        }
    }
}

impl Parsable for Card {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let rank_v: i16 = t
            .get("RANK")
            .ok()
            .ok_or("Failed to parse rank while parsing Card")?;
        let suit_i: u8 = t
            .get("SUIT")
            .ok()
            .ok_or("Failed to parse suit while parsing Card")?;
        let enhancement_t: Table = t
            .get("ENHANCEMENT")
            .ok()
            .ok_or("Failed to parse enhancement while parsing Card")?;
        let seal_t: Table = t
            .get("SEAL")
            .ok()
            .ok_or("Failed to parse seal while parsing Card")?;
        let disabled: bool = t
            .get("DISABLED")
            .ok()
            .ok_or("Failed to parse disabled while parsing Card")?;
        let face_down: bool = t
            .get("FACE_DOWN")
            .ok()
            .ok_or("Failed to parse face_down while parsing Card")?;

        let rank = Rank::from_int(rank_v);
        let suit = Suit::from_int(suit_i);
        let enhancement = Enhancement::parse(enhancement_t)?;
        let seal = Seal::parse(seal_t)?;

        Ok(Card {
            rank,
            suit,
            enhancement,
            seal,
            disabled,
            face_down,
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = &self.rank;
        let su = &self.suit;
        let e = &self.enhancement;
        let se = &self.seal;
        let d = &self.disabled;
        let ds = match d {
            true => "Disabled",
            false => "",
        };
        let fd = &self.face_down;
        write!(f, "{ds} {r} of {su} - {e} - {se}")
    }
}
