use std::{ptr::write, vec};

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
    pub fn new(
        joker: Vec<Joker>,
        planet: Vec<Planet>,
        tarot: Vec<Tarot>,
        spectral: Vec<Spectral>,
        card: Vec<Card>,
        packs: Vec<ShopPack>,
        vouchers: Vec<Voucher>,
    ) -> Self {
        Shop {
            joker,
            planet,
            tarot,
            spectral,
            card,
            packs,
            vouchers,
        }
    }
}

impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let jkr_str = vec_to_str("Jokers", &self.joker);
        let pnt_str = vec_to_str("Planets", &self.planet);
        let tar_str = vec_to_str("Tarot Cards", &self.tarot);
        let spl_str = vec_to_str("Spectrals", &self.spectral);
        let crd_str = vec_to_str("Cards", &self.card);
        let pk_str = vec_to_str("Packs", &self.packs);
        let v_str = vec_to_str("Vouchers", &self.vouchers);

        let indented: String = indent(&jkr_str, 1)
            + &indent(&pnt_str, 1)
            + &indent(&tar_str, 1)
            + &indent(&spl_str, 1)
            + &indent(&crd_str, 1)
            + &indent(&pk_str, 1)
            + &indent(&v_str, 1);

        write!(f, "Shop:\n{indented}")
    }
}
