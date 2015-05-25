extern crate cards;
extern crate poker_hands;

use std::collections::HashMap;

use cards::{Card, Rank, Suit};
use poker_hands::Hand;

fn main() {
    println!("Hello, world!");
    //TODO read cli args

    let num_sims = 10 * 1000;
    //TODO get some hole cards
    let all_hole_cards: Vec<[Card; 2]> = Vec::new();

    let mut outcomes = HashMap::new();
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
        let mut best_hand = hands[0];
        for index in 1..hands.len() {
            let hand = hands[index];
            if hand == best_hand {
                winners.push(index as i32);
            } else if hand > best_hand {
                winners.clear();
                winners.push(index as i32);
                best_hand = hand;
            }
        }
        insert_outcome(&mut outcomes, &winners, &best_hand);
    }
    //TODO print stats of what happened
}

fn insert_outcome(outcomes: &mut HashMap<Vec<i32>, HandStats>, winners: &Vec<i32>, hand: &Hand) {
    // Set up default stats if there are none yet.
    if let None = outcomes.get(winners) {
        outcomes.insert(winners.clone(), HandStats::create());
    }

    outcomes.get_mut(winners).unwrap().add_event(hand);
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

    fn add_event(&mut self, hand: &Hand) {
        let event_index: u8 = (*hand).into();
        self.events[event_index as usize] += 1;
    }

    fn total_events(self) -> i32 {
        self.events.iter().fold(0, |aggregate, event| aggregate + event)
    }
}
