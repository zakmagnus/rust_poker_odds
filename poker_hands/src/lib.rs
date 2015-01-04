extern crate cards;

use cards::Rank;
use std::fmt::{Show, Formatter};

// Rank arrays are used for kickers. They should be sorted descending.

enum Hand {
    HiCard        {ranks: [Rank, ..5]},
    Pair          {rank: Rank, kickers: [Rank, ..5]},
    TwoPair       {hiRank: Rank, loRank: Rank, kicker: Rank},
    Trips         {rank: Rank, kickers: [Rank, ..2]},
    Straight      {hiRank: Rank},
    Flush         {ranks: [Rank, ..5]},
    FullHouse     {threeOf: Rank, twoOf: Rank},
    Quads         {rank: Rank, kicker: Rank},
    StraightFlush {hiRank: Rank},
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
        //TODO
        false
    }
}

fn cmp_same_type_hand(this: & Hand, other: & Hand) -> Ordering {
    match (this, other) {
        (&Hand::HiCard{ranks: ref these_ranks}, &Hand::HiCard{ranks: ref other_ranks}) =>
            cmp_ordered_ranks(these_ranks.as_slice(), other_ranks.as_slice()),
        (&Hand::Pair{rank: ref this_rank, kickers: ref these_kickers}, &Hand::Pair{rank: ref other_rank, kickers: ref other_kickers}) => {
            let rank_cmp = this_rank.cmp(other_rank);
            if rank_cmp != Ordering::Equal {
                return rank_cmp;
            }
            cmp_ordered_ranks(these_kickers.as_slice(), other_kickers.as_slice())
        },

//TODO all the other hands lol

        // Logic error case where the hands are different types
        (_, _) => panic!("Different hand types passed to cmp_same_type_hand()!\
                         {} {}", this, other)
    }
}

fn cmp_ordered_ranks(these_ranks: &[Rank], other_ranks: &[Rank]) -> Ordering {
    assert_eq!(these_ranks.len(), other_ranks.len());
    // This works because the ranks are sorted descending, and comparable
    these_ranks.cmp(other_ranks)
}

impl Show for Hand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = match *self {
            Hand::HiCard{ref ranks} => format!("{}", ranks),
            Hand::Pair{ref rank, ref kickers} => format!("Pair of {}s, {} kickers", rank, kickers),
            Hand::TwoPair{ref hiRank, ref loRank, ref kicker} => format!("Two pair, {} and {}, {} kicker", hiRank, loRank, kicker),
            Hand::Trips{ref rank, ref kickers} => format!("Trip {}s, {} kickers", rank, kickers),
            Hand::Straight{ref hiRank} => format!("{}-high straight", hiRank),
            Hand::Flush{ref ranks} => format!("Flush of {}", ranks),
            Hand::FullHouse{ref threeOf, ref twoOf} => format!("Full house, three {}s, two {}s", threeOf, twoOf),
            Hand::Quads{ref rank, ref kicker} => format!("Quad {}s, {} kicker", rank, kicker),
            Hand::StraightFlush{ref hiRank} => format!("{}-high straight flush", hiRank),
        };
        write!(f, "{}", string)
    }
}
