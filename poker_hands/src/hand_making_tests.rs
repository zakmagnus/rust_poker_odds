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
fn air_making_test() {
    assert_makes_hand([card(Ace, Spades), card(Queen, Spades), card(Jack, Spades), card(Nine, Spades), card(Eight, Hearts)],
            HiCard(HiCardStr{ranks: [Ace, Queen, Jack, Nine, Eight]}));

    assert_makes_hand([card(Ten, Spades), card(Eight, Diamonds), card(Seven, Clubs), card(Four, Spades), card(Two, Hearts)],
            HiCard(HiCardStr{ranks: [Ten, Eight, Seven, Four, Two]}));

    assert_makes_hand([card(King, Spades), card(Five, Hearts), card(Four, Clubs), card(Three, Spades), card(Two, Spades)],
            HiCard(HiCardStr{ranks: [King, Five, Four, Three, Two]}));

    assert_makes_hand([card(Ace, Spades), card(Six, Hearts), card(Four, Clubs), card(Three, Spades), card(Two, Spades)],
            HiCard(HiCardStr{ranks: [Ace, Six, Four, Three, Two]}));
}

#[test]
fn pair_making_test() {
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
fn two_pair_making_test() {
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
fn trips_making_test() {
    assert_makes_hand([card(Jack, Spades), card(Jack, Hearts), card(Jack, Diamonds), card(Six, Clubs), card(Five, Spades)],
            Trips(TripsStr{rank: Jack, kickers: [Six, Five]}));

    assert_makes_hand([card(Ace, Clubs), card(Nine, Spades), card(Nine, Hearts), card(Nine, Diamonds), card(Five, Spades)],
            Trips(TripsStr{rank: Nine, kickers: [Ace, Five]}));

    assert_makes_hand([card(Ace, Clubs), card(King, Diamonds), card(Queen, Clubs), card(Queen, Diamonds), card(Queen, Spades)],
            Trips(TripsStr{rank: Queen, kickers: [Ace, King]}));
}

#[test]
fn straight_making_test() {
    assert_makes_hand([card(Queen, Clubs), card(Jack, Diamonds), card(Ten, Clubs), card(Nine, Diamonds), card(Eight, Spades)],
            Straight(StraightStr{hi_rank: Queen}));

    assert_makes_hand([card(Ace, Spades), card(King, Spades), card(Queen, Clubs), card(Jack, Spades), card(Ten, Spades)],
            Straight(StraightStr{hi_rank: Ace}));

    assert_makes_hand([card(Ace, Spades), card(Five, Hearts), card(Four, Clubs), card(Three, Spades), card(Two, Spades)],
            Straight(StraightStr{hi_rank: Five}));

    assert_makes_hand([card(Six, Diamonds), card(Five, Hearts), card(Four, Clubs), card(Three, Spades), card(Two, Spades)],
            Straight(StraightStr{hi_rank: Six}));

    assert_makes_hand([card(Eight, Clubs), card(Seven, Diamonds), card(Six, Diamonds), card(Five, Diamonds), card(Four, Diamonds)],
            Straight(StraightStr{hi_rank: Eight}));
}

#[test]
fn flush_making_test() {
    assert_makes_hand([card(Ace, Spades), card(Queen, Spades), card(Jack, Spades), card(Nine, Spades), card(Eight, Spades)],
            Flush(FlushStr{ranks: [Ace, Queen, Jack, Nine, Eight]}));

    assert_makes_hand([card(Ten, Diamonds), card(Eight, Diamonds), card(Seven, Diamonds), card(Four, Diamonds), card(Two, Diamonds)],
            Flush(FlushStr{ranks: [Ten, Eight, Seven, Four, Two]}));

    assert_makes_hand([card(King, Clubs), card(Five, Clubs), card(Four, Clubs), card(Three, Clubs), card(Two, Clubs)],
            Flush(FlushStr{ranks: [King, Five, Four, Three, Two]}));

    assert_makes_hand([card(Ace, Hearts), card(Six, Hearts), card(Four, Hearts), card(Three, Hearts), card(Two, Hearts)],
            Flush(FlushStr{ranks: [Ace, Six, Four, Three, Two]}));
}

#[test]
fn boat_making_test() {
    assert_makes_hand([card(Jack, Spades), card(Jack, Hearts), card(Jack, Diamonds), card(Six, Clubs), card(Six, Spades)],
            FullHouse(FullHouseStr{three_of: Jack, two_of: Six}));

    assert_makes_hand([card(Ace, Clubs), card(Ace, Diamonds), card(Nine, Spades), card(Nine, Hearts), card(Nine, Diamonds)],
            FullHouse(FullHouseStr{three_of: Nine, two_of: Ace}));

    assert_makes_hand([card(King, Clubs), card(King, Diamonds), card(Queen, Clubs), card(Queen, Diamonds), card(Queen, Spades)],
            FullHouse(FullHouseStr{three_of: Queen, two_of: King}));

    assert_makes_hand([card(King, Clubs), card(King, Diamonds), card(King, Hearts), card(Two, Diamonds), card(Two, Spades)],
            FullHouse(FullHouseStr{three_of: King, two_of: Two}));

    assert_makes_hand([card(Ace, Clubs), card(Ace, Diamonds), card(Two, Spades), card(Two, Hearts), card(Two, Diamonds)],
            FullHouse(FullHouseStr{three_of: Two, two_of: Ace}));

    assert_makes_hand([card(Ace, Clubs), card(Ace, Diamonds), card(Ace, Spades), card(Three, Hearts), card(Three, Diamonds)],
            FullHouse(FullHouseStr{three_of: Ace, two_of: Three}));
}

#[test]
fn quads_making_test() {
    assert_makes_hand([card(Ace, Clubs), card(Ace, Diamonds), card(Ace, Spades), card(Ace, Hearts), card(Three, Diamonds)],
            Quads(QuadsStr{rank: Ace, kicker: Three}));

    assert_makes_hand([card(Ace, Clubs), card(Three, Diamonds), card(Three, Spades), card(Three, Hearts), card(Three, Clubs)],
            Quads(QuadsStr{rank: Three, kicker: Ace}));

    assert_makes_hand([card(Seven, Clubs), card(Six, Diamonds), card(Six, Spades), card(Six, Hearts), card(Six, Clubs)],
            Quads(QuadsStr{rank: Six, kicker: Seven}));

    assert_makes_hand([card(Ten, Clubs), card(Ten, Diamonds), card(Ten, Spades), card(Ten, Hearts), card(Six, Clubs)],
            Quads(QuadsStr{rank: Ten, kicker: Six}));
}

#[test]
fn straight_flush_making_test() {
    assert_makes_hand([card(Queen, Clubs), card(Jack, Clubs), card(Ten, Clubs), card(Nine, Clubs), card(Eight, Clubs)],
            StraightFlush(StraightFlushStr{hi_rank: Queen}));

    assert_makes_hand([card(Ace, Spades), card(King, Spades), card(Queen, Spades), card(Jack, Spades), card(Ten, Spades)],
            StraightFlush(StraightFlushStr{hi_rank: Ace}));

    assert_makes_hand([card(Ace, Diamonds), card(Five, Diamonds), card(Four, Diamonds), card(Three, Diamonds), card(Two, Diamonds)],
            StraightFlush(StraightFlushStr{hi_rank: Five}));

    assert_makes_hand([card(Six, Hearts), card(Five, Hearts), card(Four, Hearts), card(Three, Hearts), card(Two, Hearts)],
            StraightFlush(StraightFlushStr{hi_rank: Six}));

    assert_makes_hand([card(Eight, Diamonds), card(Seven, Diamonds), card(Six, Diamonds), card(Five, Diamonds), card(Four, Diamonds)],
            StraightFlush(StraightFlushStr{hi_rank: Eight}));
}
