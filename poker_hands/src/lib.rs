#![feature(core)]
#![feature(box_syntax)]
extern crate cards;

use cards::{Rank, Suit, Card};
use std::fmt::{Debug, Formatter};
use std::cmp::{Eq, Ordering};

// Rank arrays are used for kickers. They should be sorted descending.

pub struct HiCardStr        {pub ranks: [Rank; 5]}
pub struct PairStr          {pub rank: Rank, pub kickers: [Rank; 5]}
pub struct TwoPairStr       {pub hi_rank: Rank, pub lo_rank: Rank, pub kicker: Rank}
pub struct TripsStr         {pub rank: Rank, pub kickers: [Rank; 2]}
pub struct StraightStr      {pub hi_rank: Rank}
pub struct FlushStr         {pub ranks: [Rank; 5]}
pub struct FullHouseStr     {pub three_of: Rank, pub two_of: Rank}
pub struct QuadsStr         {pub rank: Rank, pub kicker: Rank}
pub struct StraightFlushStr {pub hi_rank: Rank}

pub enum Hand {
    HiCard(HiCardStr),
    Pair(PairStr),
    TwoPair(TwoPairStr),
    Trips(TripsStr),
    Straight(StraightStr),
    Flush(FlushStr),
    FullHouse(FullHouseStr),
    Quads(QuadsStr),
    StraightFlush(StraightFlushStr)
}

use Hand::{HiCard, Pair, TwoPair, Trips, Straight, Flush, FullHouse, Quads, StraightFlush};

// Macro facilitating returning as soon as a function returns a match.
macro_rules! try_getting_hand(
    ($function:path, $hand_type:path, $cards:ident) => {
        match $function($cards) {
            Some(box hand) => return Box::new($hand_type(hand)),
            None => {},
        };
    };
);

impl Hand {
    // Pick out the best five-card hand.
    pub fn best_hand_of(cards: &[Card]) -> Hand {
        assert!(cards.len() >= 5);

        //TODO try all possible 5-card hands
        //TODO return the best one
        Straight(StraightStr{hi_rank: Rank::Ace})//TODO

        //XXX an alternate algorithm is to actually try to build up hands sequentially
    }

    // Makes five cards into a hand.
    pub fn get_hand(cards: &[Card]) -> Box<Hand> {
        assert!(cards.len() == 5);

        try_getting_hand!(hand_builder::get_straight_flush, StraightFlush, cards);
        try_getting_hand!(hand_builder::get_quads, Quads, cards);
        try_getting_hand!(hand_builder::get_full_house, FullHouse, cards);
        try_getting_hand!(hand_builder::get_flush, Flush, cards);
        try_getting_hand!(hand_builder::get_straight, Straight, cards);
        try_getting_hand!(hand_builder::get_trips, Trips, cards);
        try_getting_hand!(hand_builder::get_two_pair, TwoPair, cards);
        try_getting_hand!(hand_builder::get_pair, Pair, cards);
        box HiCard(*hand_builder::get_hi_card(cards))
    }
}

mod hand_builder {
    use cards::{Rank, Suit, Card};
    use {Hand, HiCardStr, PairStr, TwoPairStr, TripsStr, StraightStr, FlushStr, FullHouseStr, QuadsStr, StraightFlushStr};

    /*
    Functions which construct a Hand from five Cards, if they form that hand.

    Note all these methods assume the actual hand is at best the hand being
    asked for. So, for instance, build_trips() assumes the actual hand is
    trips at best, so if it finds trips, it doesn't need to check that the
    hand isn't actually quads or a boat. Quads or boat are better than
    trips, so it assumes beforehand that this hand is not those.
    */

    pub fn get_straight_flush(cards: &[Card]) -> Option<Box<StraightFlushStr>> {
        let flush_suit = get_flush_suit(cards);
        if !flush_suit.is_some() {
            return None;
        }
        // Found a flush; now look for a straight, too.

        let straight_candidate = get_straight(cards);
        match straight_candidate {
            Some(box StraightStr{hi_rank}) => Option::Some(box StraightFlushStr{hi_rank: hi_rank}),
            None => None
        }
    }

    pub fn get_flush(cards: &[Card]) -> Option<Box<FlushStr>> {
        let flush_suit = get_flush_suit(cards);
        if !flush_suit.is_some() {
            return None; // No suit, no flush!
        }

        let box HiCardStr{ranks} = get_hi_card(cards);
        Some(box FlushStr{ranks: ranks})
    }

    pub fn get_quads(cards: &[Card]) -> Option<Box<QuadsStr>> {
None//TODO
    }

    pub fn get_full_house(cards: &[Card]) -> Option<Box<FullHouseStr>> {
None//TODO
    }

    pub fn get_straight(cards: &[Card]) -> Option<Box<StraightStr>> {
None//TODO

    }

    pub fn get_trips(cards: &[Card]) -> Option<Box<TripsStr>> {
None//TODO

    }

    pub fn get_two_pair(cards: &[Card]) -> Option<Box<TwoPairStr>> {
None//TODO

    }

    pub fn get_pair(cards: &[Card]) -> Option<Box<PairStr>> {
None//TODO

    }

/* Not optional because this assumes nothing better than high card
   is the case, meaning a high card hand can definitely be formed.
 */
    pub fn get_hi_card(cards: &[Card]) -> Box<HiCardStr> {
        box HiCardStr{ranks: [Rank::Ace, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine]} //TODO

    }

    fn get_flush_suit(cards: &[Card]) -> Option<Suit> {
None//TODO

    }
}

fn hand_to_index(hand: & Hand) -> u8 {
    match *hand {
        HiCard(..) => 0,
        Pair(..) => 1,
        TwoPair(..) => 2,
        Trips(..) => 3,
        Straight(..) => 4,
        Flush(..) => 5,
        FullHouse(..) => 6,
        Quads(..) => 7,
        StraightFlush(..) => 8,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Pick the best hand type, if they're different.
        let this_index = hand_to_index(self);
        let other_index = hand_to_index(other);
        let initial_comparison = this_index.cmp(&other_index);
        if initial_comparison != Ordering::Equal {
            return initial_comparison;
        }

        // Same type of hand; tiebreak if possible.
        cmp_same_type_hand(self, other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        let cmp_maybe = self.partial_cmp(other);
        match cmp_maybe {
            Some(cmp) => cmp == Ordering::Equal,
            None => panic!("partial_cmp() failed to order hands {:?} and {:?}", self, other)
        }
    }
}

fn cmp_same_type_hand(this: & Hand, other: & Hand) -> Ordering {
    // capacity = max number of comparables = hand size
    let mut this_comparable_buffer: Vec<Rank> = Vec::with_capacity(5);
    let mut other_comparable_buffer: Vec<Rank> = Vec::with_capacity(5);

    match (this, other) {
        (&HiCard(HiCardStr{ranks: ref these_ranks}), &HiCard(HiCardStr{ranks: ref other_ranks})) => {
            copy_all(&mut this_comparable_buffer, these_ranks);
            copy_all(&mut other_comparable_buffer, other_ranks);
        },
        (&Pair(PairStr{rank: this_rank, kickers: ref these_kickers}), &Pair(PairStr{rank: other_rank, kickers: ref other_kickers})) => {
            assert_eq!(3, these_kickers.len());
            assert_eq!(3, other_kickers.len());

            this_comparable_buffer.push(this_rank);
            other_comparable_buffer.push(other_rank);

            copy_all(&mut this_comparable_buffer, these_kickers);
            copy_all(&mut other_comparable_buffer, other_kickers);
        },
        (&TwoPair(TwoPairStr{hi_rank: this_hi_rank, lo_rank: this_lo_rank, kicker: this_kicker}), &TwoPair(TwoPairStr{hi_rank: other_hi_rank, lo_rank: other_lo_rank, kicker: other_kicker})) => {
            this_comparable_buffer.push(this_hi_rank);
            other_comparable_buffer.push(other_hi_rank);

            this_comparable_buffer.push(this_lo_rank);
            other_comparable_buffer.push(other_lo_rank);

            this_comparable_buffer.push(this_kicker);
            other_comparable_buffer.push(other_kicker);
        },
        (&Trips(TripsStr{rank: this_rank, kickers: ref these_kickers}), &Trips(TripsStr{rank: other_rank, kickers: ref other_kickers})) => {
            assert_eq!(2, these_kickers.len());
            assert_eq!(2, other_kickers.len());

            this_comparable_buffer.push(this_rank);
            other_comparable_buffer.push(other_rank);

            copy_all(&mut this_comparable_buffer, these_kickers);
            copy_all(&mut other_comparable_buffer, other_kickers);
        },
        (&Straight(StraightStr{hi_rank: this_rank}), &Straight(StraightStr{hi_rank: other_rank})) => {
            this_comparable_buffer.push(this_rank);
            other_comparable_buffer.push(other_rank);
        },
        (&Flush(FlushStr{ranks: ref these_ranks}), &Flush(FlushStr{ranks: ref other_ranks})) => {
            copy_all(&mut this_comparable_buffer, these_ranks);
            copy_all(&mut other_comparable_buffer, other_ranks);
        },
        (&FullHouse(FullHouseStr{three_of: this_three_of, two_of: this_two_of}), &FullHouse(FullHouseStr{three_of: other_three_of, two_of: other_two_of})) => {
            this_comparable_buffer.push(this_three_of);
            other_comparable_buffer.push(other_three_of);

            this_comparable_buffer.push(this_two_of);
            other_comparable_buffer.push(other_two_of);
        },
        (&Quads(QuadsStr{rank: this_rank, kicker: this_kicker}), &Quads(QuadsStr{rank: other_rank, kicker: other_kicker})) => {
            this_comparable_buffer.push(this_rank);
            other_comparable_buffer.push(other_rank);

            this_comparable_buffer.push(this_kicker);
            other_comparable_buffer.push(other_kicker);
        },
        (&StraightFlush(StraightFlushStr{hi_rank: this_rank}), &StraightFlush(StraightFlushStr{hi_rank: other_rank})) => {
            this_comparable_buffer.push(this_rank);
            other_comparable_buffer.push(other_rank);
        },

        // Logic error case where the hands are different types
        (_, _) => { panic!("Different hand types passed to cmp_same_type_hand()!\
                         {:?} {:?}", this, other) },
    };
    assert_eq!(this_comparable_buffer.len(), other_comparable_buffer.len());
    this_comparable_buffer.cmp(&other_comparable_buffer)
}

fn copy_all<T: Copy> (dest: & mut Vec<T>, src: &[T]) {
    for element in src.iter() {
        dest.push(*element);
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let string = match *self {
            HiCard(HiCardStr{ref ranks}) => format!("{:?}", ranks),
            Pair(PairStr{rank, ref kickers}) => format!("Pair of {:?}s, {:?} kickers", rank, kickers),
            TwoPair(TwoPairStr{hi_rank, lo_rank, kicker}) => format!("Two pair, {:?} and {:?}, {:?} kicker", hi_rank, lo_rank, kicker),
            Trips(TripsStr{rank, ref kickers}) => format!("Trip {:?}s, {:?} kickers", rank, kickers),
            Straight(StraightStr{hi_rank}) => format!("{:?}-high straight", hi_rank),
            Flush(FlushStr{ref ranks}) => format!("Flush of {:?}", ranks),
            FullHouse(FullHouseStr{three_of, two_of}) => format!("Full house, three {:?}s, two {:?}s", three_of, two_of),
            Quads(QuadsStr{rank, kicker}) => format!("Quad {:?}s, {:?} kicker", rank, kicker),
            StraightFlush(StraightFlushStr{hi_rank}) => format!("{:?}-high straight flush", hi_rank),
        };
        write!(f, "{}", string)
    }
}
