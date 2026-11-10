use super::*;

/* This represents possible blinds the player may face */
pub enum Blind {
    Small(u64, Tag),
    Big(u64, Tag),
    Boss(u64, String, String),
}

impl fmt::Display for Blind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blind::Small(s, t) => write!(f, "Small Blind Score:{s} Skip tag:{t}"),
            Blind::Big(s, t) => write!(f, "Big Blind Score:{s} Skip tag:{t}"),
            Blind::Boss(s, n, d) => write!(f, "Boss:{n}, Ability:{d} Blind Score:{s}"),
        }
    }
}
