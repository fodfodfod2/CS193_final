use std::fmt::write;

use super::*;

pub struct Ctx {
    hands: i8,
    discards: i8,
    max_hands: i8,
    max_discards: i8,
    last_played_consumable: Consumable,
    hands_played: HashMap<HandType, bool>,
    hand_lvls: HashMap<HandType, i16>,
    most_played_hand: HandType,
}

impl Ctx {
    pub fn new(
        hands: i8,
        discards: i8,
        max_hands: i8,
        max_discards: i8,
        last_played_consumable: Consumable,
        hands_played: HashMap<HandType, bool>,
        hand_lvls: HashMap<HandType, i16>,
        most_played_hand: HandType,
    ) -> Self {
        Ctx {
            hands,
            discards,
            max_hands,
            max_discards,
            last_played_consumable,
            hands_played,
            hand_lvls,
            most_played_hand,
        }
    }
}

impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = &self.hands;
        let d = &self.discards;
        let mh = &self.max_hands;
        let md = &self.max_discards;
        let lpc = &self.last_played_consumable;
        let hp = &self.hands_played;
        let hl = &self.hand_lvls;
        let mph = &self.most_played_hand;

        write!(
            f,
            "Context:\n\tUsed {h} out of {mh} hands\n\tUsed {d} out of {md} discards\n\tmost played hand: {mph}"
        )
    }
}
