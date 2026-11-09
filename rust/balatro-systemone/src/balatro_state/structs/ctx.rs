use super::*;

pub struct Ctx {
    hands: i8,
    discards: i8,
    max_hands: i8,
    max_discards: i8,
    last_played_consumable: Consumable,
    hand_played: HashMap<HandType, bool>,
    hand_lvls: HashMap<HandType, i16>,
    most_played_hand: HandType,
}

impl Ctx {
    pub fn default() -> Self {
        Ctx {
            hands: -1,
            discards: -1,
            max_hands: -1,
            max_discards: -1,
            last_played_consumable: Consumable::Other,
            hand_played: HashMap::new(),
            hand_lvls: HashMap::new(),
            most_played_hand: HandType::HighCard,
        }
    }
}
