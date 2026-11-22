use mlua::Table;

use super::*;

pub enum VoucherType {
    Overstock,
    OverstockPlus,
    ClearanceSale,
    Liquidation,
    Hone,
    GlowUp,
    RerollSurplus,
    RerollGlut,
    CrystalBall,
    OmenGlobe,
    Telescope,
    Observatory,
    Grabber,
    NachoTong,
    Wasteful,
    Recyclomancy,
    TarotMerchant,
    TarotTycoon,
    PlanetMerchant,
    PlanetTycoon,
    SeedMoney,
    MoneyTree,
    Blank,
    Antimatter,
    MagicTrick,
    Illusion,
    Hieroglyph,
    Petroglyph,
    DirectorsCut,
    Retcon,
    PaintBrush,
    Palette,
}

impl VoucherType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Overstock" => Self::Overstock,
            "Overstock Plus" => Self::OverstockPlus,
            "Clearance Sale" => Self::ClearanceSale,
            "Liquidation" => Self::Liquidation,
            "Hone" => Self::Hone,
            "Glow Up" => Self::GlowUp,
            "Reroll Surplus" => Self::RerollSurplus,
            "Reroll Glut" => Self::RerollGlut,
            "Crystal Ball" => Self::CrystalBall,
            "Omen Globe" => Self::OmenGlobe,
            "Telescope" => Self::Telescope,
            "Observatory" => Self::Observatory,
            "Grabber" => Self::Grabber,
            "Nacho Tong" => Self::NachoTong,
            "Wasteful" => Self::Wasteful,
            "Recyclomancy" => Self::Recyclomancy,
            "Tarot Merchant" => Self::TarotMerchant,
            "Tarot Tycoon" => Self::TarotTycoon,
            "Planet Merchant" => Self::PlanetMerchant,
            "Planet Tycoon" => Self::PlanetTycoon,
            "Seed Money" => Self::SeedMoney,
            "Money Tree" => Self::MoneyTree,
            "Blank" => Self::Blank,
            "Antimatter" => Self::Antimatter,
            "Magic Trick" => Self::MagicTrick,
            "Illusion" => Self::Illusion,
            "Hieroglyph" => Self::Hieroglyph,
            "Petroglyph" => Self::Petroglyph,
            "Director's Cut" => Self::DirectorsCut,
            "Retcon" => Self::Retcon,
            "Paint Brush" => Self::PaintBrush,
            "Palette" => Self::Palette,
            _ => panic!("Unknown voucher name: {s:?}"),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Overstock => "Overstock",
            Self::OverstockPlus => "Overstock Plus",
            Self::ClearanceSale => "Clearance Sale",
            Self::Liquidation => "Liquidation",
            Self::Hone => "Hone",
            Self::GlowUp => "Glow Up",
            Self::RerollSurplus => "Reroll Surplus",
            Self::RerollGlut => "Reroll Glut",
            Self::CrystalBall => "Crystal Ball",
            Self::OmenGlobe => "Omen Globe",
            Self::Telescope => "Telescope",
            Self::Observatory => "Observatory",
            Self::Grabber => "Grabber",
            Self::NachoTong => "Nacho Tong",
            Self::Wasteful => "Wasteful",
            Self::Recyclomancy => "Recyclomancy",
            Self::TarotMerchant => "Tarot Merchant",
            Self::TarotTycoon => "Tarot Tycoon",
            Self::PlanetMerchant => "Planet Merchant",
            Self::PlanetTycoon => "Planet Tycoon",
            Self::SeedMoney => "Seed Money",
            Self::MoneyTree => "Money Tree",
            Self::Blank => "Blank",
            Self::Antimatter => "Antimatter",
            Self::MagicTrick => "Magic Trick",
            Self::Illusion => "Illusion",
            Self::Hieroglyph => "Hieroglyph",
            Self::Petroglyph => "Petroglyph",
            Self::DirectorsCut => "Director's Cut",
            Self::Retcon => "Retcon",
            Self::PaintBrush => "Paint Brush",
            Self::Palette => "Palette",
        }
    }

    pub fn get_desc(&self) -> &'static str {
        match self {
            Self::Overstock => "+1 card slot available in shop (to 3 card slots)",
            Self::OverstockPlus => "+1 card slot available in shop (to 4 card slots)",
            Self::ClearanceSale => "All cards and packs in shop are 25% off",
            Self::Liquidation => "All cards and packs in shop are 50% off",
            Self::Hone => "Foil, Holographic, and Polychrome cards appear 2X more often",
            Self::GlowUp => "Foil, Holographic, and Polychrome cards appear 4X more often",
            Self::RerollSurplus => "Rerolls cost $2 less",
            Self::RerollGlut => "Rerolls cost an additional $2 less",
            Self::CrystalBall => "+1 consumable slot",
            Self::OmenGlobe => "Spectral cards may appear in any of the Arcana Packs",
            Self::Telescope => "Celestial Packs always contain the Planet card for your most played poker hand",
            Self::Observatory => "Planet cards in your consumable area give X1.5 Mult for their specified poker hand",
            Self::Grabber => "Permanently gain +1 hand per round",
            Self::NachoTong => "Permanently gain an additional +1 hand per round",
            Self::Wasteful => "Permanently gain +1 discard each round",
            Self::Recyclomancy => "Permanently gain an additional +1 discard each round",
            Self::TarotMerchant => "Tarot cards appear 2X more frequently in the shop",
            Self::TarotTycoon => "Tarot cards appear 4X more frequently in the shop",
            Self::PlanetMerchant => "Planet cards appear 2X more frequently in the shop",
            Self::PlanetTycoon => "Planet cards appear 4X more frequently in the shop",
            Self::SeedMoney => "Raise the cap on interest earned in each round to $10",
            Self::MoneyTree => "Raise the cap on interest earned in each round to $20",
            Self::Blank => "Does nothing?",
            Self::Antimatter => "+1 Joker slot",
            Self::MagicTrick => "Playing cards can be purchased from the shop",
            Self::Illusion => "Playing cards in shop may have an Enhancement, Edition, and/or a Seal",
            Self::Hieroglyph => "-1 Ante, -1 hand each round",
            Self::Petroglyph => "-1 Ante, -1 discard each round",
            Self::DirectorsCut => "Reroll Boss Blind 1 time per Ante, $10 per reroll",
            Self::Retcon => "Reroll Boss Blind unlimited times, $10 per reroll",
            Self::PaintBrush => "+1 hand size",
            Self::Palette => "+1 hand size",
        }
    }
}

/* This will represent a Voucher */
pub struct Voucher {
    voucher_type: VoucherType,
    cost: i16,
}

impl Voucher {
    pub fn get_name(&self) -> &'static str {
        self.voucher_type.get_name()
    }

    pub fn get_desc(&self) -> &'static str {
        self.voucher_type.get_desc()
    }
}

impl Parsable for Voucher {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Voucher")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Voucher")?;
        let cost: i16 = t
            .get("COST")
            .ok()
            .ok_or("Failed to parse cost while parsing Voucher")?;

        Ok(Voucher { voucher_type: VoucherType::from_str(&name), cost})
    }
}

impl Buyable for Voucher {
    fn get_item_name(&self) -> &str {
        "Voucher"
    }

    fn get_price(&self) -> i16 {
        self.cost
    }
}

impl fmt::Display for Voucher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.get_name();
        let d = &self.get_desc();
        write!(f, "Voucher:{n} | Ability:{d}")
    }
}
