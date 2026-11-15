use mlua::Table;

use super::*;

/* This will represent a Voucher */
pub struct Voucher {
    name: String,
    cost: i16,
    desc: String,
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

        Ok(Voucher { name, cost, desc })
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
        let n = &self.name;
        let d = &self.desc;
        write!(f, "Voucher:{n} | Ability:{d}")
    }
}
