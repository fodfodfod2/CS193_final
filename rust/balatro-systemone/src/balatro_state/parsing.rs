#![allow(unused)]

use crate::balatro_state::state::*;

use mlua::{Lua, Table};
use std::fs;

const SAVE_PATH: &str = "/home/rlawyer/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro/DUMP.jkr";

fn read_file() -> String {
    fs::read_to_string(SAVE_PATH).expect("Failed to read file")
}

fn get_master_table() -> (Lua, Table) {
    let lua: Lua = Lua::new();

    let lua_str = format!("return {}", read_file());
    let res: Table = lua.load(&lua_str).eval().expect("Failed to parse table");
    (lua, res)
}

pub fn read_game_state() {
    let (lua, table) = get_master_table();

    let state = parse_state(table.clone());
    let hand = parse_hand(table.clone());
    let owned_vouchers = parse_owned_vouchers(table.clone());
    let owned_jokers = parse_owned_jokers(table.clone());
    let owned_tags = parse_owned_tags(table.clone());
    let owned_consumables = parse_owned_consumables(table.clone());
    // SHOP
    // OPENPACK
    let tags = parse_owned_tags(table.clone());
    let (current_blind, blinds) = parse_blinds(table.clone());
    //CTX
    let deck = parse_deck(table.clone());
}

fn parse_owned_consumables(table: Table) -> Vec<Consumable> {
    let consumable_table: Table = table.get("CONSUMEABLES")
        .expect("Failed to read consumeables table");

    let tarot_table: Table = consumable_table.get("TAROT")
        .expect("Failed to read tarot from consumeables");
    let planet_table: Table = consumable_table.get("PLANET")
        .expect("Failed to read planet from consumeables");
    let spectral_table: Table = consumable_table.get("SPECTRAL")
        .expect("Failed to read spectral from consumeables");

    let mut tarot = parse_type_vec(tarot_table, parse_tarot);
    let mut planet = parse_type_vec(planet_table, parse_planet);
    let mut spectral = parse_type_vec(spectral_table, parse_spectral);

    let mut res: Vec<Consumable> = Vec::new();
    for t in tarot {
        res.push(Consumable::Tarot(t));
    }
    for p in planet {
        res.push(Consumable::Planet(p));
    }
    for s in spectral {
        res.push(Consumable::Spectral(s));
    }

    res
}

fn parse_blinds(table: Table) -> (u8, [Blind; 3]) {
    let blinds: Table = table
        .get("BLINDS")
        .expect("Failed to read owned blinds from master");

    let boss_table: Table = blinds.get("BOSS").expect("Failed to read boss blind");
    let big_table: Table = blinds.get("BIG").expect("Failed to read big blind");
    let small_table: Table = blinds.get("SMALL").expect("Failed to read small blind");

    let small_score: u64 = small_table
        .get("SCORE")
        .expect("Failed to read score from small blind");
    let small_tag_table: Table = small_table
        .get("TAG")
        .expect("Failed to read tag from small blind");
    let small_tag = parse_tag(small_tag_table);

    let big_score: u64 = big_table
        .get("SCORE")
        .expect("Failed to read score from big blind");
    let big_tag_table: Table = big_table
        .get("TAG")
        .expect("Failed to read tag from big blind");
    let big_tag = parse_tag(big_tag_table);

    let boss_score: u64 = boss_table
        .get("SCORE")
        .expect("Failed to read score from boss blind");
    let boss_name: String = boss_table
        .get("NAME")
        .expect("Failed to read name from boss blind");
    let boss_desc: String = boss_table
        .get("DESC")
        .expect("Failed to read desc from boss blind");

    let small_blind: Blind = Blind::Small(small_score, small_tag);
    let big_blind: Blind = Blind::Big(big_score, big_tag);
    let boss_blind: Blind = Blind::Boss(boss_score, boss_name, boss_desc);

    let cur_blind: u8 = blinds.get("CURRENT")
        .expect("Failed to read cur_blind from boss blind");

    (cur_blind, [small_blind, big_blind, boss_blind])
}

fn parse_owned_tags(table: Table) -> Vec<Tag> {
    let tags: Table = table
        .get("TAGS_OWNED")
        .expect("Failed to read owned tags from master");
    parse_type_vec(tags, parse_tag)
}

fn parse_owned_vouchers(table: Table) -> Vec<Voucher> {
    let vouchers: Table = table
        .get("VOUCHERS_OWNED")
        .expect("Failed to read owned vouchers from master");
    parse_type_vec(vouchers, parse_voucher)
}

fn parse_owned_jokers(table: Table) -> Vec<Joker> {
    let jokers: Table = table
        .get("JOKERS_OWNED")
        .expect("Failed to read owned jokers from master");
    parse_type_vec(jokers, parse_joker)
}

fn parse_state(table: Table) -> State {
    let val = table
        .get("STATE")
        .expect("Failed to read state from master");
    State::from(val)
}

fn parse_hand(table: Table) -> Vec<Card> {
    let hand: Table = table
        .get("HAND")
        .expect("failed to parse table while parsing hand");
    parse_type_vec(hand, parse_card)
}

fn parse_deck(table: Table) -> Vec<Card> {
    let deck: Table = table
        .get("DECK")
        .expect("failed to parse table while parsing deck");
    parse_type_vec(deck, parse_card)
}

fn parse_type_vec<T>(table: Table, func: fn(Table) -> T) -> Vec<T> {
    let mut counter = 1;
    let mut res: Vec<T> = Vec::new();
    loop {
        let cur_card: Table = match table.get(counter) {
            Ok(v) => v,
            Err(_) => {
                counter -= 1;
                println!("Parsed {counter} {}", std::any::type_name::<T>());
                return res;
            }
        };
        println!("successful parse");

        res.push(func(cur_card));
        counter += 1;
    }
    res
}

/*
 * This function will parse a card from a lua table
 */
fn parse_card(table: Table) -> Card {
    let rank_v: i16 = table
        .get("RANK")
        .expect("Failed to parse rank while parsing Card");
    let suit_s: String = table
        .get("SUIT")
        .expect("Failed to parse suit while parsing Card");
    let enhancement_t: Table = table
        .get("ENHANCEMENT")
        .expect("Failed to parse enhancement while parsing Card");
    let seal_t: Table = table
        .get("SEAL")
        .expect("Failed to parse seal while parsing Card");
    let disabled: bool = table
        .get("DISABLED")
        .expect("Failed to parse disabled while parsing Card");
    let face_down: bool = table
        .get("FACE_DOWN")
        .expect("Failed to parse face_down while parsing Card");

    let rank = Rank::from_int(rank_v);
    let suit = Suit::from_str(&suit_s);
    let enhancement = parse_enhancement(enhancement_t);
    let seal = parse_seal(seal_t);

    Card::new(rank, suit, enhancement, seal, disabled, face_down)
}

/*
 * This function will parse a Joker from a lua table
 */
fn parse_joker(table: Table) -> Joker {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse name while parsing Joker");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse desc while parsing Joker");
    let edition_t: Table = table
        .get("EDITION")
        .expect("Failed to parse edition while parsing Joker");
    let sticker_t: Table = table
        .get("STICKER")
        .expect("Failed to parse sticker while parsing Joker");
    let disabled: bool = table
        .get("DISABLED")
        .expect("Failed to parse disabled while parsing Joker");
    let sell_value: i16 = table
        .get("SELL_VALUE")
        .expect("Failed to parse sell_value while parsing Joker");
    let invis_cnt: i16 = table.get("INVISIBLE_CNT").unwrap_or_default();
    let perish_cnt: i16 = table.get("PERISH_CNT").unwrap_or_default();

    let edition = parse_edition(edition_t);
    let sticker = parse_sticker(sticker_t);

    Joker::new(
        name, desc, edition, sell_value, disabled, invis_cnt, perish_cnt, sticker,
    )
}

fn parse_tag(table: Table) -> Tag {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Enhancement");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Enhancement");

    Tag::new(name, desc)
}

fn parse_enhancement(table: Table) -> Enhancement {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Enhancement");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Enhancement");

    Enhancement::new(name, desc)
}

fn parse_seal(table: Table) -> Seal {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Seal");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Seal");

    Seal::new(name, desc)
}

fn parse_edition(table: Table) -> Edition {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Edition");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Edition");

    Edition::new(name, desc)
}

fn parse_sticker(table: Table) -> Sticker {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Sticker");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Sticker");

    Sticker::new(name, desc)
}

fn parse_voucher(table: Table) -> Voucher {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Voucher");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Voucher");

    Voucher::new(name, desc)
}

fn parse_tarot(table: Table) -> Tarot {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Tarot");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Tarot");
    let sell_value: i16 = table
        .get("SELL_VALUE")
        .expect("Failed to parse sell value while parsing Tarot");

    Tarot::new(name, desc, sell_value)
}

fn parse_planet(table: Table) -> Planet {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Planet");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Planet");
    let sell_value: i16 = table
        .get("SELL_VALUE")
        .expect("Failed to parse sell value while parsing Planet");
    let hand_id: String = table
        .get("HAND_ID")
        .expect("Failed to parse hand id while parsing Planet");
    let hand = HandType::from_str(&hand_id);

    Planet::new(name, desc, sell_value, hand)
}

fn parse_spectral(table: Table) -> Spectral {
    let name: String = table
        .get("NAME")
        .expect("Failed to parse Name while parsing Spectral");
    let desc: String = table
        .get("DESC")
        .expect("Failed to parse Desc while parsing Spectral");
    let sell_value: i16 = table
        .get("SELL_VALUE")
        .expect("Failed to parse sell value while parsing Spectral");

    Spectral::new(name, desc, sell_value)
}
