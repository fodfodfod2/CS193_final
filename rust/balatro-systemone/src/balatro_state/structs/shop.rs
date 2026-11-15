use std::{fmt::format, ptr::write, vec};

use mlua::table::Table;

use crate::ApiChoice;

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

    pub fn get_actions(&self, c: &Ctx) -> Vec<ApiChoice> {
        let mut res = Vec::new();
        let m = c.get_money();

        if c.get_open_joker_slots() > 0 {
            for j in &self.joker {
                if j.get_price() > m {
                    res.push(get_choice(j));
                }
            }
        }
        res
    }
}

impl Parsable for Shop {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let shop_t: Table = t
            .get("SHOP")
            .ok()
            .ok_or("Failed to parse shop table from master")?;

        let j_t: Table = shop_t
            .get("JOKER")
            .ok()
            .ok_or("Failed to parse jokers table from shop table")?;
        let joker: Vec<Joker> = parse_type_vec(j_t);

        let p_t: Table = shop_t
            .get("PLANET")
            .ok()
            .ok_or("Failed to parse planets table from shop table")?;
        let planet: Vec<Planet> = parse_type_vec(p_t);

        let t_t: Table = shop_t
            .get("TAROT")
            .ok()
            .ok_or("Failed to parse tarots table from shop table")?;
        let tarot: Vec<Tarot> = parse_type_vec(t_t);

        let s_t: Table = shop_t
            .get("SPECTRAL")
            .ok()
            .ok_or("Failed to parse spectrals table from shop table")?;
        let spectral: Vec<Spectral> = parse_type_vec(s_t);

        let c_t: Table = shop_t
            .get("CARD")
            .ok()
            .ok_or("Failed to parse cards table from shop table")?;
        let card: Vec<Card> = parse_type_vec(c_t);

        let v_t: Table = shop_t
            .get("VOUCHER")
            .ok()
            .ok_or("Failed to parse vouchers table from shop table")?;
        let vouchers: Vec<Voucher> = parse_type_vec(v_t);

        let pk_t: Table = shop_t
            .get("PACK")
            .ok()
            .ok_or("Failed to parse packs table from shop table")?;
        let packs: Vec<ShopPack> = parse_type_vec(pk_t);

        Ok(Shop {
            joker,
            planet,
            tarot,
            spectral,
            card,
            vouchers,
            packs,
        })
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
            + "\n" + &indent(&pnt_str, 1)
            + "\n" + &indent(&tar_str, 1)
            + "\n" + &indent(&spl_str, 1)
            + "\n" + &indent(&crd_str, 1)
            + "\n" + &indent(&pk_str, 1)
            + "\n" + &indent(&v_str, 1);

        write!(f, "Shop:\n{indented}")
    }
}
