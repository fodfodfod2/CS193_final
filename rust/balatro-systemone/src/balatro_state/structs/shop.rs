use super::*;

pub struct Shop {
    joker: Vec<Joker>,
    planet: Vec<Planet>,
    tarot: Vec<Tarot>,
    spectral: Vec<Spectral>,
    card: Vec<Card>,
    packs: Vec<ShopPack>,
    vouchers: Vec<Voucher>,
}

impl Shop {
    pub fn empty() -> Self {
        Shop {
            joker: Vec::new(),
            planet: Vec::new(),
            tarot: Vec::new(),
            spectral: Vec::new(),
            card: Vec::new(),
            packs: Vec::new(),
            vouchers: Vec::new(),
        }
    }
}
