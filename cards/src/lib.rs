use std::option::Option;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt::{Debug, Formatter};

mod tests;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
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
