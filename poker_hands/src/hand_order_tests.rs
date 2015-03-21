#![cfg(test)]

use super::*;
use cards::Rank::*;

#[test]
fn cmp_basic_hands() {
    let ordered_hands = [air(), pair(), two_pair(), trips(), straight(), flush(), boat(), quads(), str_flush()];
    for i in 0..(ordered_hands.len() - 1) {
        assert!(ordered_hands[i] < ordered_hands[i + 1]);
        assert!(ordered_hands[i] == ordered_hands[i]);
    }

    for i in 0..ordered_hands.len() {
        for j in 0..ordered_hands.len() {
            let expected_cmp = i.cmp(&j);
            let actual_cmp = ordered_hands[i].cmp(&ordered_hands[j]);
            assert!(expected_cmp == actual_cmp)
        }
    }

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
