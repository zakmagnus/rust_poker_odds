extern crate cards;

use cards::Rank;
use std::fmt::{Show, Formatter};

// Rank arrays are used for kickers. They should be sorted descending.

#[deriving(Eq, Ord)]
enum Hand {
    HiCard        {ranks: [Rank, ..5]},
    Pair          {rank: Rank, kickers: [Rank, ..5]},
    TwoPair       {hi_rank: Rank, lo_rank: Rank, kicker: Rank},
    Trips         {rank: Rank, kickers: [Rank, ..2]},
    Straight      {hi_rank: Rank},
    Flush         {ranks: [Rank, ..5]},
    FullHouse     {three_of: Rank, two_of: Rank},
    Quads         {rank: Rank, kicker: Rank},
    StraightFlush {hi_rank: Rank},
}

fn hand_to_index(hand: & Hand) -> u8 {
    match *hand {
        Hand::HiCard{..} => 0,
        Hand::Pair{..} => 1,
        Hand::TwoPair{..} => 2,
        Hand::Trips{..} => 3,
        Hand::Straight{..} => 4,
        Hand::Flush{..} => 5,
        Hand::FullHouse{..} => 6,
        Hand::Quads{..} => 7,
        Hand::StraightFlush{..} => 8,
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Pick the best hand type, if they're different.
        let this_index = hand_to_index(self);
        let other_index = hand_to_index(other);
        let initial_comparison = this_index.cmp(&other_index);
        if initial_comparison != Ordering::Equal {
            return Option::Some(initial_comparison);
        }

        // Same type of hand; tiebreak if possible.
        Option::Some(cmp_same_type_hand(self, other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        let cmp_maybe = self.partial_cmp(other);
        match cmp_maybe {
            Some(cmp) => cmp == Ordering::Equal,
            None => panic!("partial_cmp() failed to order hands {} and {}", self, other)
        }
    }
}

fn cmp_same_type_hand(this: & Hand, other: & Hand) -> Ordering {
    // capacity = max number of comparables = hand size
    let mut this_comparable_buffer = Vec::with_capacity(5);
    let mut other_comparable_buffer = Vec::with_capacity(5);

    match (this, other) {
        (&Hand::HiCard{ranks: ref these_ranks}, &Hand::HiCard{ranks: ref other_ranks}) => {
            this_comparable_buffer.push_all(these_ranks);
            other_comparable_buffer.push_all(other_ranks);
        },
        (&Hand::Pair{rank: ref this_rank, kickers: ref these_kickers}, &Hand::Pair{rank: ref other_rank, kickers: ref other_kickers}) => {
            assert_eq!(3, these_kickers.len());
            assert_eq!(3, other_kickers.len());

            this_comparable_buffer.push(this_rank.clone());
            other_comparable_buffer.push(other_rank.clone());

            this_comparable_buffer.push_all(these_kickers);
            other_comparable_buffer.push_all(other_kickers);
        },
        (&Hand::TwoPair{hi_rank: ref this_hi_rank, lo_rank: ref this_lo_rank, kicker: ref this_kicker}, &Hand::TwoPair{hi_rank: ref other_hi_rank, lo_rank: ref other_lo_rank, kicker: ref other_kicker}) => {
            this_comparable_buffer.push(this_hi_rank.clone());
            other_comparable_buffer.push(other_hi_rank.clone());

            this_comparable_buffer.push(this_lo_rank.clone());
            other_comparable_buffer.push(other_lo_rank.clone());

            this_comparable_buffer.push(this_kicker.clone());
            other_comparable_buffer.push(other_kicker.clone());
        },
        (&Hand::Trips{rank: ref this_rank, kickers: ref these_kickers}, &Hand::Trips{rank: ref other_rank, kickers: ref other_kickers}) => {
            assert_eq!(2, these_kickers.len());
            assert_eq!(2, other_kickers.len());

            this_comparable_buffer.push(this_rank.clone());
            other_comparable_buffer.push(other_rank.clone());

            this_comparable_buffer.push_all(these_kickers);
            other_comparable_buffer.push_all(other_kickers);
        },
        (&Hand::Straight{hi_rank: ref this_rank}, &Hand::Straight{hi_rank: ref other_rank}) => {
            this_comparable_buffer.push(this_rank.clone());
            other_comparable_buffer.push(other_rank.clone());
        },
        (&Hand::Flush{ranks: ref these_ranks}, &Hand::Flush{ranks: ref other_ranks}) => {
            this_comparable_buffer.push_all(these_ranks);
            other_comparable_buffer.push_all(other_ranks);
        },
        (&Hand::FullHouse{three_of: ref this_three_of, two_of: ref this_two_of}, &Hand::FullHouse{three_of: ref other_three_of, two_of: ref other_two_of}) => {
            this_comparable_buffer.push(this_three_of.clone());
            other_comparable_buffer.push(other_three_of.clone());

            this_comparable_buffer.push(this_two_of.clone());
            other_comparable_buffer.push(other_two_of.clone());
        },
        (&Hand::Quads{rank: ref this_rank, kicker: ref this_kicker}, &Hand::Quads{rank: ref other_rank, kicker: ref other_kicker}) => {
            this_comparable_buffer.push(this_rank.clone());
            other_comparable_buffer.push(other_rank.clone());

            this_comparable_buffer.push(this_kicker.clone());
            other_comparable_buffer.push(other_kicker.clone());
        },
        (&Hand::StraightFlush{hi_rank: ref this_rank}, &Hand::StraightFlush{hi_rank: ref other_rank}) => {
            this_comparable_buffer.push(this_rank.clone());
            other_comparable_buffer.push(other_rank.clone());
        },

        // Logic error case where the hands are different types
        (_, _) => { panic!("Different hand types passed to cmp_same_type_hand()!\
                         {} {}", this, other) },
    };
    assert_eq!(this_comparable_buffer.len(), other_comparable_buffer.len());
    this_comparable_buffer.cmp(&other_comparable_buffer)
}

impl Show for Hand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = match *self {
            Hand::HiCard{ref ranks} => format!("{}", ranks),
            Hand::Pair{ref rank, ref kickers} => format!("Pair of {}s, {} kickers", rank, kickers),
            Hand::TwoPair{ref hi_rank, ref lo_rank, ref kicker} => format!("Two pair, {} and {}, {} kicker", hi_rank, lo_rank, kicker),
            Hand::Trips{ref rank, ref kickers} => format!("Trip {}s, {} kickers", rank, kickers),
            Hand::Straight{ref hi_rank} => format!("{}-high straight", hi_rank),
            Hand::Flush{ref ranks} => format!("Flush of {}", ranks),
            Hand::FullHouse{ref three_of, ref two_of} => format!("Full house, three {}s, two {}s", three_of, two_of),
            Hand::Quads{ref rank, ref kicker} => format!("Quad {}s, {} kicker", rank, kicker),
            Hand::StraightFlush{ref hi_rank} => format!("{}-high straight flush", hi_rank),
        };
        write!(f, "{}", string)
    }
}
