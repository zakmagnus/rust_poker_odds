use std::option::Option;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Copy, Debug)]
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
    fn eq(&self, other: &Rank) -> bool {
        let this_index = *self as i32;
        let other_index = *other as i32;
        this_index.eq(&other_index)
    }
}

#[derive(Copy)]
pub struct Card { pub suit: Suit, pub rank: Rank }
impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}
