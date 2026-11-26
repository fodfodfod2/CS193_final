#![allow(unused)]

use crate::balatro_state::structs::*;

use mlua::{Lua, Table};
use std::{env, fs, panic};

/*
 * The path of the dump, find relative to your AppData folder
 * Stored in the .envrc in the project root
 */
const SAVE_PATH: &str = "/home/rlawyer/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro/DUMP.jkr";

/* Read a file into a string */
fn read_file() -> String {
    fs::read_to_string(env::var("DUMP_PATH").unwrap()).expect("Failed to read file")
}

/* Read the master table from the dump file */
fn get_master_table() -> (Lua, Table) {
    let lua: Lua = Lua::new();

    let lua_str = format!("return {}", read_file());
    let res: Table = lua
        .load(&lua_str)
        .eval()
        .expect("Failed to read master table");
    (lua, res)
}

/*
 * Read the gamestate from the master table
 */
pub fn read_game_state() -> Option<GameState> {
    let (lua, table) = get_master_table();

    let state = parse_state(table.clone())?;
    let hand = parse_hand(table.clone());
    let vouchers_owned = parse_owned_vouchers(table.clone())?;
    let jokers_owned = parse_owned_jokers(table.clone())?;
    let tags_owned = parse_owned_tags(table.clone())?;
    let consumables = parse_owned_consumables(table.clone())?;
    let (current_blind, blinds) = Blind::parse(table.clone());
    let ctx = match Ctx::parse(table.clone()) {
        Ok(v) => v,
        Err(s) => panic!("{}", s),
    };
    let deck = parse_deck(table.clone())?;

    let shop: Option<Shop> = if state == State::Shop {
        Some(Shop::parse(table.clone()).unwrap())
    } else {
        None
    };
    let pack: Option<OpenPack> = if state == State::PackOpening {
        Some(match OpenPack::parse(table.clone().get("PACK").ok()?) {
            Ok(v) => v,
            Err(s) => panic!("{}", s),
        })
    } else {
        None
    };

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

/* parse the owned consumables from the master table */
fn parse_owned_consumables(table: Table) -> Option<Vec<Consumable>> {
    let consumable_table: Table = table.get("CONSUMEABLES").ok()?;

    let mut consumables = parse_type_vec(consumable_table);

    let mut res: Vec<Consumable> = Vec::new();
    for c in consumables {
        res.push(c);
    }

    Some(res)
}

/* parse the owned tags from the master table */
fn parse_owned_tags(table: Table) -> Option<Vec<Tag>> {
    let tags: Table = table.get("TAGS_OWNED").ok()?;
    Some(parse_type_vec(tags))
}

/* parse the owned vouchers from the master table */
fn parse_owned_vouchers(table: Table) -> Option<Vec<Voucher>> {
    let vouchers: Table = table.get("VOUCHERS_OWNED").ok()?;
    Some(parse_type_vec(vouchers))
}

/* parse the owned jokers from the master table */
fn parse_owned_jokers(table: Table) -> Option<Vec<Joker>> {
    let jokers: Table = table.get("JOKERS_OWNED").ok()?;
    Some(parse_type_vec(jokers))
}

/* parse the state from the master table */
fn parse_state(table: Table) -> Option<State> {
    let val = table.get("STATE").ok()?;
    Some(State::from(val))
}

/* parse the hand from the master table */
fn parse_hand(table: Table) -> Option<Vec<Card>> {
    let hand: Table = table.get("HAND").ok()?;
    Some(parse_type_vec(hand))
}

/* parse the deck from the master table */
fn parse_deck(table: Table) -> Option<Vec<Card>> {
    let deck: Table = table.get("DECK").ok()?;
    Some(parse_type_vec(deck))
}
