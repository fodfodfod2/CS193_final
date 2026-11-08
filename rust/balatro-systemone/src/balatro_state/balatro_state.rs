use std::fs;
use mlua::{Lua, Table};

const SAVE_PATH: &str = "/home/rlawyer/.local/share/Steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/AppData/Roaming/Balatro/DUMP.jkr";

fn read_file() -> String {
    fs::read_to_string(SAVE_PATH).expect("Failed to read file")
}

fn get_master_table() -> Table {
    let lua: Lua = Lua::new();

    let lua_str = format!("return {}", read_file());
    let res: Table = lua.load(&lua_str).eval().unwrap();
    res
}

