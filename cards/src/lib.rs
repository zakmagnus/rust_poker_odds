use std::option::Option;
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::fmt::{Show, Formatter};

pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[deriving(Eq, Ord)]
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

impl Show for Suit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = match *self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        };
        write!(f, "{}", string)
    }
}

impl Show for Rank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = match *self {
            Rank::Ace => "Ace",
            Rank::King => "King",
            Rank::Queen => "Queen",
            Rank::Jack => "Jack",
            Rank::Ten => "Ten",
            Rank::Nine => "Nine",
            Rank::Eight => "Eight",
            Rank::Seven => "Seven",
            Rank::Six => "Six",
            Rank::Five => "Five",
            Rank::Four => "Four",
            Rank::Three => "Three",
            Rank::Two => "Two",
        };
        write!(f, "{}", string)
    }
}

pub struct Card { suit: Suit, rank: Rank }
impl Show for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
