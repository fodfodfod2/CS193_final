

/* This file will hold all of the pub structs and pub enums used for the game state */

#[derive(Debug)]
pub struct Card<'a> {
    rank:Rank,
    suit:Suit,
    enhancement:Enhancement<'a>,
    seal:Seal<'a>,
    disabled:bool,
    face_down:bool,
}

/* This will represent a Joker the player can own or buy */
#[derive(Debug)]
pub struct Joker<'a> {
    name:&'a str,
    desc:&'a str,
    edition:Edition<'a>,
    sell_value:i16,
    disabled:bool,
    invisible_cnt:i16,
    sticker:Sticker<'a>,
}

/* This will represent a Voucher */
#[derive(Debug)]
pub struct Voucher<'a> {
    name:&'a str,
    desc:&'a str,
}

impl<'a> Voucher<'a> {
    pub fn new(name:&'a str, desc:&'a str) -> Self {
        Voucher {
            name,
            desc,
        }
    }
}

/* This will represent a Planet card */
#[derive(Debug)]
pub struct Planet<'a> {
    name:&'a str,
    desc:&'a str,
    sell_value:i16,
    hand:HandType,
}

impl<'a> Planet<'a> {
    pub fn new(name:&'a str, desc:&'a str, sell_value:i16, hand:HandType)
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
pub struct Tarot<'a> {
    name:&'a str,
    desc:&'a str,
    sell_value:i16,
}

impl<'a> Tarot<'a> {
    pub fn new(name:&'a str, desc:&'a str, sell_value:i16) -> Self {
        Tarot {
            name,
            desc,
            sell_value,
        }
    }
}

/* This will represent a spectral card */
#[derive(Debug)]
pub struct Spectral<'a> {
    name:&'a str,
    desc:&'a str,
    sell_value:i16,
}

impl<'a> Spectral<'a> {
    pub fn new(name:&'a str, desc:&'a str, sell_value:i16) -> Self {
        Spectral {
            name,
            desc,
            sell_value,
        }
    }
}

/* This will store a sticker applicable to jokers */
#[derive(Debug)]
pub struct Sticker<'a> {
    name:&'a str,
    desc:&'a str,
}

impl<'a> Sticker<'a> {
    pub fn new(name:&'a str, desc:&'a str) -> Self {
        Sticker {
            name,
            desc,
        }
    }
}

/* This will store an edition applicable to jokers */
#[derive(Debug)]
pub struct Edition<'a> {
    name:&'a str,
    desc:&'a str,
}

impl<'a> Edition<'a> {
    pub fn new(name:&'a str, desc:&'a str) -> Self {
        Edition {
            name,
            desc,
        }
    }
}

/* This will store a Seal applicable to cards */
#[derive(Debug)]
pub struct Seal<'a> {
    name:&'a str,
    desc:&'a str,
}

impl<'a> Seal<'a> {
    pub fn new(name:&'a str, desc:&'a str) -> Self {
        Seal {
            name,
            desc,
        }
    }
}

/* This will store a Enhancement applicable to cards */
#[derive(Debug)]
pub struct Enhancement<'a> {
    name:&'a str,
    desc:&'a str,
}

impl<'a> Enhancement<'a> {
    pub fn new(name:&'a str, desc:&'a str) -> Self {
        Enhancement {
            name,
            desc,
        }
    }
}

/* This represents possible skip tags the player may gain/be offered */
#[derive(Debug)]
pub struct Tag<'a> {
    name:&'a str,
    desc:&'a str,
}

impl<'a> Tag<'a> {
    pub fn new(name:&'a str, desc:&'a str) -> Self {
        Tag {
            name,
            desc,
        }
    }
}

/* This represents possible blinds the player may face */
#[derive(Debug)]
pub enum Blind<'a> {
    Small(u64, Tag<'a>),
    Big(u64, Tag<'a>),
    Boss(u64, &'a str, &'a str),
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
}

/* This represents the possible suits a card can be */
#[derive(Debug)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}
