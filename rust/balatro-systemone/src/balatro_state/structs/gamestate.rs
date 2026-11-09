use super::*;

pub struct GameState {
    state: State,
    hand: Vec<Card>,
    vouchers_owned: Vec<Voucher>,
    jokers_owned: Vec<Joker>,
    tags_owned: Vec<Tag>,
    consumables: Vec<Consumable>,
    shop: Shop,
    pack: OpenPack,
    blinds: [Blind; 3],
    current_blind: u8,
    ctx: Ctx,
    deck: Vec<Card>,
}

impl GameState {
    pub fn default() -> Self {
        GameState {
            state: State::Other,
            hand: Vec::new(),
            vouchers_owned: Vec::new(),
            jokers_owned: Vec::new(),
            tags_owned: Vec::new(),
            consumables: Vec::new(),
            shop: Shop::empty(),
            pack: OpenPack::new(0, PackContents::Other),
            blinds: [
                Blind::Small(0, Tag::default()),
                Blind::Big(0, Tag::default()),
                Blind::Boss(0, String::from(""), String::from("")),
            ],
            current_blind: 255,
            ctx: Ctx::default(),
            deck: Vec::new(),
        }
    }
}
