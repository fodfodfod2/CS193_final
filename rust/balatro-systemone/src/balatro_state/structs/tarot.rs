use mlua::Table;

use crate::Action;

use super::*;

/* This will represent a Tarot card */
pub enum TarotType {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmporer,
    TheHeirophant,
    TheLovers,
    TheChariot,
    Justice,
    TheHermit,
    TheWheelofFortune,
    Strength,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
}

impl TarotType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "The Fool" => Self::TheFool,
            "The Magician" => Self::TheMagician,
            "The High Priestess" => Self::TheHighPriestess,
            "The Empress" => Self::TheEmpress,
            "The Emperor" => Self::TheEmporer,
            "The Hierophant" => Self::TheHeirophant,
            "The Lovers" => Self::TheLovers,
            "The Chariot" => Self::TheChariot,
            "Justice" => Self::Justice,
            "The Hermit" => Self::TheHermit,
            "The Wheel of Fortune" => Self::TheWheelofFortune,
            "Strength" => Self::Strength,
            "The Hanged Man" => Self::TheHangedMan,
            "Death" => Self::Death,
            "Temperance" => Self::Temperance,
            "The Devil" => Self::TheDevil,
            "The Tower" => Self::TheTower,
            "The Star" => Self::TheStar,
            "The Moon" => Self::TheMoon,
            "The Sun" => Self::TheSun,
            "Judgement" => Self::Judgement,
            "The World" => Self::TheWorld,
            s => panic!("Failed to match tarot to {}", s),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::TheFool => "The Fool",
            Self::TheMagician => "The Magician",
            Self::TheHighPriestess => "The High Priestess",
            Self::TheEmpress => "The Empress",
            Self::TheEmporer => "The Emperor",
            Self::TheHeirophant => "The Hierophant",
            Self::TheLovers => "The Lovers",
            Self::TheChariot => "The Chariot",
            Self::Justice => "Justice",
            Self::TheHermit => "The Hermit",
            Self::TheWheelofFortune => "The Wheel of Fortune",
            Self::Strength => "Strength",
            Self::TheHangedMan => "The Hanged Man",
            Self::Death => "Death",
            Self::Temperance => "Temperance",
            Self::TheDevil => "The Devil",
            Self::TheTower => "The Tower",
            Self::TheStar => "The Star",
            Self::TheMoon => "The Moon",
            Self::TheSun => "The Sun",
            Self::Judgement => "Judgement",
            Self::TheWorld => "The World",
        }
    }

    pub fn get_desc(&self) -> &'static str {
        match self {
            Self::TheFool => {
                "Creates the last Tarot or Planet card used during this run (The Fool excluded). This card is in the context as Last Played Tarot or Planet"
            }
            Self::TheMagician => "Enhances 2 selected cards to Lucky Cards",
            Self::TheHighPriestess => "Creates up to 2 random Planet cards (Must have room)",
            Self::TheEmpress => "Enhances 2 selected cards to Mult Cards",
            Self::TheEmporer => "Creates up to 2 random Tarot cards (Must have room)",
            Self::TheHeirophant => "Enhances 2 selected cards to Bonus Cards",
            Self::TheLovers => "Enhances 1 selected card into a Wild Card",
            Self::TheChariot => "Enhances 1 selected card into a Steel Card",
            Self::Justice => "Enhances 1 selected card into a Glass Card",
            Self::TheHermit => "Doubles money (Max of $20)",
            Self::TheWheelofFortune => {
                "1 in 4 chance to add Foil, Holographic, or Polychrome edition to a random Joker"
            }
            Self::Strength => "Increases rank of up to 2 selected cards by 1",
            Self::TheHangedMan => "Destroys up to 2 selected cards",
            Self::Death => {
                "Select 2 cards, convert the left card into the right card (Drag to rearrange)"
            }
            Self::Temperance => "Gives the total sell value of all current Jokers (Max of $50)",
            Self::TheDevil => "Enhances 1 selected card into a Gold Card",
            Self::TheTower => "Enhances 1 selected card into a Stone Card",
            Self::TheStar => "Converts up to 3 selected cards to Diamonds",
            Self::TheMoon => "Converts up to 3 selected cards to Clubs",
            Self::TheSun => "Converts up to 3 selected cards to Hearts",
            Self::Judgement => "Creates a random Joker card (Must have room)",
            Self::TheWorld => "Converts up to 3 selected cards to Spades",
        }
    }

    pub fn get_select_num(&self) -> (i8, i8) {
        match &self {
            TarotType::TheFool => (0, 0),
            TarotType::TheMagician => (1, 2),
            TarotType::TheHighPriestess => (0, 0),
            TarotType::TheEmpress => (1, 2),
            TarotType::TheEmporer => (0, 0),
            TarotType::TheHeirophant => (1, 2),
            TarotType::TheLovers => (1, 1),
            TarotType::TheChariot => (1, 1),
            TarotType::Justice => (1, 1),
            TarotType::TheHermit => (0, 0),
            TarotType::TheWheelofFortune => (0, 0),
            TarotType::Strength => (1, 2),
            TarotType::TheHangedMan => (1, 2),
            TarotType::Death => (2, 2),
            TarotType::Temperance => (0, 0),
            TarotType::TheDevil => (1, 1),
            TarotType::TheTower => (1, 1),
            TarotType::TheStar => (1, 3),
            TarotType::TheMoon => (1, 3),
            TarotType::TheSun => (1, 3),
            TarotType::Judgement => (0, 0),
            TarotType::TheWorld => (1, 3),
        }
    }
}

pub struct Tarot {
    tarot_type: TarotType,
    sell_value: i16,
    cost: i16,
}

impl Tarot {
    pub fn get_name(&self) -> &'static str {
        &self.tarot_type.get_name()
    }

    pub fn get_desc(&self) -> &'static str {
        &self.tarot_type.get_desc()
    }

    pub fn get_sell_val(&self) -> i16 {
        self.sell_value
    }
}

impl Buyable for Tarot {
    fn get_item_name(&self) -> &str {
        &self.tarot_type.get_name()
    }

    fn get_price(&self) -> i16 {
        self.cost
    }

    fn get_action(&self, i: i16) -> crate::Action {
        Action::UseCons(i, self.tarot_type.get_select_num())
    }
}

impl Parsable for Tarot {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Tarot")?;
        let sell_value: i16 = t
            .get("SELL_VALUE")
            .ok()
            .ok_or("Failed to parse sell value while parsing Tarot")?;
        let cost: i16 = t
            .get("COST")
            .ok()
            .ok_or("Failed to parse cost while parsing Tarot")?;
        Ok(Tarot {
            tarot_type: TarotType::from_str(&name),
            sell_value,
            cost,
        })
    }
}
impl fmt::Display for Tarot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.tarot_type.get_name();
        let d = &self.tarot_type.get_desc();
        let sv = self.sell_value;

        write!(f, "{n} | use to {d}")
    }
}
