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
