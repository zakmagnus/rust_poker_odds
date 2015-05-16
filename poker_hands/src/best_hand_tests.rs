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
    let cards = [card(Ace, Spades), card(Ace, Hearts), card(Ace, Clubs), card(King, Hearts), card(Queen, Hearts), card(Jack, Hearts), card(Ten, Hearts)];
    assert_best_hand(&cards, StraightFlush(StraightFlushStr{hi_rank: Ace}));
}
