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
    assert_makes_hand([card(Ace, Spades), card(Queen, Spades), card(Jack, Spades), card(Nine, Spades), card(Eight, Hearts)], HiCard(HiCardStr{ranks: [Ace, Queen, Jack, Nine, Eight]}));
}
