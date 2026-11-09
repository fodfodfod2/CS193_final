#![allow(unused)]

use std::collections::HashMap;
use std::fmt;

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

pub struct Ctx {
    hands: i8,
    discards: i8,
    max_hands: i8,
    max_discards: i8,
    last_played_consumable: Consumable,
    hand_played: HashMap<HandType, bool>,
    hand_lvls: HashMap<HandType, i16>,
    most_played_hand: HandType,
}

impl Ctx {
    fn default() -> Self {
        Ctx {
            hands: -1,
            discards: -1,
            max_hands: -1,
            max_discards: -1,
            last_played_consumable: Consumable::Other,
            hand_played: HashMap::new(),
            hand_lvls: HashMap::new(),
            most_played_hand: HandType::HighCard,
        }
    }
}

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
    fn empty() -> Self {
        Shop {
            joker: Vec::new(),
            planet: Vec::new(),
            tarot: Vec::new(),
            spectral: Vec::new(),
            card: Vec::new(),
            packs: Vec::new(),
            vouchers: Vec::new(),
        }
    }
}

pub enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
    Other,
}

/* This file will hold all of the pub structs and pub enums used for the game state */

pub struct Card {
    rank: Rank,
    suit: Suit,
    enhancement: Enhancement,
    seal: Seal,
    disabled: bool,
    face_down: bool,
}

impl Card {
    pub fn new(
        rank: Rank,
        suit: Suit,
        enhancement: Enhancement,
        seal: Seal,
        disabled: bool,
        face_down: bool,
    ) -> Self {
        Card {
            rank,
            suit,
            enhancement,
            seal,
            disabled,
            face_down,
        }
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

/* This will represent a Planet card */
pub struct Planet {
    name: String,
    desc: String,
    sell_value: i16,
    hand: HandType,
}

impl Planet {
    pub fn new(name: String, desc: String, sell_value: i16, hand: HandType) -> Self {
        Planet {
            name,
            desc,
            sell_value,
            hand,
        }
    }
}

/* This will represent a Tarot card */
pub struct Tarot {
    name: String,
    desc: String,
    sell_value: i16,
}

impl Tarot {
    pub fn new(name: String, desc: String, sell_value: i16) -> Self {
        Tarot {
            name,
            desc,
            sell_value,
        }
    }
}

/* This will represent a spectral card */
pub struct Spectral {
    name: String,
    desc: String,
    sell_value: i16,
}

impl Spectral {
    pub fn new(name: String, desc: String, sell_value: i16) -> Self {
        Spectral {
            name,
            desc,
            sell_value,
        }
    }
}

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

/* This will store a Seal applicable to cards */
pub struct Seal {
    name: String,
    desc: String,
}

impl Seal {
    pub fn new(name: String, desc: String) -> Self {
        Seal { name, desc }
    }
}

/* This will store a Enhancement applicable to cards */
pub struct Enhancement {
    name: String,
    desc: String,
}

impl Enhancement {
    pub fn new(name: String, desc: String) -> Self {
        Enhancement { name, desc }
    }
}

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

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = &self.name;
        let d = &self.desc;
        write!(f, "Tag: {n} | Ability: {d}")
    }
}

pub enum PackType {
    Card,
    Joker,
    Planet,
    Tarot,
    Spectral,
    Other,
}

pub struct ShopPack {
    pack_type: PackType,
    item_cnt: i8,
    price: i8,
    select_num: i8,
}

impl ShopPack {
    pub fn new(pack_type: PackType, item_cnt: i8, price: i8, select_num: i8) -> Self {
        ShopPack {
            pack_type,
            item_cnt,
            price,
            select_num,
        }
    }
}

pub enum PackContents {
    Card(Vec<Card>),
    Joker(Vec<Joker>),
    Planet(Vec<Planet>),
    Tarot(Vec<Tarot>),
    Spectral(Vec<Spectral>),
    Other,
}

pub struct OpenPack {
    select_num: i8,
    items: PackContents,
}

impl OpenPack {
    pub fn new(select_num: i8, items: PackContents) -> Self {
        OpenPack { select_num, items }
    }
}

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
            _ => write!(f, "BLIND_NOT_RECOGNIZED"),
        }
    }
}

/* This represents the different hands the player can level and play */
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
    Other,
}

impl HandType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "High Card" => HandType::HighCard,
            "Pair" => HandType::Pair,
            "Two Pair" => HandType::TwoPair,
            "Three Of A Kind" => HandType::ThreeOfAKind,
            "Straight" => HandType::Straight,
            "Flush" => HandType::Flush,
            "Full House" => HandType::FullHouse,
            "Four Of A Kind" => HandType::FourOfAKind,
            "Straight Flush" => HandType::StraightFlush,
            "Royal Flush" => HandType::RoyalFlush,
            "Five Of A Kind" => HandType::FiveOfAKind,
            "Flush House" => HandType::FlushHouse,
            "Flush Five" => HandType::FlushFive,
            _ => {
                println!("Failed to match string to a hand type");
                HandType::Other
            }
        }
    }
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandType::HighCard => write!(f, "High Card"),
            HandType::Pair => write!(f, "Pair"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::ThreeOfAKind => write!(f, "Three Of A Kind"),
            HandType::Straight => write!(f, "Straight"),
            HandType::Flush => write!(f, "Flush"),
            HandType::FullHouse => write!(f, "Full House"),
            HandType::FourOfAKind => write!(f, "Four Of A Kind"),
            HandType::StraightFlush => write!(f, "Straight Flush"),
            HandType::RoyalFlush => write!(f, "Royal Flush"),
            HandType::FiveOfAKind => write!(f, "Five Of A Kind"),
            HandType::FlushHouse => write!(f, "Flush House"),
            HandType::FlushFive => write!(f, "Flush Five"),
            _ => write!(f, "HANDTYPE_NOT_RECOGNIZED"),
        }
    }
}

/* This represents the possible ranks a card can be */
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    King,
    Queen,
    Ace,
    Other,
}

impl Rank {
    pub fn from_int(int: i16) -> Self {
        match int {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            13 => Rank::King,
            12 => Rank::Queen,
            14 => Rank::Ace,
            _ => Rank::Other,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::King => write!(f, "King"),
            Rank::Queen => write!(f, "Queen"),
            Rank::Ace => write!(f, "Ace"),
            _ => write!(f, "RANK_NOT_RECOGNIZED"),
        }
    }
}

/* This represents the possible suits a card can be */
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
    Other,
}

impl Suit {
    pub fn from_str(string: &str) -> Self {
        match string {
            "Diamonds" => Suit::Diamonds,
            "Hearts" => Suit::Hearts,
            "Spades" => Suit::Spades,
            "Clubs" => Suit::Clubs,
            _ => Suit::Other,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Other => write!(f, "SUIT_NOT_RECOGNIZED"),
        }
    }
}

pub enum State {
    BlindSelect,
    Tutorial,
    SelectingHand,
    PlayTarot,
    HandPlayed,
    RoundEval,
    DrawToHand,
    Shop,
    TarotPack,
    PlanetPack,
    SpectralPack,
    StandardPack,
    BuffPack,
    MainMenu,
    GameOver,
    NewRound,
    Splash,
    RedeemCustomVoucher,
    CustomPack,
    Sandbox,
    Other,
}

impl State {
    pub fn from(int: i32) -> Self {
        match int {
            1 => State::SelectingHand,
            2 => State::HandPlayed,
            3 => State::DrawToHand,
            4 => State::GameOver,
            5 => State::Shop,
            6 => State::PlayTarot,
            7 => State::BlindSelect,
            8 => State::RoundEval,
            9 => State::TarotPack,
            10 => State::PlanetPack,
            11 => State::MainMenu,
            12 => State::Tutorial,
            13 => State::Splash,
            14 => State::Sandbox,
            15 => State::SpectralPack,
            17 => State::StandardPack,
            18 => State::BuffPack,
            19 => State::NewRound,
            998 => State::RedeemCustomVoucher,
            999 => State::CustomPack,
            _ => State::Other,
        }
    }

    pub fn is_decision_state(&self) -> bool {
        matches!(
            self,
            State::SelectingHand
                | State::TarotPack
                | State::PlanetPack
                | State::SpectralPack
                | State::StandardPack
                | State::BuffPack
                | State::Shop
        )
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::BlindSelect => write!(f, "Blind Select"),
            State::Tutorial => write!(f, "Tutorial"),
            State::SelectingHand => write!(f, "Selecting Hand"),
            State::PlayTarot => write!(f, "Play Tarot"),
            State::HandPlayed => write!(f, "Hand Played"),
            State::RoundEval => write!(f, "Round Eval"),
            State::DrawToHand => write!(f, "Draw To Hand"),
            State::Shop => write!(f, "Shop"),
            State::TarotPack => write!(f, "TarotP ack"),
            State::PlanetPack => write!(f, "Planet Pack"),
            State::SpectralPack => write!(f, "Spectral Pack"),
            State::StandardPack => write!(f, "Standard Pack"),
            State::BuffPack => write!(f, "Buffoon Pack"),
            State::MainMenu => write!(f, "Main Menu"),
            State::GameOver => write!(f, "Game Over"),
            State::NewRound => write!(f, "New Round"),
            State::Splash => write!(f, "Splash Screen"),
            State::RedeemCustomVoucher => write!(f, "Redeeming Custom Voucher"),
            State::CustomPack => write!(f, "Custom Pack Opening"),
            State::Sandbox => write!(f, "Sandbox"),
            _ => write!(f, "STATE_NOT_RECOGNIZED"),
        }
    }
}
