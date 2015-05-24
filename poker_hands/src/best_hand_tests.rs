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

fn assert_best_hand(cards: &[Card], expected_hand: Hand) {
    for index in 1..cards.len() {
        assert!(cards[index].rank <= cards[index - 1].rank);
    }
    let actual_hand = Hand::best_hand_of(cards);
    assert_equal(&expected_hand, &actual_hand);
}

#[test]
fn find_straight_flushes() {
    let mut cards = [card(Ace, Spades), card(Ace, Hearts), card(Ace, Clubs), card(King, Hearts), card(Queen, Hearts), card(Jack, Hearts), card(Ten, Hearts)];
    assert_best_hand(&cards, StraightFlush(StraightFlushStr{hi_rank: Ace}));

    cards = [card(Ace, Spades), card(Ace, Clubs), card(King, Hearts), card(Queen, Hearts), card(Jack, Hearts), card(Ten, Hearts), card(Nine, Hearts)];
    assert_best_hand(&cards, StraightFlush(StraightFlushStr{hi_rank: King}));

    cards = [card(Ace, Hearts), card(Ace, Clubs), card(Queen, Hearts), card(Jack, Hearts), card(Ten, Hearts), card(Nine, Hearts), card(Eight, Hearts)];
    assert_best_hand(&cards, StraightFlush(StraightFlushStr{hi_rank: Queen}));

    cards = [card(Ten, Hearts), card(Nine, Hearts), card(Eight, Hearts), card(Seven, Hearts), card(Six, Hearts), card(Five, Hearts), card(Four, Hearts)];
    assert_best_hand(&cards, StraightFlush(StraightFlushStr{hi_rank: Ten}));

    cards = [card(Ace, Hearts), card(Seven, Spades), card(Six, Diamonds), card(Five, Hearts), card(Four, Hearts), card(Three, Hearts), card(Two, Hearts)];
    assert_best_hand(&cards, StraightFlush(StraightFlushStr{hi_rank: Five}));
}

#[test]
fn find_quads() {
    let mut cards = [card(Ace, Spades), card(Ace, Diamonds), card(Ace, Clubs), card(Two, Spades), card(Two, Hearts), card(Two, Diamonds), card(Two, Clubs)];
    assert_best_hand(&cards, Quads(QuadsStr{rank: Two, kicker: Ace}));

    cards = [card(Five, Spades), card(Four, Spades), card(Three, Spades), card(Two, Spades), card(Two, Hearts), card(Two, Diamonds), card(Two, Clubs)];
    assert_best_hand(&cards, Quads(QuadsStr{rank: Two, kicker: Five}));

    cards = [card(King, Spades), card(Eight, Spades), card(Three, Spades), card(Three, Hearts), card(Three, Diamonds), card(Three, Clubs), card(Two, Hearts)];
    assert_best_hand(&cards, Quads(QuadsStr{rank: Three, kicker: King}));
}

#[test]
fn find_boats() {
    let mut cards = [card(Ace, Spades), card(Ace, Diamonds), card(Ace, Clubs), card(Three, Spades), card(Two, Hearts), card(Two, Diamonds), card(Two, Clubs)];
    assert_best_hand(&cards, FullHouse(FullHouseStr{three_of: Ace, two_of: Two}));

    cards = [card(King, Clubs), card(Queen, Spades), card(Queen, Diamonds), card(Jack, Clubs), card(Jack, Spades), card(Jack, Hearts), card(Ten, Diamonds)];
    assert_best_hand(&cards, FullHouse(FullHouseStr{three_of: Jack, two_of: Queen}));

    cards = [card(Jack, Clubs), card(Jack, Spades), card(Nine, Diamonds), card(Nine, Clubs), card(Nine, Spades), card(Three, Hearts), card(Three, Diamonds)];
    assert_best_hand(&cards, FullHouse(FullHouseStr{three_of: Nine, two_of: Jack}));

    cards = [card(Ace, Hearts), card(Queen, Spades), card(Four, Hearts), card(Four, Clubs), card(Four, Spades), card(Two, Diamonds), card(Two, Clubs)];
    assert_best_hand(&cards, FullHouse(FullHouseStr{three_of: Four, two_of: Two}));
}
