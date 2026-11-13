use mlua::Table;

use super::*;

/* This represents possible skip tags the player may gain/be offered */
pub struct Tag {
    name: String,
    desc: String,
}

impl Tag {
    pub fn new(name: String, desc: String) -> Self {
        Tag { name, desc }
    }

    pub fn default() -> Self {
        Tag {
            name: String::from(""),
            desc: String::from(""),
        }
    }
}

impl Parsable for Tag {
    fn parse(t: Table) -> Result<Self, &'static str> {
        let name: String = t
            .get("NAME")
            .ok()
            .ok_or("Failed to parse Name while parsing Enhancement")?;
        let desc: String = t
            .get("DESC")
            .ok()
            .ok_or("Failed to parse Desc while parsing Enhancement")?;

        Ok(Tag::new(name, desc))
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "Tag: {n} | Ability: {d}")
    }
}
