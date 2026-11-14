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
    let res: Table = lua
        .load(&lua_str)
        .eval()
        .expect("Failed to read master table");
    (lua, res)
}

pub fn read_game_state() -> Option<GameState> {
    let (lua, table) = get_master_table();

    let state = parse_state(table.clone())?;
    let hand = parse_hand(table.clone());
    let vouchers_owned = parse_owned_vouchers(table.clone())?;
    let jokers_owned = parse_owned_jokers(table.clone())?;
    let tags_owned = parse_owned_tags(table.clone())?;
    let consumables = parse_owned_consumables(table.clone())?;
    let (current_blind, blinds) = Blind::parse(table.clone());
    let ctx = match  Ctx::parse(table.clone()) {
        Ok(v) => v,
        Err(s) => panic!("{}", s),
    };
    let deck = parse_deck(table.clone())?;

    //TODO
    let shop: Option<Shop> = None;
    let pack: Option<OpenPack> = None;

    Some(GameState::new(
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
    ))
}

fn parse_owned_consumables(table: Table) -> Option<Vec<Consumable>> {
    let consumable_table: Table = table.get("CONSUMEABLES").ok()?;

    let tarot_table: Table = consumable_table.get("TAROT").ok()?;
    let planet_table: Table = consumable_table.get("PLANET").ok()?;
    let spectral_table: Table = consumable_table.get("SPECTRAL").ok()?;

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

    Some(res)
}

fn parse_owned_tags(table: Table) -> Option<Vec<Tag>> {
    let tags: Table = table.get("TAGS_OWNED").ok()?;
    Some(parse_type_vec(tags))
}

fn parse_owned_vouchers(table: Table) -> Option<Vec<Voucher>> {
    let vouchers: Table = table.get("VOUCHERS_OWNED").ok()?;
    Some(parse_type_vec(vouchers))
}

fn parse_owned_jokers(table: Table) -> Option<Vec<Joker>> {
    let jokers: Table = table.get("JOKERS_OWNED").ok()?;
    Some(parse_type_vec(jokers))
}

fn parse_state(table: Table) -> Option<State> {
    let val = table.get("STATE").ok()?;
    Some(State::from(val))
}

fn parse_hand(table: Table) -> Option<Vec<Card>> {
    let hand: Table = table.get("HAND").ok()?;
    Some(parse_type_vec(hand))
}

fn parse_deck(table: Table) -> Option<Vec<Card>> {
    let deck: Table = table.get("DECK").ok()?;
    Some(parse_type_vec(deck))
}

fn parse_type_vec<T: Parsable>(table: Table) -> Vec<T> {
    let mut counter = 1;
    let mut res: Vec<T> = Vec::new();
    loop {
        let cur_card: Table = match table.get(counter) {
            Ok(v) => v,
            Err(_) => {
                counter -= 1;
                return res;
            }
        };

        res.push(T::parse(cur_card).unwrap());
        counter += 1;
    }
    res
}
