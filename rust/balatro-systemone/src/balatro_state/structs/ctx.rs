use std::fmt::write;

use mlua::{Table, table};

use super::*;

pub struct Ctx {
    hands: i8,
    discards: i8,
    max_hands: i8,
    max_discards: i8,
    last_played_consumable: Option<Consumable>,
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
        last_played_consumable: Option<Consumable>,
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

impl Parsable for Ctx {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let ctx_t: Table = t
            .get("CTX")
            .ok()
            .ok_or("Failed to read ctx t from master")?;

        let hands: i8 = ctx_t
            .get("HANDS")
            .ok()
            .ok_or("Failed to read hands from ctx t")?;
        let discards: i8 = ctx_t
            .get("DISCARDS")
            .ok()
            .ok_or("Failed to read discards from ctx t")?;
        let max_hands: i8 = ctx_t
            .get("MAX_HANDS")
            .ok()
            .ok_or("Failed to read max hands")?;
        let max_discards: i8 = ctx_t
            .get("MAX_DISCARDS")
            .ok()
            .ok_or("failed to read max discards")?;

        let last_played_consumable: Table = ctx_t
            .get("LAST_PLAYED_CONSUMEABLE")
            .ok()
            .ok_or("failed to read last played consumable")?;
        let lp_tarot: Table = last_played_consumable
            .get("TAROT")
            .ok()
            .ok_or("Failed to read tarot from last played consumeable")?;
        let lp_planet: Table = last_played_consumable
            .get("PLANET")
            .ok()
            .ok_or("Failed to read Planet from last played consumeable")?;
        println!("{}", serde_json::to_string(&lp_tarot).unwrap());
        let lp_cns: Option<Consumable> = match Tarot::parse(lp_tarot) {
            Ok(t) => Some(Consumable::Tarot(t)),
            Err(_) => match Planet::parse(lp_planet) {
                Ok(p) => Some(Consumable::Planet(p)),
                Err(_) => None,
            },
        };
        let mps_str: String = ctx_t
            .get("MOST_PLAYED_HAND")
            .ok()
            .ok_or("Failed to parse most_played_hand from ctx table")?;

        let most_played_hand = HandType::from_str(&mps_str);

        let mut hands_played: HashMap<HandType, bool> = HashMap::new();
        let mut hand_lvls: HashMap<HandType, i16> = HashMap::new();
        let hp_t: Table = ctx_t
            .get("HANDS_PLAYED")
            .ok()
            .ok_or("Failed to read hands played table from ctx")?;
        let hl_t: Table = ctx_t
            .get("HAND_LVLS")
            .ok()
            .ok_or("Failed to read hand levels table from ctx")?;

        for pair in hp_t.pairs::<String, bool>() {
            let (h, p) = pair
                .ok()
                .ok_or("Failed to parse hands played from local table")?;
            hands_played.insert(HandType::from_str(&h), p);
        }

        for pair in hl_t.pairs::<String, bool>() {
            let (h, l) = pair
                .ok()
                .ok_or("Failed to parse hand lvls from local table")?;
            hands_played.insert(HandType::from_str(&h), l);
        }

        Ok(Ctx {
            hands,
            discards,
            max_hands,
            max_discards,
            last_played_consumable: lp_cns,
            hands_played,
            hand_lvls,
            most_played_hand,
        })
    }
}

impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = &self.hands;
        let d = &self.discards;
        let mh = &self.max_hands;
        let md = &self.max_discards;
        let lpc_str = match &self.last_played_consumable {
            Some(v) => format!("{}", v),
            None => String::from("None"),
        };
        let hp = &self.hands_played;
        let hl = &self.hand_lvls;
        let mph = &self.most_played_hand;

        write!(
            f,
            "Context:\
            \n\t{h} out of {mh} hands remaining\
            \n\t{d} out of {md} discards remaining\
            \n\tmost played hand: {mph}\
            \n\tLast Played Tarot or Planet: {lpc_str}"
        )
    }
}
