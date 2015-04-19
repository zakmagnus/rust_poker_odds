#![feature(box_syntax)]
#![feature(box_patterns)]
extern crate cards;

mod hand_order_tests;
mod hand_making_tests;

use cards::{Rank, Card};
use std::fmt::{Debug, Formatter};
use std::cmp::{Eq, Ordering};

// Rank arrays are used for kickers. They should be sorted descending.

pub struct HiCardStr        {pub ranks: [Rank; 5]}
pub struct PairStr          {pub rank: Rank, pub kickers: [Rank; 3]}
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
        for i in 0..4 {
            assert!(cards[i].rank >= cards[i + 1].rank);
        }

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
    use {HiCardStr, PairStr, TwoPairStr, TripsStr, StraightStr, FlushStr, FullHouseStr, QuadsStr, StraightFlushStr};

    /*
    Functions which construct a Hand from five Cards, if they form that hand.

    Note all these functions assume the actual hand is at best the hand being
    asked for. So, for instance, build_trips() assumes the actual hand is
    trips at best, so if it finds trips, it doesn't need to check that the
    hand isn't actually quads or a boat. Quads or boat are better than
    trips, so it assumes beforehand that this hand is not those.

    These functions also assume that the cards they're being passed are
    already sorted by rank.
    */

    pub fn get_straight_flush(cards: &[Card]) -> Option<Box<StraightFlushStr>> {
        let flush_suit = get_flush_suit(cards);
        if !flush_suit.is_some() {
            return None;
        }
        // Found a flush; now look for a straight, too.

        let straight_candidate = get_straight(cards);
        match straight_candidate {
            Some(box StraightStr{hi_rank}) => Some(box StraightFlushStr{hi_rank: hi_rank}),
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
        // In sorted quads, either the first or last two cards must have the same rank.
        let high_kicker = cards[0].rank != cards[1].rank;
        let low_kicker = cards[3].rank != cards[4].rank;
        if (!low_kicker) && (!high_kicker) {
            return None
        }

        // Now which end the kicker is on is known.
        // Assume the rest are quads and check that assumption.
        let (kicker, quad_rank, quad_start_index) =
        if low_kicker {
            (cards[4].rank, cards[0].rank, 0)
        } else {
            (cards[0].rank, cards[1].rank, 1)
        };
        for i in quad_start_index..(quad_start_index + 4) {
            if cards[i].rank != quad_rank {
                return None
            }
        }
        Some(box QuadsStr{rank: quad_rank, kicker: kicker})
    }

    pub fn get_full_house(cards: &[Card]) -> Option<Box<FullHouseStr>> {
        /* In a sorted boat, the first two cards have the same rank, and
         * the last two cards have the same rank. The middle card shares
         * a rank with one of the ends.
         */
        if cards[0].rank != cards[1].rank {
            return None
        }
        if cards[4].rank != cards[1].rank {
            return None
        }

        let high_pair_rank = cards[0].rank;
        let low_pair_rank = cards[4].rank;
        // Sanity check - sorted cards, no five-of-a-kind!
        assert!(high_pair_rank != low_pair_rank);
        let middle_rank = cards[2].rank;
        let three_of_high = high_pair_rank == middle_rank;
        let three_of_low = low_pair_rank == middle_rank;

        if (!three_of_high) && (!three_of_low) {
            // No three-of-a-kind at all.
            return None
        }

        let (three_of, two_of) =
        if three_of_high {
            (high_pair_rank, low_pair_rank)
        } else {
            (low_pair_rank, high_pair_rank)
        };
        Some(box FullHouseStr{three_of: three_of, two_of: two_of})
    }

    pub fn get_straight(cards: &[Card]) -> Option<Box<StraightStr>> {
        for i in 1..5 {
            let this_card = cards[i];
            let prev_card = cards[i - 1];
            // Wheel detection
            if i == 1 && prev_card.rank == Rank::Ace && this_card.rank == Rank::Two {
                continue;
            }
            let this_rank_index = cards[i].rank as i32;
            let prev_rank_index = cards[i - 1].rank as i32;
            if this_rank_index != prev_rank_index + 1 {
                return None
            }
        }
        // Straight detected! Now make sure to get the wheel right.
        if cards[0].rank == Rank::Ace {
            Some(box StraightStr{hi_rank: Rank::Four})
        } else {
            Some(box StraightStr{hi_rank: cards[0].rank})
        }
    }

    pub fn get_trips(cards: &[Card]) -> Option<Box<TripsStr>> {
        let (trip_rank, high_kicker, low_kicker) =
        if cards[0].rank == cards[2].rank {
            (cards[0].rank, cards[3].rank, cards[4].rank)
        } else if cards[1].rank == cards[3].rank {
            (cards[1].rank, cards[0].rank, cards[4].rank)
        } else if cards[2].rank == cards[4].rank {
            (cards[2].rank, cards[0].rank, cards[1].rank)
        } else {
            return None
        };

        Some(box TripsStr{rank: trip_rank, kickers: [high_kicker, low_kicker]})
    }

    pub fn get_two_pair(cards: &[Card]) -> Option<Box<TwoPairStr>> {
        let mut high_pair_rank = None;
        let mut low_pair_rank = None;
        let mut i = 1;
        while i < 5 {
            let this_rank = cards[i].rank;
            let prev_rank = cards[i - 1].rank;
            if this_rank == prev_rank {
                if !high_pair_rank.is_some() {
                    high_pair_rank = Some(this_rank);
                } else if !low_pair_rank.is_some() {
                    low_pair_rank = Some(this_rank);
                } else {
                    panic!("Two pairs have already been found, yet have found a third one. High pair: {:?} Low pair: {:?} Third pair: {:?} Hand: {:?}",
                        high_pair_rank, low_pair_rank, this_rank, cards);
                }
                i += 2; // Found a pair, so don't compare the next rank to this one.
            } else {
                i += 1;
            }
        }

        if !(high_pair_rank.is_some() && low_pair_rank.is_some()) {
            return None
        }

        let hi_rank = high_pair_rank.unwrap();
        let lo_rank = low_pair_rank.unwrap();

        let mut kicker = cards[0].rank;
        for i in 1..5 {
            let rank = cards[i].rank;
            if rank != hi_rank && rank != lo_rank {
                kicker = rank;
                break;
            }
        }

        Some(box TwoPairStr{hi_rank: hi_rank, lo_rank: lo_rank, kicker: kicker})
    }

    pub fn get_pair(cards: &[Card]) -> Option<Box<PairStr>> {
        let mut pair_start = None;
        for i in 0..4 {
            let this_rank = cards[i].rank;
            let next_rank = cards[i + 1].rank;
            if this_rank == next_rank {
                pair_start = Some(i);
                break;
            }
        }
        if !pair_start.is_some() {
            return None
        }

        let pair_rank = cards[pair_start.unwrap()].rank;
        let mut kickers = [Rank::Ace, Rank::Ace, Rank::Ace]; // Dummy values.
        let mut kicker_index = 0;
        for i in 0..5 {
            let this_rank = cards[i].rank;
            if this_rank == pair_rank {
                continue;
            }
            kickers[kicker_index] = this_rank;
            kicker_index += 1;
        }
        assert!(kicker_index == 3);

        return Some(box PairStr{rank: pair_rank, kickers: kickers})
    }

/* Not optional because this assumes nothing better than high card
   is the case, meaning a high card hand can definitely be formed.
 */
    pub fn get_hi_card(cards: &[Card]) -> Box<HiCardStr> {
        // Note the cards are assumed to be sorted by rank already.
        box HiCardStr{ranks: [cards[0].rank,
                              cards[1].rank,
                              cards[2].rank,
                              cards[3].rank,
                              cards[4].rank]}
    }

    fn get_flush_suit(cards: &[Card]) -> Option<Suit> {
        let flush_suit = cards[0].suit;
        for card in cards.iter() {
            let suit = card.suit;
            if suit != flush_suit {
                return None;
            }
        }
        Some(flush_suit)
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
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
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
        (_, _) => { panic!("Different hand types passed to cmp_same_type_hand()! \
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
