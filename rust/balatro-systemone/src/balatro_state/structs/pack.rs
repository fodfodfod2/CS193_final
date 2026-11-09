use super::*;

pub enum PackType {
    Card,
    Joker,
    Planet,
    Tarot,
    Spectral,
    Other,
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

pub enum PackContents {
    Card(Vec<Card>),
    Joker(Vec<Joker>),
    Planet(Vec<Planet>),
    Tarot(Vec<Tarot>),
    Spectral(Vec<Spectral>),
    Other,
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
