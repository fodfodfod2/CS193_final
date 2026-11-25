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
    // RoyalFlush,
    // /* Royal flush is apparrently just a straight flush in balatro */
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandType {
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

    fn holds_card(c: &Vec<Card>, ignored_idx: Vec<i8>, s: Option<Suit>, r: Option<Rank>) -> i8 {
        let mut res = 0;
        'outer: for i in 0..c.len() {
            for j in 0..ignored_idx.len() {
                if i == j {
                    continue 'outer;
                }
            }
            let mut matches_pat = true;
            if let Some(s) = s
                && s != *c[i].get_suit() {
                    matches_pat = false;
                }
            if let Some(r) = r
                && r != *c[i].get_rank() {
                    matches_pat = false;
                }

            if matches_pat {
                res += 1;
            }
        }
        res
    }

    fn has_straight(c: &Vec<Card>) -> bool {
        if c.len() != 5 {
            return false;
        }

        let mut buffer = true;
        for i in 0..c.len() {
            if Self::holds_card(c, vec![i as i8], None, Some(*c[i].get_rank())) != 1 {
                if buffer {
                    buffer = false;
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn from_cards(c: Vec<Card>) -> Self {
        if c.is_empty() || c.len() > 5 {
            panic!("Invalid hand size");
        }

        let mut hands: HashMap<HandType, bool> = HashMap::new();
        for ht in HandType::iter() {
            hands.insert(ht, true);
        }

        if Self::holds_card(&c, vec![0], Some(*c[0].get_suit()), None) != 4 {
            hands.insert(HandType::FlushFive, false);
            hands.insert(HandType::FlushHouse, false);
            hands.insert(HandType::StraightFlush, false);
            hands.insert(HandType::Flush, false);
        }

        let mut pairs: Vec<i8> = vec![0, 0, 0, 0, 0];
        for i in 0..c.len() {
            pairs[Self::holds_card(&c, vec![0], None, Some(*c[i].get_rank())) as usize] += 1;
        }

        if pairs[4] != 5 {
            hands.insert(Self::FiveOfAKind, false);
        }
        if pairs[3] != 4 {
            hands.insert(Self::FourOfAKind, false);
        }
        if pairs[2] != 3 {
            hands.insert(Self::ThreeOfAKind, false);
            hands.insert(Self::FullHouse, false);
            hands.insert(Self::FlushHouse, false);
        }
        if pairs[1] != 2 {
            hands.insert(Self::Pair, false);
            hands.insert(Self::FullHouse, false);
            hands.insert(Self::FlushHouse, false);
        }
        if pairs[1] != 4 {
            hands.insert(Self::TwoPair, false);
        }

        if !Self::has_straight(&c) {
            hands.insert(Self::Straight, false);
            hands.insert(Self::StraightFlush, false);
        }

        let mut res = Self::HighCard;
        for ht in HandType::iter() {
            if *hands.get(&ht).unwrap() {
                res = ht;
            }
        }
        res
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
