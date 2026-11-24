use std::fmt::{format, write};

use mlua::{Table, table};

use super::*;
use strum::IntoEnumIterator;

pub struct Ctx {
    hands: i8,
    discards: i8,
    max_hands: i8,
    max_discards: i8,
    money: i16,
    max_joker_cnt: i8,
    cur_joker_cnt: i8,
    max_cons_cnt: i8,
    cur_cons_cnt: i8,
    last_played_consumable: Option<Consumable>,
    hands_played: HashMap<HandType, bool>,
    hand_lvls: HashMap<HandType, i16>,
    most_played_hand: Option<HandType>,
}

impl Ctx {
    pub fn get_money(&self) -> i16 {
        self.money
    }

    pub fn get_open_joker_slots(&self) -> i8 {
        self.max_joker_cnt - self.cur_joker_cnt
    }

    pub fn get_open_cons_slots(&self) -> i8 {
        self.max_cons_cnt - self.cur_cons_cnt
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
        let money: i16 = ctx_t
            .get("MONEY")
            .ok()
            .ok_or("failed to read money from ctx")?;
        let max_joker_cnt: i8 = ctx_t
            .get("MAX_JOKERS")
            .ok()
            .ok_or("failed to read max jokers from ctx")?;
        let cur_joker_cnt: i8 = ctx_t
            .get("JOKER_CNT")
            .ok()
            .ok_or("failed to read current jokers from ctx")?;
        let max_cons_cnt: i8 = ctx_t
            .get("MAX_CONSUMEABLES")
            .ok()
            .ok_or("failed to read max consumeables from ctx")?;
        let cur_cons_cnt: i8 = ctx_t
            .get("CONSUMEABLE_CNT")
            .ok()
            .ok_or("failed to read current jokers from ctx")?;

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

        let most_played_hand = match mps_str.as_str() {
            "" => None,
            s => Some(HandType::from_str(&mps_str)),
        };

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

        for pair in hl_t.pairs::<String, i16>() {
            let (h, l) = pair
                .ok()
                .ok_or("Failed to parse hand lvls from local table")?;
            hand_lvls.insert(HandType::from_str(&h), l);
        }

        Ok(Ctx {
            hands,
            discards,
            max_hands,
            max_discards,
            money,
            max_joker_cnt,
            cur_joker_cnt,
            max_cons_cnt,
            cur_cons_cnt,
            last_played_consumable: lp_cns,
            hands_played,
            hand_lvls,
            most_played_hand,
        })
    }
}

impl fmt::Display for Ctx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h = self.hands;
        let d = self.discards;
        let mh = self.max_hands;
        let md = self.max_discards;
        let m = self.money;
        let cj = self.cur_joker_cnt;
        let mj = self.max_joker_cnt;
        let cc = self.cur_cons_cnt;
        let mc = self.max_cons_cnt;

        let lpc_str = match &self.last_played_consumable {
            Some(v) => format!("{}", v),
            None => String::from("None"),
        };
        let hp = &self.hands_played;
        let hl = &self.hand_lvls;
        let mph = &self.most_played_hand;
        let mph_str = match mph {
            Some(ht) => format!("{ht}"),
            None => String::from("None"),
        };

        let mut hs: String = String::from("HANDTYPES");
        for ht in HandType::iter() {
            hs += &format!(
                "\n\t\t{ht}: Lvl:{}, Played:{}",
                hl.get(&ht).unwrap(),
                hp.get(&ht).unwrap()
            );
        }

        write!(
            f,
            "Context:\
            \n\t{h} out of {mh} hands remaining\
            \n\t{d} out of {md} discards remaining\
            \n\tmost played hand: {mph_str}\
            \n\tLast Played Tarot or Planet: {lpc_str}\
            \n\tYou have {m}$\
            \n\tYou are using {cj} out of {mj} Joker slots\
            \n\tYou are using {cc} out of {mc} Consumable slots\
            \n\t{hs}"
        )
    }
}
