use std::panic;

use mlua::Table;

use super::*;

pub enum PackType {
    Card,
    Joker,
    Planet,
    Tarot,
    Spectral,
}

impl PackType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Card" => PackType::Card,
            "Joker" => PackType::Joker,
            "Planet" => PackType::Planet,
            "Tarot" => PackType::Tarot,
            "Spectral" => PackType::Spectral,
            _ => panic!("No Valid pack type"),
        }
    }
}

pub struct ShopPack {
    pack_type: PackType,
    item_cnt: i8,
    cost: i8,
    select_num: i8,
}

impl ShopPack {
    pub fn new(pack_type: PackType, item_cnt: i8, cost: i8, select_num: i8) -> Self {
        ShopPack {
            pack_type,
            item_cnt,
            cost,
            select_num,
        }
    }
}

impl Parsable for ShopPack {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let select_num: i8 = t
            .get("SELECT_NUM")
            .ok()
            .ok_or("Failed to parse select num from shop pack")?;
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse name from shop pack")?;
        let item_cnt: i8 = t
            .get("SIZE")
            .ok()
            .ok_or("Failed to get item cnt from shop pack")?;
        let cost: i8 = t
            .get("COST")
            .ok()
            .ok_or("Failed to get cost while parsing shop pack")?;
        let type_str: String = t
            .get("TYPE")
            .ok()
            .ok_or("Failed to get type str from shop pack")?;
        let pack_type = PackType::from_str(&type_str);

        Ok(ShopPack {
            pack_type,
            item_cnt,
            cost,
            select_num,
        })
    }
}

impl fmt::Display for ShopPack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptn = match &self.pack_type {
            PackType::Card => "Standard",
            PackType::Joker => "Buffoon",
            PackType::Planet => "Celestial",
            PackType::Tarot => "Arcana",
            PackType::Spectral => "Spectral",
        };
        let pcn = match &self.pack_type {
            PackType::Card => "Card",
            PackType::Joker => "Joker",
            PackType::Planet => "Planet Card",
            PackType::Tarot => "Tarot Card",
            PackType::Spectral => "Spectral Card",
        };
        let c = &self.cost;
        let sn = &self.select_num;
        let ic = &self.item_cnt;
        write!(
            f,
            "{ptn} Pack, Open to select {sn} out of {ic} random {pcn}s.\
             Costs {c}$"
        )
    }
}

pub enum PackContents {
    Card(Vec<Card>),
    Joker(Vec<Joker>),
    Planet(Vec<Planet>),
    Tarot(Vec<Tarot>),
    Spectral(Vec<Spectral>),
}

pub struct OpenPack {
    select_num: i8,
    items: PackContents,
}

impl OpenPack {
    pub fn new(select_num: i8, items: PackContents) -> Self {
        OpenPack { select_num, items }
    }
}

impl Parsable for OpenPack {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let select_num: i8 = t
            .get("SELECT_NUM")
            .ok()
            .ok_or("Failed to parse select num from open pack")?;
        let type_str: String = t
            .get("TYPE")
            .ok()
            .ok_or("Failed to parse type from open pack")?;
        let contents_t: Table = t
            .get("CONTENTS")
            .ok()
            .ok_or("Failed to parse contents from open pack")?;
        let items = match type_str.as_str() {
            "Tarot" => PackContents::Tarot(parse_type_vec(contents_t)),
            "Planet" => PackContents::Planet(parse_type_vec(contents_t)),
            "Joker" => PackContents::Joker(parse_type_vec(contents_t)),
            "Card" => PackContents::Card(parse_type_vec(contents_t)),
            "Spectral" => PackContents::Spectral(parse_type_vec(contents_t)),
            e => panic!(
                "Failed to process pack contents from open pack. type was {}",
                e
            ),
        };

        Ok(OpenPack { select_num, items })
    }
}

impl fmt::Display for OpenPack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptn = match &self.items {
            PackContents::Card(_) => "Standard",
            PackContents::Joker(_) => "Buffoon",
            PackContents::Planet(_) => "Celestial",
            PackContents::Tarot(_) => "Arcana",
            PackContents::Spectral(_) => "Spectral",
        };
        let pcn = match &self.items {
            PackContents::Card(_) => "Card",
            PackContents::Joker(_) => "Joker",
            PackContents::Planet(_) => "Planet Card",
            PackContents::Tarot(_) => "Tarot Card",
            PackContents::Spectral(_) => "Spectral Card",
        };
        let pst: String = match &self.items {
            PackContents::Card(c) => c
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            PackContents::Joker(j) => j
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            PackContents::Planet(p) => p
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            PackContents::Tarot(t) => t
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            PackContents::Spectral(s) => s
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        };

        let sn = &self.select_num;

        write!(f, "{ptn} Pack, select {sn} items from the following\n{pst}")
    }
}
