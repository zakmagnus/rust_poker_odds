#![cfg(test)]

use super::*;
use cards::Rank::*;

fn cmp_order<T: Ord>(list: &[T]) {
    for i in 0..(list.len() - 1) {
        assert!(list[i] < list[i + 1]);
        assert!(list[i] == list[i]);
    }

    for i in 0..list.len() {
        for j in 0..list.len() {
            let expected_cmp = i.cmp(&j);
            let actual_cmp = list[i].cmp(&list[j]);
            assert!(expected_cmp == actual_cmp)
        }
    }
}

#[test]
fn cmp_basic_hands() {
    let ordered_hands = [air(), pair(), two_pair(), trips(), straight(), flush(), boat(), quads(), str_flush()];
    cmp_order(&ordered_hands)
}
fn str_flush() -> Hand {
    Hand::StraightFlush(StraightFlushStr{hi_rank: Jack})
}
fn quads() -> Hand {
    Hand::Quads(QuadsStr{rank: Ace, kicker: Five})
}
fn boat() -> Hand {
    Hand::FullHouse(FullHouseStr{three_of: Queen, two_of: Ace})
}
fn flush() -> Hand {
    Hand::Flush(FlushStr{ranks: [King, Jack, Ten, Six, Two]})
}
fn straight() -> Hand {
    Hand::Straight(StraightStr{hi_rank: Eight})
}
fn trips() -> Hand {
    Hand::Trips(TripsStr{rank: Three, kickers: [King, Ten]})
}
fn two_pair() -> Hand {
    Hand::TwoPair(TwoPairStr{hi_rank: Ace, lo_rank: Ten, kicker: Jack})
}
fn pair() -> Hand {
    Hand::Pair(PairStr{rank: Ten, kickers: [Ace, Five, Two]})
}
fn air() -> Hand {
    Hand::HiCard(HiCardStr{ranks: [King, Jack, Seven, Four, Two]})
}
