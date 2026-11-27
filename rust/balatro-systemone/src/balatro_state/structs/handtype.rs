use std::{fmt::write, hash::Hash};

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

use crate::balatro_state::HandType::{FourOfAKind, HighCard};

use super::*;

/* This represents the different hands the player can level and play */
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, EnumIter, Clone, Copy)]
pub enum HandType {
    None,
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

    fn holds_card(c: &Vec<&Card>, ignored_idx: Vec<i8>, s: Option<Suit>, r: Option<Rank>) -> i8 {
        let mut res = 0;
        'outer: for i in 0..c.len() {
            for j in 0..ignored_idx.len() {
                if i as i8 == ignored_idx[j] {
                    continue 'outer;
                }
            }
            let mut matches_pat = true;
            if let Some(s) = s {
                if s != *c[i].get_suit() {
                    matches_pat = false;
                }
            }
            if let Some(r) = r {
                if r != *c[i].get_rank() {
                    matches_pat = false;
                }
            }

            if matches_pat {
                res += 1;
            }
        }
        res
    }

    fn has_straight(c: &Vec<&Card>) -> bool {
        if c.len() != 5 {
            return false;
        }

        /* Check for wraparound straights */
        if ((Self::holds_card(c, Vec::new(), None, Some(Rank::Two)) > 0)
            && (Self::holds_card(c, Vec::new(), None, Some(Rank::King)) > 0))
        {
            return false;
        }

        let mut buffer = 2;
        for i in 0..c.len() {
            let next_rank = Rank::from_int((((c[i].get_rank().to_int() + 1) % 13) + 2) as i16);
            let last_rank = Rank::from_int((((c[i].get_rank().to_int() - 1 + 13) % 13) + 2) as i16);
            if Self::holds_card(c, vec![i as i8], None, Some(next_rank)) != 1 {
                if buffer > 0 {
                    buffer -= 1;
                } else {
                    return false;
                }
            }

            if Self::holds_card(c, vec![i as i8], None, Some(last_rank)) != 1 {
                if buffer > 0 {
                    buffer -= 1;
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn from_cards(c: &Vec<&Card>) -> Self {
        if c.is_empty() || c.len() > 5 {
            panic!("Invalid hand size");
        }

        let mut hands: HashMap<HandType, bool> = HashMap::new();
        for ht in HandType::iter() {
            hands.insert(ht, true);
        }

        if Self::holds_card(c, vec![0], Some(*c[0].get_suit()), None) != 4 {
            hands.insert(HandType::FlushFive, false);
            hands.insert(HandType::FlushHouse, false);
            hands.insert(HandType::StraightFlush, false);
            hands.insert(HandType::Flush, false);
        }

        let mut pairs: Vec<i8> = vec![0, 0, 0, 0, 0];
        for i in 0..c.len() {
            pairs[Self::holds_card(c, vec![i as i8], None, Some(*c[i].get_rank())) as usize] += 1;
        }

        if pairs[4] != 5 {
            hands.insert(Self::FiveOfAKind, false);
            hands.insert(HandType::FlushFive, false);
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

    pub fn get_possible_hands(num: usize, hand: &Vec<Card>) -> Vec<(Vec<&Card>, Self)> {
        let combos: Vec<Vec<&Card>> = hand.iter().combinations(5).collect();
        let mut res = Vec::new();

        for i in 0..num {
            res.push((Vec::new(), HandType::None));
        }

        for c in combos {
            let ht = Self::from_cards(&c);
            res.push((c, ht));
        }
        res.sort_by_key(|x| std::cmp::Reverse(x.1));
        res.into_iter()
            .enumerate()
            .filter_map(|(i, (c, ht))| {
                if ht != HandType::None && i < num {
                    Some((c, ht))
                } else {
                    None
                }
            })
            .collect()
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
            HandType::None => write!(f, "NONE ERR"),
        }
    }
}
