#![allow(unused)]

use crate::balatro_state::structs::*;

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
    // Shop
    // OPENPACK
    let tags = parse_owned_tags(table.clone());
    let (current_blind, blinds) = Blind::parse(table.clone());
    //CTX
    let deck = parse_deck(table.clone());
}

//fn parse_ctx(table: Table) -> Ctx {
//    let ctx_table: Table = table.get("CTX").expect("Failed to read ctx table");
//
//    let hands: i8 = ctx_table
//        .get("HANDS_PLAYED")
//        .expect("Failed to read hands from ctx table");
//    let discards: i8 = ctx_table
//        .get("DISCARDS")
//        .expect("Failed to read discards from ctx table");
//    let max_hands: i8 = ctx_table
//        .get("MAX_HANDS")
//        .expect("Failed to read max hands");
//    let max_discards: i8 = ctx_table
//        .get("MAX_DISCARDS")
//        .expect("failed to read max discards");
//
//
//    let last_played_consumable: Table = ctx_table
//        .get("LAST_PLAYED_CONSUMABLE")
//        .expect("failed to read last played consumable");
//    let lp_tarot: Table = last_played_consumable.get("TAROT").expect("Failed to read tarot from last played consumeable");
//    let lp_planet: Table = last_played_consumable.get("PLANET").expect("Failed to read Planet from last played consumeable");
//
//    let lp_cns: Consumable = match parse_tarot(lp_tarot) {
//        Ok(t) => Consumable::Tarot(t),
//        Err(_) => match parse_planet(lp_planet) {
//            Ok(p) => Consumable::Planet(p),
//            Err(_) => panic!("Failed to read lp_cns"),
//        },
//    }
//
//}

fn parse_owned_consumables(table: Table) -> Vec<Consumable> {
    let consumable_table: Table = table
        .get("CONSUMEABLES")
        .expect("Failed to read consumeables table");

    let tarot_table: Table = consumable_table
        .get("TAROT")
        .expect("Failed to read tarot from consumeables");
    let planet_table: Table = consumable_table
        .get("PLANET")
        .expect("Failed to read planet from consumeables");
    let spectral_table: Table = consumable_table
        .get("SPECTRAL")
        .expect("Failed to read spectral from consumeables");

    let mut tarot = parse_type_vec(tarot_table);
    let mut planet = parse_type_vec(planet_table);
    let mut spectral = parse_type_vec(spectral_table);

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

fn parse_owned_tags(table: Table) -> Vec<Tag> {
    let tags: Table = table
        .get("TAGS_OWNED")
        .expect("Failed to read owned tags from master");
    parse_type_vec(tags)
}

fn parse_owned_vouchers(table: Table) -> Vec<Voucher> {
    let vouchers: Table = table
        .get("VOUCHERS_OWNED")
        .expect("Failed to read owned vouchers from master");
    parse_type_vec(vouchers)
}

fn parse_owned_jokers(table: Table) -> Vec<Joker> {
    let jokers: Table = table
        .get("JOKERS_OWNED")
        .expect("Failed to read owned jokers from master");
    parse_type_vec(jokers)
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
    parse_type_vec(hand)
}

fn parse_deck(table: Table) -> Vec<Card> {
    let deck: Table = table
        .get("DECK")
        .expect("failed to parse table while parsing deck");
    parse_type_vec(deck)
}

fn parse_type_vec<T: Parsable>(table: Table) -> Vec<T> {
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

        res.push(T::parse(cur_card).unwrap());
        counter += 1;
    }
    res
}
