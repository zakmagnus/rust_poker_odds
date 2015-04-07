#![cfg(test)]

use std::fmt::Debug;
use super::*;
use cards::Rank;
use cards::Rank::*;

fn cmp_order<T: Ord + Debug>(list: &[T]) {
    for i in 0..(list.len() - 1) {
        let this = &list[i];
        let next = &list[i + 1];
        if this >= next {
            panic!("{:?} >= {:?}", this, next);
        }
        assert_eq!(this, this);
    }

    for i in 0..list.len() {
        for j in 0..list.len() {
            let expected_cmp = i.cmp(&j);
            let this = &list[i];
            let other = &list[j];
            let actual_cmp = this.cmp(other);
            if expected_cmp != actual_cmp {
                panic!("{:?} {:?} {:?}, but expected {:?}", this, actual_cmp, other,
                        expected_cmp);
            }
        }
    }
}

fn cmp_by_rank<T, M>(maker: M)
        where T: Ord + Debug,
              M: FnMut(&Rank) -> Option<T> {
    let ranks = Rank::all_ordered();
    let mut list: Vec<T> = Vec::with_capacity(13);
    let things = ranks.iter().filter_map(maker);
    for thing in things {
        list.push(thing);
    }
    cmp_order(&list);
}

#[test]
fn trips_smoke_test() {
    let ordered_tripses = [
            Hand::Trips(TripsStr{rank: Two, kickers: [Queen, Three]}),
            Hand::Trips(TripsStr{rank: Two, kickers: [Queen, Ten]}),
            Hand::Trips(TripsStr{rank: Two, kickers: [Ace, Ten]}),
            Hand::Trips(TripsStr{rank: Five, kickers: [Queen, Three]}),
            Hand::Trips(TripsStr{rank: Five, kickers: [King, Queen]}),
            Hand::Trips(TripsStr{rank: Ten, kickers: [Eight, Four]}),
            Hand::Trips(TripsStr{rank: Ten, kickers: [Jack, Four]}),
            Hand::Trips(TripsStr{rank: Ten, kickers: [King, Queen]}),
            Hand::Trips(TripsStr{rank: King, kickers: [Jack, Four]}),
            Hand::Trips(TripsStr{rank: King, kickers: [Ace, Four]}),
            Hand::Trips(TripsStr{rank: Ace, kickers: [Three, Two]}),
            Hand::Trips(TripsStr{rank: Ace, kickers: [Six, Two]})
            ];
    cmp_order(&ordered_tripses);
}

#[test]
fn boat_smoke_test() {
    let ordered_boats = [
        Hand::FullHouse(FullHouseStr{three_of: Two, two_of: Ace}),
        Hand::FullHouse(FullHouseStr{three_of: Five, two_of: Six}),
        Hand::FullHouse(FullHouseStr{three_of: King, two_of: Three}),
        Hand::FullHouse(FullHouseStr{three_of: Ace, two_of: Four})
            ];
    cmp_order(&ordered_boats);
}

#[test]
fn cmp_boats() {
    let two_of = Three;
    cmp_by_rank(|&three_of| {
        if three_of == two_of {
            // Impossible five-of-a-kind case
            None
        } else {
            Option::Some(Hand::FullHouse(FullHouseStr{three_of: three_of, two_of: two_of}))
        }
    });
}

#[test]
fn cmp_boats_by_two_of() {
    let three_of = Queen;
    cmp_by_rank(|&two_of| {
        if three_of == two_of {
            // Impossible five-of-a-kind case
            None
        } else {
            Option::Some(Hand::FullHouse(FullHouseStr{three_of: three_of, two_of: two_of}))
        }
    });
}

#[test]
fn quads_smoke_test() {
    let ordered_quads = [
        Hand::Quads(QuadsStr{rank: Two, kicker: Ace}),
        Hand::Quads(QuadsStr{rank: Five, kicker: Six}),
        Hand::Quads(QuadsStr{rank: King, kicker: Three}),
        Hand::Quads(QuadsStr{rank: Ace, kicker: Four})
            ];
    cmp_order(&ordered_quads);
}

#[test]
fn cmp_quads() {
    let kicker = Three;
    cmp_by_rank(|&quad_rank| {
        if quad_rank == kicker {
            // Impossible five-of-a-kind case
            None
        } else {
            Option::Some(Hand::Quads(QuadsStr{rank: quad_rank, kicker: kicker}))
        }
    });
}

#[test]
fn cmp_quad_kickers() {
    let quad_rank = Queen;
    cmp_by_rank(|&kicker| {
        if quad_rank == kicker {
            // Impossible five-of-a-kind case
            None
        } else {
            Option::Some(Hand::Quads(QuadsStr{rank: quad_rank, kicker: kicker}))
        }
    });
}

#[test]
fn cmp_straights() {
    cmp_by_rank(|&rank| {
        if rank < Five {
            // Impossibly low straight
            None
        } else {
            Option::Some(Hand::Straight(StraightStr{hi_rank: rank}))
        }
    });
}

#[test]
fn cmp_str_flushes() {
    cmp_by_rank(|&rank| {
        if rank < Five {
            // Impossibly low straight flush
            None
        } else {
            Option::Some(Hand::StraightFlush(StraightFlushStr{hi_rank: rank}))
        }
    });
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
