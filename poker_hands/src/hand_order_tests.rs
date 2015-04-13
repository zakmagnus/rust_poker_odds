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
fn air_test() {
    let airer = |ranks: &[Rank; 5]| {
        if has_dups(ranks) {
            return None;
        }
        Some(Hand::HiCard(HiCardStr{ranks: *ranks}))
    };

    let rank_1 = King;
    let rank_2 = Jack;
    let rank_3 = Ten;
    let rank_4 = Four;
    let rank_5 = Three;
    cmp_by_rank(|&rank| { airer(&[rank, rank_2, rank_3, rank_4, rank_5]) });
    cmp_by_rank(|&rank| { airer(&[rank_1, rank, rank_3, rank_4, rank_5]) });
    cmp_by_rank(|&rank| { airer(&[rank_1, rank_2, rank, rank_4, rank_5]) });
    cmp_by_rank(|&rank| { airer(&[rank_1, rank_2, rank_3, rank, rank_5]) });
    cmp_by_rank(|&rank| { airer(&[rank_1, rank_2, rank_3, rank_4, rank]) });
}

#[test]
fn air_smoke_test() {
    let ordered_airs = [
        Hand::HiCard(HiCardStr{ranks: [Nine, Eight, Six, Three, Two]}),
        Hand::HiCard(HiCardStr{ranks: [Jack, Eight, Six, Three, Two]}),
        Hand::HiCard(HiCardStr{ranks: [Jack, Eight, Seven, Three, Two]}),
        Hand::HiCard(HiCardStr{ranks: [Jack, Nine, Seven, Three, Two]}),
        Hand::HiCard(HiCardStr{ranks: [King, Queen, Ten, Nine, Eight]}),
        Hand::HiCard(HiCardStr{ranks: [King, Queen, Jack, Ten, Two]}),
        Hand::HiCard(HiCardStr{ranks: [Ace, Six, Five, Three, Two]}),
        Hand::HiCard(HiCardStr{ranks: [Ace, Jack, Eight, Six, Two]}),
        Hand::HiCard(HiCardStr{ranks: [Ace, Jack, Eight, Six, Three]})
        ];
    cmp_order(&ordered_airs);
}

#[test]
fn pair_test() {
    let pairer = |rank, kickers: &[Rank; 3]| {
        if contains(kickers, &rank) {
            return None;
        }
        if has_dups(kickers) {
            return None;
        }
        Some(Hand::Pair(PairStr{rank: rank, kickers: *kickers}))
    };

    let pair_rank = Nine;
    let kicker_1 = Ace;
    let kicker_2 = Eight;
    let kicker_3 = Five;
    cmp_by_rank(|&rank| { pairer(pair_rank, &[kicker_1, kicker_2, rank]) });
    cmp_by_rank(|&rank| { pairer(pair_rank, &[kicker_1, rank, kicker_3]) });
    cmp_by_rank(|&rank| { pairer(pair_rank, &[rank, kicker_2, kicker_3]) });
    cmp_by_rank(|&rank| { pairer(rank, &[kicker_1, kicker_2, kicker_3]) });
}

fn contains<T: Eq>(list: &[T], thing: &T) -> bool {
    for listed_thing in list {
        if listed_thing == thing {
            return true;
        }
    }
    false
}

fn has_dups<T: Eq>(list: &[T]) -> bool {
    for i in 0..(list.len() - 1) {
        let this = &list[i];
        let next = &list[i + 1];
        if this == next {
            return true;
        }
    }
    false
}

#[test]
fn pair_smoke_test() {
    let ordered_pairs = [
        Hand::Pair(PairStr{rank: Two, kickers: [Ten, Five, Three]}),
        Hand::Pair(PairStr{rank: Two, kickers: [King, Queen, Ten]}),
        Hand::Pair(PairStr{rank: Two, kickers: [Ace, Five, Four]}),
        Hand::Pair(PairStr{rank: Two, kickers: [Ace, Jack, Ten]}),
        Hand::Pair(PairStr{rank: Four, kickers: [Six, Three, Two]}),
        Hand::Pair(PairStr{rank: Four, kickers: [King, Jack, Two]}),
        Hand::Pair(PairStr{rank: Four, kickers: [Ace, Jack, Ten]}),
        Hand::Pair(PairStr{rank: Nine, kickers: [Four, Three, Two]}),
        Hand::Pair(PairStr{rank: Nine, kickers: [King, Three, Two]}),
        Hand::Pair(PairStr{rank: Nine, kickers: [King, Jack, Two]}),
        Hand::Pair(PairStr{rank: Nine, kickers: [King, Jack, Ten]}),
        Hand::Pair(PairStr{rank: Queen, kickers: [Five, Three, Two]}),
        Hand::Pair(PairStr{rank: Queen, kickers: [King, Jack, Ten]}),
        Hand::Pair(PairStr{rank: Queen, kickers: [Ace, Three, Two]}),
        Hand::Pair(PairStr{rank: Ace, kickers: [Seven, Three, Two]}),
        Hand::Pair(PairStr{rank: Ace, kickers: [Queen, Ten, Nine]}),
        Hand::Pair(PairStr{rank: Ace, kickers: [King, Seven, Two]})
            ];
    cmp_order(&ordered_pairs);
}

#[test]
fn two_pair_test() {
    let two_pairer = |hi_rank: Rank, lo_rank: Rank, kicker: Rank| {
        if hi_rank == lo_rank ||
           hi_rank == kicker ||
           lo_rank == kicker {
            None
        } else {
            Some(Hand::TwoPair(TwoPairStr{hi_rank: hi_rank, lo_rank: lo_rank, kicker: kicker}))
        }
    };
    let hi_rank = Ten;
    let lo_rank = Two;
    let kicker = Ace;
    cmp_by_rank(|&rank| {
        two_pairer(rank, lo_rank, kicker)
    });
    cmp_by_rank(|&rank| {
        two_pairer(hi_rank, lo_rank, rank)
    });
    cmp_by_rank(|&rank| {
        two_pairer(hi_rank, rank, kicker)
    });
}

#[test]
fn two_pair_smoke_test() {
    let ordered_two_pairs = [
            Hand::TwoPair(TwoPairStr{hi_rank: Two, lo_rank: Queen, kicker: Three}),
            Hand::TwoPair(TwoPairStr{hi_rank: Two, lo_rank: Queen, kicker: Ten}),
            Hand::TwoPair(TwoPairStr{hi_rank: Two, lo_rank: Ace, kicker: Ten}),
            Hand::TwoPair(TwoPairStr{hi_rank: Five, lo_rank: Queen, kicker: Three}),
            Hand::TwoPair(TwoPairStr{hi_rank: Five, lo_rank: King, kicker: Queen}),
            Hand::TwoPair(TwoPairStr{hi_rank: Ten, lo_rank: Eight, kicker: Four}),
            Hand::TwoPair(TwoPairStr{hi_rank: Ten, lo_rank: Jack, kicker: Four}),
            Hand::TwoPair(TwoPairStr{hi_rank: Ten, lo_rank: King, kicker: Queen}),
            Hand::TwoPair(TwoPairStr{hi_rank: King, lo_rank: Jack, kicker: Four}),
            Hand::TwoPair(TwoPairStr{hi_rank: King, lo_rank: Ace, kicker: Four}),
            Hand::TwoPair(TwoPairStr{hi_rank: Ace, lo_rank: Three, kicker: Two}),
            Hand::TwoPair(TwoPairStr{hi_rank: Ace, lo_rank: Six, kicker: Two})
            ];
    cmp_order(&ordered_two_pairs);
}

#[test]
fn trips_test() {
    let tripper = |trips_rank: Rank, lo_kicker: Rank, hi_kicker: Rank| {
        if trips_rank == lo_kicker ||
           trips_rank == hi_kicker ||
           lo_kicker == hi_kicker {
            None
        } else {
            Some(Hand::Trips(TripsStr{rank: trips_rank, kickers: [hi_kicker, lo_kicker]}))
        }
    };
    let trips_rank = Ten;
    let lo_kicker = Two;
    let hi_kicker = Ace;
    cmp_by_rank(|&rank| {
        tripper(rank, lo_kicker, hi_kicker)
    });
    cmp_by_rank(|&rank| {
        tripper(trips_rank, lo_kicker, rank)
    });
    cmp_by_rank(|&rank| {
        tripper(trips_rank, rank, hi_kicker)
    });
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
