#![cfg(test)]

use std::fmt::Debug;
use super::*;
use super::Hand::*;

use cards::Card;
use cards::Rank::*;
use cards::Suit::*;
use cards::card;

fn assert_equal<T: Eq + Debug>(expected: &T, actual: &T) {
    assert!(expected == actual, "Expected {:?} but got {:?}", expected, actual)
}

fn assert_makes_hand(cards: [Card; 5], hand: Hand) {
    let actual_hand = Hand::get_hand(&cards);
    assert_equal(&hand, &actual_hand);
}

#[test]
fn air_test() {
    assert_makes_hand([card(Ace, Spades), card(Queen, Spades), card(Jack, Spades), card(Nine, Spades), card(Eight, Hearts)],
            HiCard(HiCardStr{ranks: [Ace, Queen, Jack, Nine, Eight]}));

    assert_makes_hand([card(Ten, Spades), card(Eight, Diamonds), card(Seven, Clubs), card(Four, Spades), card(Two, Hearts)],
            HiCard(HiCardStr{ranks: [Ten, Eight, Seven, Four, Two]}));
}

#[test]
fn pair_test() {
    assert_makes_hand([card(Ace, Spades), card(Ace, Hearts), card(Jack, Spades), card(Nine, Spades), card(Eight, Hearts)],
            Pair(PairStr{rank: Ace, kickers: [Jack, Nine, Eight]}));

    assert_makes_hand([card(Queen, Spades), card(Jack, Hearts), card(Jack, Spades), card(Six, Spades), card(Two, Spades)],
            Pair(PairStr{rank: Jack, kickers: [Queen, Six, Two]}));

    assert_makes_hand([card(Queen, Spades), card(Jack, Hearts), card(Ten, Diamonds), card(Ten, Clubs), card(Two, Spades)],
            Pair(PairStr{rank: Ten, kickers: [Queen, Jack, Two]}));

    assert_makes_hand([card(Queen, Spades), card(Jack, Hearts), card(Ten, Diamonds), card(Six, Clubs), card(Six, Spades)],
            Pair(PairStr{rank: Six, kickers: [Queen, Jack, Ten]}));
}

#[test]
fn two_pair_test() {
    assert_makes_hand([card(Queen, Spades), card(Jack, Hearts), card(Jack, Diamonds), card(Six, Clubs), card(Six, Spades)],
            TwoPair(TwoPairStr{hi_rank: Jack, lo_rank: Six, kicker: Queen}));

    assert_makes_hand([card(King, Spades), card(King, Hearts), card(Queen, Diamonds), card(Jack, Clubs), card(Jack, Spades)],
            TwoPair(TwoPairStr{hi_rank: King, lo_rank: Jack, kicker: Queen}));

    assert_makes_hand([card(Nine, Diamonds), card(Nine, Spades), card(Eight, Diamonds), card(Eight, Clubs), card(Two, Spades)],
            TwoPair(TwoPairStr{hi_rank: Nine, lo_rank: Eight, kicker: Two}));

    assert_makes_hand([card(Ace, Diamonds), card(Three, Spades), card(Three, Diamonds), card(Two, Clubs), card(Two, Spades)],
            TwoPair(TwoPairStr{hi_rank: Three, lo_rank: Two, kicker: Ace}));
}

#[test]
fn trips_test() {
    assert_makes_hand([card(Jack, Spades), card(Jack, Hearts), card(Jack, Diamonds), card(Six, Clubs), card(Five, Spades)],
            Trips(TripsStr{rank: Jack, kickers: [Six, Five]}));

    assert_makes_hand([card(Ace, Clubs), card(Nine, Spades), card(Nine, Hearts), card(Nine, Diamonds), card(Five, Spades)],
            Trips(TripsStr{rank: Nine, kickers: [Ace, Five]}));

    assert_makes_hand([card(Ace, Clubs), card(King, Diamonds), card(Queen, Clubs), card(Queen, Diamonds), card(Queen, Spades)],
            Trips(TripsStr{rank: Queen, kickers: [Ace, King]}));
}
