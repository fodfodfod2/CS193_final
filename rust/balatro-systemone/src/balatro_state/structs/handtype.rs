use std::hash::Hash;

use strum::{EnumIter, IntoEnumIterator};

use crate::balatro_state::HandType::{FourOfAKind, HighCard};

use super::*;

/* This represents the different hands the player can level and play */
#[derive(Debug, Eq, Hash, PartialEq, EnumIter, Clone, Copy)]
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
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandType {
    /* This will get a handtype from a given str */
    pub fn from_str(string: &str) -> Self {
        match string {
            "High Card" => HandType::HighCard,
            "Pair" => HandType::Pair,
            "Two Pair" => HandType::TwoPair,
            "Three of a Kind" => HandType::ThreeOfAKind,
            "Straight" => HandType::Straight,
            "Flush" => HandType::Flush,
            "Full House" => HandType::FullHouse,
            "Four of a Kind" => HandType::FourOfAKind,
            "Straight Flush" => HandType::StraightFlush,
            "Five of a Kind" => HandType::FiveOfAKind,
            "Flush House" => HandType::FlushHouse,
            "Flush Five" => HandType::FlushFive,
            s => panic!("Unrecognized HandType \"{s}\""),
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
            HandType::FiveOfAKind => write!(f, "Five Of A Kind"),
            HandType::FlushHouse => write!(f, "Flush House"),
            HandType::FlushFive => write!(f, "Flush Five"),
            s => panic!("handtype {s} not recognized"),
        }
    }
}
