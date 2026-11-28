use std::{
    fmt::{format, write},
    panic,
};

use crate::{
    Action::SellJoker,
    ApiChoice, ApiQuestion, ApiQuestionCriteria,
    balatro_state::Blind::{Boss, Small},
    game_interface::Action,
};

use super::*;

/* This is the master GameState object, everything else runs through this */
pub struct GameState {
    state: State,
    hand: Option<Vec<Card>>,
    vouchers_owned: Vec<Voucher>,
    jokers_owned: Vec<Joker>,
    tags_owned: Vec<Tag>,
    consumables: Vec<Consumable>,
    shop: Option<Shop>,
    pack: Option<OpenPack>,
    blinds: [Blind; 3],
    current_blind: u8,
    ctx: Ctx,
    deck: Vec<Card>,
}

impl GameState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        state: State,
        hand: Option<Vec<Card>>,
        vouchers_owned: Vec<Voucher>,
        jokers_owned: Vec<Joker>,
        tags_owned: Vec<Tag>,
        consumables: Vec<Consumable>,
        shop: Option<Shop>,
        pack: Option<OpenPack>,
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

    /* This will get the actions from selling jokers or consumables */
    fn get_sell_actions(&self) -> Vec<ApiChoice> {
        let mut res: Vec<ApiChoice> = Vec::new();

        for i in 0..self.jokers_owned.len() {
            let j = &self.jokers_owned[i];
            let n = j.get_name();
            let sv = j.get_sell_value();
            if sv > 0 {
                let cn_str = format!("Sell {n}");
                let cd_str = format!("Sell Joker --{j}-- for {sv}$");
                res.push(ApiChoice::new(
                    &cn_str,
                    &cd_str,
                    Action::SellJoker(<i16>::try_from(i).unwrap()),
                ));
            }
        }

        for i in 0..self.consumables.len() {
            let c = &self.consumables[i];
            let n = c.get_name();
            let tn = c.get_type_name();
            let sv = c.get_sell_value();
            if sv > 0 {
                let cn_str = format!("Sell {n}");
                let cd_str = format!("Sell {tn} --{c}-- for {sv}$");
                res.push(ApiChoice::new(
                    &cn_str,
                    &cd_str,
                    Action::SellCons(<i16>::try_from(i).unwrap()),
                ));
            }
        }
        res
    }

    /* This will get the actions from a shop, panic if none exists */
    fn get_shop_actions(&self) -> Vec<ApiChoice> {
        match &self.shop {
            Some(s) => s.get_actions(&self.ctx),
            None => panic!("Get shop actions called without an existing shop"),
        }
    }

    /*
     * This will get the actions of picking cards from a pack, panic if no pack is open
     */
    fn get_pack_actions(&self) -> Vec<ApiChoice> {
        match &self.pack {
            Some(p) => p.get_choices(&self.ctx),
            None => panic!("Tried to get pack options with no open pack"),
        }
    }

    /* This function will get the current hand, panic if none exists */
    pub fn get_hand(&self) -> &Vec<Card> {
        match &self.hand {
            Some(h) => h,
            None => panic!("No hand exists"),
        }
    }

    /*
     * This will get an item with a certain idx in a pack
     * panic if no pack is open
     */
    pub fn get_pack_item(&self, idx: usize) -> Box<&dyn fmt::Display> {
        match &self.pack {
            Some(p) => p.get_item(idx),
            None => panic!("Tried to get pack item with no pack open"),
        }
    }

    /*
     * This will get the actions at blind selection, such as skip or play
     */
    fn get_blind_actions(&self) -> Vec<ApiChoice> {
        let mut res: Vec<ApiChoice> = Vec::new();
        let cur = &self.blinds[usize::from(self.current_blind)];
        if self.current_blind == 2 {
            res.push(ApiChoice::new(
                "Play",
                &format!("Play the boss blind: {cur}"),
                Action::NextBlind,
            ));
        } else {
            let tag = match cur {
                Blind::Small(_, t) => t,
                Blind::Big(_, t) => t,
                Blind::Boss(_, _, _) => {
                    panic!("Somehow a big blind was in spot {}", self.current_blind)
                }
            };
            res.push(ApiChoice::new(
                "Play",
                &format!("Play the next blind: {cur}"),
                Action::NextBlind,
            ));
            res.push(ApiChoice::new(
                "Skip",
                &format!("Skip the next blind ({cur}) and gain a skip tag: {tag}"),
                Action::SkipBlind,
            ));
        }

        res
    }

    /*
     * This will get the actions from using consumables
     */
    fn get_consumable_actions(&self) -> Vec<ApiChoice>{
        let mut res: Vec<ApiChoice> = Vec::new();
        for (i, cons) in self.consumables.iter().enumerate() {
            if cons.can_play(&self.ctx) {
                res.push(
                    ApiChoice::new(
                        &format!("Play {}", cons.get_name()),
                        &format!("Play {} | {}",
                            cons.get_name(), cons.get_desc()),
                        Action::UseCons(i as i16, cons.get_targets())
                    )
                );
            }
        }

        res
    }

    /*
     * This will get all possible actions for the current game state
     */
    pub fn get_possible_actions(&self) -> Option<ApiQuestion> {
        if !self.state.is_decision_state() {
            return None;
        }

        let qname = "Action";
        let qinstr = "Which action should you take?";

        let mut sell_qs = self.get_sell_actions();
        let mut cons_qs = self.get_consumable_actions();

        let mut gen_qs = Vec::new();
        gen_qs.extend(sell_qs);
        gen_qs.extend(cons_qs);

        match &self.state {
            State::Shop => {
                let mut shop_qs = self.get_shop_actions();

                gen_qs.extend(shop_qs);
                Some(ApiQuestion::new(
                    qname,
                    qinstr,
                    ApiQuestionCriteria::Choice(gen_qs),
                ))
            }
            State::BlindSelect => {
                let mut blind_qs = self.get_blind_actions();
                gen_qs.extend(blind_qs);
                Some(ApiQuestion::new(
                    qname,
                    qinstr,
                    ApiQuestionCriteria::Choice(gen_qs),
                ))
            }
            State::PackOpening => {
                let mut pack_qs = self.get_pack_actions();
                gen_qs.extend(pack_qs);
                Some(ApiQuestion::new(
                    qname,
                    qinstr,
                    ApiQuestionCriteria::Choice(gen_qs),
                ))
            }
            State::RoundEval => {
                let ac = vec![ApiChoice::new(
                    "Continue",
                    "Continue to the next blind",
                    Action::EndRoundEval,
                )];
                Some(ApiQuestion::new(
                    qname,
                    qinstr,
                    ApiQuestionCriteria::Choice(ac),
                ))
            }
            State::SelectingHand => {
                let mut hand_qs = vec![ApiChoice::new(
                    "Play Hand",
                    "Play cards from your hand",
                    Action::PlayHand,
                )];

                if self.ctx.get_rem_discard() > 0 {
                    hand_qs.push_mut(ApiChoice::new(
                        "Discard Hand",
                        "Discard cards from your hand",
                        Action::DiscardHand,
                    ));
                }

                gen_qs.extend(hand_qs);
                Some(ApiQuestion::new(
                    qname,
                    qinstr,
                    ApiQuestionCriteria::Choice(gen_qs),
                ))
            }
            (s) => panic!("No matched state, got {}", s),
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hand_str = match &self.hand {
            Some(v) => vec_to_str("Hand", v),
            None => String::new(),
        };

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

        let shop_str = match shop {
            Some(s) => format!("{}", s),
            None => String::new(),
        };

        let pack_str = match pack {
            Some(p) => format!("{}", p),
            None => String::new(),
        };

        write!(
            f,
            "Current state:{state}\
            \n\n{deck_str}\
            \n{hand_str}\
            \n\n{jkr_str}\
            \n{cns_str}\
            \n{ov_str}\
            \n{tag_str}\
            \n\n{blind_str}\
            \n{cur_blind_str}\
            \n\n{ctx}\
            \n{pack_str}\
            \n{shop_str}"
        )
    }
}
