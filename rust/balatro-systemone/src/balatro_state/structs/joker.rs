use super::*;

/* This will store a sticker applicable to jokers */
pub struct Sticker {
    name: String,
    desc: String,
}

impl Sticker {
    pub fn new(name: String, desc: String) -> Self {
        Sticker { name, desc }
    }
}

impl fmt::Display for Sticker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "{n} | Ability: {d}")
    }
}

/* This will store an edition applicable to jokers */
pub struct Edition {
    name: String,
    desc: String,
}

impl Edition {
    pub fn new(name: String, desc: String) -> Self {
        Edition { name, desc }
    }
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "{n} | Ability: {d}")
    }
}

/* This will represent a Joker the player can own or buy */
pub struct Joker {
    name: String,
    desc: String,
    edition: Edition,
    sell_value: i16,
    disabled: bool,
    invisible_cnt: i16,
    perish_cnt: i16,
    sticker: Sticker,
}

impl Joker {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        desc: String,
        edition: Edition,
        sell_value: i16,
        disabled: bool,
        invisible_cnt: i16,
        perish_cnt: i16,
        sticker: Sticker,
    ) -> Self {
        Joker {
            name,
            desc,
            edition,
            sell_value,
            disabled,
            invisible_cnt,
            perish_cnt,
            sticker,
        }
    }
}

impl fmt::Display for Joker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        let e = &self.edition;
        let sv = &self.sell_value;
        let disabled = &self.disabled;
        let invis_cnt = &self.invisible_cnt;
        let perish_cnt = &self.perish_cnt;
        let st = &self.sticker;
        let ds = match disabled {
            true => "Disabled",
            false => "",
        };

        write!(
            f,
            " {ds} {n} | Ability: {d} | \n\t{e}\n\t{st}\n\tCan be sold for {sv}"
        )
    }
}
