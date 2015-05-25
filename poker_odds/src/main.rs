extern crate cards;
extern crate poker_hands;

use cards::{Card, Rank, Suit};
use poker_hands::Hand;

fn main() {
    println!("Hello, world!");
    //TODO read cli args

    let num_sims = 10 * 1000;
    //TODO get some hole cards
    let all_hole_cards: Vec<[Card; 2]> = Vec::new();
    //TODO map (winner array) -> HandStats
    for _ in 0..num_sims {
        let board = pick_random_board(&all_hole_cards);
        let mut hands = Vec::with_capacity(all_hole_cards.len());
        for hole_cards in &all_hole_cards {
            let mut cards: Vec<Card> = Vec::with_capacity(hole_cards.len() + board.len());
            cards.extend(board.iter().cloned());
            cards.extend(hole_cards.iter().cloned());
            // Sort descending - best_hand_of() requires this.
            cards.sort_by(|first, second| second.cmp(first));
            let hand = Hand::best_hand_of(&cards);
            hands.push(hand);
        }

        let mut winners = Vec::new();
        winners.push(0);
        let mut best_hand = hands.get(0);
        for index in 1..hands.len() {
            let hand = hands.get(index);
            if hand == best_hand {
                winners.push(index);
            } else if hand > best_hand {
                winners.clear();
                winners.push(index);
                best_hand = hand;
            }
        }
        //TODO key into winning stats and add this hand to its events
    }
    //TODO print stats of what happened
}

fn pick_random_board(all_hole_cards: &[[Card; 2]]) -> [Card; 5] {
    //TODO
    [Card{rank: Rank::Ace, suit: Suit::Spades}; 5]
}

struct HandStats {
    events: [i32; 9], // Number of times each hand happened
}

impl HandStats {
    fn create() -> HandStats {
        HandStats{events: [0; 9]}
    }

    fn add_event(&mut self, hand: Hand) {
        let event_index: u8 = hand.into();
        self.events[event_index as usize] += 1;
    }

    fn total_events(self) -> i32 {
        self.events.iter().fold(0, |aggregate, event| aggregate + event)
    }
}
