use mlua::Table;

use super::*;

/* This will represent a Voucher */
pub struct Voucher {
    name: String,
    desc: String,
}

impl Voucher {
    pub fn new(name: String, desc: String) -> Self {
        Voucher { name, desc }
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

        Ok(Voucher { name, desc })
    }
}

impl fmt::Display for Voucher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "Voucher:{n} | Ability:{d}")
    }
}
