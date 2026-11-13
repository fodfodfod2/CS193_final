use mlua::Table;

use super::*;

/* This represents possible blinds the player may face */
pub enum Blind {
    Small(u64, Tag),
    Big(u64, Tag),
    Boss(u64, String, String),
}

impl Blind {
    pub fn parse(t: Table) -> (u8, [Blind; 3]) {
        let blinds: Table = t
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
        let small_tag = Tag::parse(small_tag_table).expect("Failed to parse small blind tag table");

        let big_score: u64 = big_table
            .get("SCORE")
            .expect("Failed to read score from big blind");
        let big_tag_table: Table = big_table
            .get("TAG")
            .expect("Failed to read tag from big blind");
        let big_tag = Tag::parse(big_tag_table).expect("Failed to parse big blind tag table");

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

        let cur_blind: u8 = blinds
            .get("CURRENT")
            .expect("Failed to read cur_blind from boss blind");

        (cur_blind, [small_blind, big_blind, boss_blind])
    }
}

impl fmt::Display for Blind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blind::Small(s, t) => write!(f, "Small Blind | Score:{s} Skip tag:{t}"),
            Blind::Big(s, t) => write!(f, "Big Blind | Score:{s} Skip tag:{t}"),
            Blind::Boss(s, n, d) => write!(f, "Boss Blind | Name:{n} Ability:{d} Blind Score:{s}"),
        }
    }
}
