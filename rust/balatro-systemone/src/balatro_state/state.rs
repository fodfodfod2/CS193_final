#![allow(unused)]

use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameState {
    state: u8,
    hand: Vec<Card>,
    vouchers_owned: Vec<Voucher>,
    jokers_owned: Vec<Joker>,
    tags_owned: Vec<Tag>,
    consumables: Vec<Consumable>,
    shop: Shop,
    pack: OpenPack,
    blinds: [Blind; 3],
    ctx: Ctx,
    deck: Vec<Card>
}


#[derive(Debug)]
struct Ctx {
    hands: i8,
    discards: i8,
    max_hands: i8,
    max_discards: i8,
    last_played_consumable: Consumable,
    hand_played: HashMap<HandType, bool>,
    hand_lvls: HashMap<HandType, i16>,
    most_played_hand: HandType,
}

#[derive(Debug)]
struct Shop {
    joker: Vec<Joker>,
    planet: Vec<Planet>,
    tarot: Vec<Tarot>,
    spectral: Vec<Spectral>,
    card: Vec<Card>,
    packs: Vec<ShopPack>,
    vouchers: Vec<Voucher>,
}

#[derive(Debug)]
enum Consumable {
    Planet(Planet),
    Tarot(Tarot),
    Spectral(Spectral),
}


/* This file will hold all of the pub structs and pub enums used for the game state */

#[derive(Debug)]
pub struct Card {
    rank:Rank,
    suit:Suit,
    enhancement:Enhancement,
    seal:Seal,
    disabled:bool,
    face_down:bool,
}

/* This will represent a Joker the player can own or buy */
#[derive(Debug)]
pub struct Joker {
    name:String,
    desc:String,
    edition:Edition,
    sell_value:i16,
    disabled:bool,
    invisible_cnt:i16,
    sticker:Sticker,
}

/* This will represent a Voucher */
#[derive(Debug)]
pub struct Voucher {
    name:String,
    desc:String,
}

impl Voucher {
    pub fn new(name:String, desc:String) -> Self {
        Voucher {
            name,
            desc,
        }
    }
}

/* This will represent a Planet card */
#[derive(Debug)]
pub struct Planet {
    name:String,
    desc:String,
    sell_value:i16,
    hand:HandType,
}

impl Planet {
    pub fn new(name:String, desc:String, sell_value:i16, hand:HandType)
        -> Self {
            Planet {
                name,
                desc,
                sell_value,
                hand,
            }
    }
}

/* This will represent a Tarot card */
#[derive(Debug)]
pub struct Tarot {
    name:String,
    desc:String,
    sell_value:i16,
}

impl Tarot {
    pub fn new(name:String, desc:String, sell_value:i16) -> Self {
        Tarot {
            name,
            desc,
            sell_value,
        }
    }
}

/* This will represent a spectral card */
#[derive(Debug)]
pub struct Spectral {
    name:String,
    desc:String,
    sell_value:i16,
}

impl Spectral {
    pub fn new(name:String, desc:String, sell_value:i16) -> Self {
        Spectral {
            name,
            desc,
            sell_value,
        }
    }
}

/* This will store a sticker applicable to jokers */
#[derive(Debug)]
pub struct Sticker {
    name:String,
    desc:String,
}

impl Sticker {
    pub fn new(name:String, desc:String) -> Self {
        Sticker {
            name,
            desc,
        }
    }
}

/* This will store an edition applicable to jokers */
#[derive(Debug)]
pub struct Edition {
    name:String,
    desc:String,
}

impl Edition {
    pub fn new(name:String, desc:String) -> Self {
        Edition {
            name,
            desc,
        }
    }
}

/* This will store a Seal applicable to cards */
#[derive(Debug)]
pub struct Seal {
    name:String,
    desc:String,
}

impl Seal {
    pub fn new(name:String, desc:String) -> Self {
        Seal {
            name,
            desc,
        }
    }
}

/* This will store a Enhancement applicable to cards */
#[derive(Debug)]
pub struct Enhancement {
    name:String,
    desc:String,
}

impl Enhancement {
    pub fn new(name:String, desc:String) -> Self {
        Enhancement {
            name,
            desc,
        }
    }
}

/* This represents possible skip tags the player may gain/be offered */
#[derive(Debug)]
pub struct Tag {
    name:String,
    desc:String,
}

impl Tag {
    pub fn new(name:String, desc:String) -> Self {
        Tag {
            name,
            desc,
        }
    }
}


#[derive(Debug)]
pub enum PackType {
    Card,
    Joker,
    Planet,
    Tarot,
    Spectral,
}

#[derive(Debug)]
pub struct ShopPack {
    pack_type: PackType,
    item_cnt: i8,
    price: i8,
    select_num: i8,
}

impl ShopPack {
    pub fn new(pack_type: PackType, item_cnt: i8, price: i8, select_num: i8)
        -> Self {
            ShopPack {
                pack_type,
                item_cnt,
                price,
                select_num,
            }
    }
}

#[derive(Debug)]
pub enum PackContents{
    Card(Vec<Card>),
    Joker(Vec<Joker>),
    Planet(Vec<Planet>),
    Tarot(Vec<Tarot>),
    Spectral(Vec<Spectral>),
}

#[derive(Debug)]
pub struct OpenPack {
    pack_type: PackType,
    select_num: i8,
    items: PackContents,
}

impl OpenPack {
    pub fn new(pack_type: PackType, select_num: i8, items: PackContents)
        -> Self {
            OpenPack {
                pack_type,
                select_num,
                items,
            }
    }
}


/* This represents possible blinds the player may face */
#[derive(Debug)]
pub enum Blind {
    Small(u64, Tag),
    Big(u64, Tag),
    Boss(u64, String, String),
}

/* This represents the different hands the player can level and play */
#[derive(Debug)]
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
}

/* This represents the possible ranks a card can be */
#[derive(Debug)]
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
    fn from_int(int: i16) -> Self {
        match  int {
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
#[derive(Debug)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
    Other,
}

impl Suit {
    fn from_str(string: &str) -> Self {
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
