#![cfg(test)]

use super::*;
use super::Rank::*;
use std::cmp::Ordering;

#[test]
fn cmp_whole_deck() {
    let ordered_ranks = [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];

    for i in 0..12 {
        assert!(ordered_ranks[i] < ordered_ranks[i + 1]);
        assert!(ordered_ranks[i] == ordered_ranks[i]);
    }

    for rank1 in ordered_ranks.iter() {
        for rank2 in ordered_ranks.iter() {
            // Actually, two cards of the same rank may NOT be equal!
            if rank1 == rank2 {
                continue
            }
            cmp_two_ranks(*rank1, *rank2, rank1.cmp(rank2))
        }
    }
}

fn cmp_two_ranks(rank1: Rank, rank2: Rank, expected_ord: Ordering) {
    for card1 in all_cards_of_rank(rank1).iter() {
        for card2 in all_cards_of_rank(rank2).iter() {
            let actual_ord = card1.cmp(card2);
            if actual_ord != expected_ord {
                panic!("{:?} {:?} {:?}, but expected {:?}",
                    card1, actual_ord, card2, expected_ord)
            }
        }
    }
}

fn all_cards_of_rank(rank: Rank) -> [Card; 4] {
    [Card{rank: rank, suit: Suit::Spades},
     Card{rank: rank, suit: Suit::Hearts},
     Card{rank: rank, suit: Suit::Diamonds},
     Card{rank: rank, suit: Suit::Clubs}]
}
