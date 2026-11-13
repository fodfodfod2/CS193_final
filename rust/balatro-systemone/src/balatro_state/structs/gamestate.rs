use std::fmt::write;

use super::*;

pub struct GameState {
    state: State,
    hand: Vec<Card>,
    vouchers_owned: Vec<Voucher>,
    jokers_owned: Vec<Joker>,
    tags_owned: Vec<Tag>,
    consumables: Vec<Consumable>,
    shop: Shop,
    pack: OpenPack,
    blinds: [Blind; 3],
    current_blind: u8,
    ctx: Ctx,
    deck: Vec<Card>,
}

impl GameState {
    pub fn new(
        state: State,
        hand: Vec<Card>,
        vouchers_owned: Vec<Voucher>,
        jokers_owned: Vec<Joker>,
        tags_owned: Vec<Tag>,
        consumables: Vec<Consumable>,
        shop: Shop,
        pack: OpenPack,
        blinds: [Blind; 3],
        current_blind: u8,
        ctx: Ctx,
        deck: Vec<Card>,
    ) -> Self {
        GameState {
            state,
            hand,
            vouchers_owned,
            jokers_owned,
            tags_owned,
            consumables,
            shop,
            pack,
            blinds,
            current_blind,
            ctx,
            deck,
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hand_str = vec_to_str("Hand", &self.hand);
        let deck_str = vec_to_str("Deck", &self.deck);
        let jkr_str = vec_to_str("Jokers Owned", &self.jokers_owned);
        let tag_str = vec_to_str("Tags owned", &self.tags_owned);
        let cns_str = vec_to_str("consumables", &self.consumables);
        let ov_str = vec_to_str("Vouchers Owned", &self.vouchers_owned);

        let blind_str = format!(
            "Blinds\n\t{}\n\t{}\n\t{}",
            self.blinds[0], self.blinds[1], self.blinds[2]
        );
        let cur_blind_str = match &self.current_blind {
            0 => String::from("Current Blind: Small Blind"),
            1 => String::from("Current Blind: Big Blind"),
            2 => String::from("Current Blind: Boss Blind"),
            _ => panic!("BAD_CUR_BLIND"),
        };

        let state = &self.state;
        let shop = &self.shop;
        let pack = &self.pack;
        let ctx = &self.ctx;

        write!(
            f,
            "Current state:{state}\n\n{deck_str}\n{hand_str}\n\n{jkr_str}\n{cns_str}\n{ov_str}\n{tag_str}\n\n{blind_str}\n{cur_blind_str}\n\n{ctx}\n{pack}\n{shop}"
        )
    }
}
