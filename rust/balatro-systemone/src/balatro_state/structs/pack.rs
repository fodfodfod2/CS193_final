use super::*;

pub enum PackType {
    Card,
    Joker,
    Planet,
    Tarot,
    Spectral,
}

pub struct ShopPack {
    pack_type: PackType,
    item_cnt: i8,
    price: i8,
    select_num: i8,
}

impl ShopPack {
    pub fn new(pack_type: PackType, item_cnt: i8, price: i8, select_num: i8) -> Self {
        ShopPack {
            pack_type,
            item_cnt,
            price,
            select_num,
        }
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
        let p = &self.price;
        let sn = &self.select_num;
        let ic = &self.item_cnt;
        write!(
            f,
            "{ptn} Pack, Open to select {sn} out of {ic} random {pcn}s. Costs {p}$"
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
