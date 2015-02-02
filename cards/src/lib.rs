#![feature(core)]

use std::option::Option;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt::{Debug, Formatter};

#[derive(Copy, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Eq, Ord, Copy, Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

// This uses a high ace.
fn rank_to_index(rank: & Rank) -> u8 {
    match *rank {
        Rank::Ace => 14,
        Rank::King => 13,
        Rank::Queen => 12,
        Rank::Jack => 11,
        Rank::Ten => 10,
        Rank::Nine => 9,
        Rank::Eight => 8,
        Rank::Seven => 7,
        Rank::Six => 6,
        Rank::Five => 5,
        Rank::Four => 4,
        Rank::Three => 3,
        Rank::Two => 2,
    }
}

// Ace is high in this ordering.
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this_index = rank_to_index(self);
        let other_index = rank_to_index(other);
        let comparison = this_index.cmp(&other_index);
        Option::Some(comparison)
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Rank) -> bool {
        let this_index = rank_to_index(self);
        let other_index = rank_to_index(other);
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
