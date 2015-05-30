extern crate rand;

use std::option::Option;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt::{Debug, Formatter};
use rand::{Rng, Rand};

mod tests;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

use Suit::*;
impl From<u8> for Suit {
    fn from(index: u8) -> Self {
        match index % 4 {
            0 => Spades,
            1 => Hearts,
            2 => Clubs,
            3 => Diamonds,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Rank {
    // The order here is used for comparison. Note the high ace.
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
    Queen,
    King,
    Ace,
}

use Rank::*;
impl Rank {
    pub fn all_ordered() -> [Rank; 13] {
        [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace]
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_index = *self as i32;
        let other_index = *other as i32;
        this_index.cmp(&other_index)
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Rank {}
impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl From<u8> for Rank {
    fn from(index: u8) -> Self {
        match index % 13 {
            0 => Two,
            1 => Three,
            2 => Four,
            3 => Five,
            4 => Six,
            5 => Seven,
            6 => Eight,
            7 => Nine,
            8 => Ten,
            9 => Jack,
            10 => Queen,
            11 => King,
            12 => Ace,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone)]
pub struct Card { pub suit: Suit, pub rank: Rank }
impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_ordering = self.rank.cmp(&other.rank);
        if rank_ordering != Ordering::Equal {
            return rank_ordering
        }
        return self.suit.cmp(&other.suit)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Card {}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

pub fn card(rank: Rank, suit: Suit) -> Card {
    Card{rank: rank, suit: suit}
}

impl Rand for Card {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let index = rng.gen_range(0, 51);
        let suit: Suit = Suit::from(index / 4);
        let rank: Rank = Rank::from(index % 4);
        card(rank, suit)
    }
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        ((self.suit as u8) * 4) + (self.rank as u8)
    }
}
