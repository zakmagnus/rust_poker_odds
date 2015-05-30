extern crate rand;
extern crate getopts;
extern crate cards;
extern crate poker_hands;

use std::env;
use std::collections::HashMap;
use getopts::{Options, Matches, HasArg, Occur};
use rand::{thread_rng, Rng};

use cards::{Card, Rank, Suit, card};
use cards::Rank::*;
use cards::Suit::*;
use poker_hands::{Hand, NUM_HANDS};

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = create_opts();
    let arg_matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(error) => panic!("Could not parse {:?}; error: {:?}", args, error)
    };

    let num_sims = 10 * 1000; // TODO optionally get from args
    let all_hole_cards = get_hole_cards(&arg_matches);

    let mut outcomes = HashMap::new();
    // TODO try doing this in parallel!
    for _ in 0..num_sims {
        // TODO optionally get a partial board from args
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

    for (outcome, stats) in outcomes {
        let total_events = stats.total_events();
        let outcome_percent = (total_events as f64 / num_sims as f64) * 100f64;
        let outcome_name = name_outcome(&outcome);
        println!("{} ({} times, {}%)", outcome_name, total_events, outcome_percent);
        //TODO sort the hands by %
        for hand_index in 0..NUM_HANDS {
            let hand_events = stats.events[hand_index];
            if hand_events == 0 {
                continue;
            }
            let hand_percent = (hand_events as f64 / total_events as f64) * 100f64;
            println!("\t{}: {} times, {}%", Hand::name_hand_index(hand_index), hand_events, hand_percent);
        }
    }
    //TODO sort the outcomes by %
}

const HOLE_CARDS_ARG: &'static str = "h";
fn create_opts() -> Options {
    // Unfortunately, there doesn't seem to be a way to require that an option appears at least once.
    let mut opts = Options::new();
    opts.opt(HOLE_CARDS_ARG, "hole cards", "A single player's hole cards", "XxYy", HasArg::Yes, Occur::Multi);
    opts
}

fn get_hole_cards(matches: &Matches) -> Vec<[Card; 2]> {
    assert!(matches.opt_count(HOLE_CARDS_ARG) >= 1, "No hole cards specified");
    let hole_strings = matches.opt_strs(HOLE_CARDS_ARG);
    let mut all_hole_cards = Vec::with_capacity(hole_strings.len());
    for hole_string in &hole_strings {
        let hole_cards = parse_cards_string(hole_string);
        assert!(hole_cards.len() == 2, "{} specifies {} cards, not 2", hole_string, hole_cards.len());
        all_hole_cards.push([hole_cards[0], hole_cards[1]]);
    }
    all_hole_cards
}

fn parse_cards_string(cards_string: &str) -> Vec<Card> {
    let chars: Vec<char> = cards_string.chars().collect();
    assert!(chars.len() % 2 == 0, "Odd numbers of characters, cannot be cards: {}", cards_string);

    let num_cards = chars.len() / 2;
    let mut cards = Vec::with_capacity(num_cards);
    for card_index in 0..num_cards {
        let rank_index = card_index * 2;
        let suit_index = rank_index + 1;
        let rank_char = chars[rank_index];
        let suit_char = chars[suit_index];
        let rank = parse_rank(rank_char).expect(
                &format!("Couldn't parse {} (position {} in {}) as a rank",
                rank_char, rank_index, cards_string));
        let suit = parse_suit(suit_char).expect(
                &format!("Couldn't parse {} (position {} in {}) as a suit",
                suit_char, suit_index, cards_string));
        cards.push(card(rank, suit));
    }
    cards
}

fn parse_rank(rank_char: char) -> Option<Rank> {
    let rank = match rank_char {
        'A' | 'a' => Ace,
        'K' | 'k' => King,
        'Q' | 'q' => Queen,
        'J' | 'j' => Jack,
        'T' | 't' => Ten,
        '9' => Nine,
        '8' => Eight,
        '7' => Seven,
        '6' => Six,
        '5' => Five,
        '4' => Four,
        '3' => Three,
        '2' => Two,
        _ => return None
    };
    Some(rank)
}

fn parse_suit(suit_char: char) -> Option<Suit> {
    let suit = match suit_char {
        'S' | 's' => Spades,
        'H' | 'h' => Hearts,
        'C' | 'c' => Clubs,
        'D' | 'd' => Diamonds,
        _ => return None
    };
    Some(suit)
}

fn insert_outcome(outcomes: &mut HashMap<Vec<i32>, HandStats>, winners: &Vec<i32>, hand: &Hand) {
    // Set up default stats if there are none yet.
    if let None = outcomes.get(winners) {
        outcomes.insert(winners.clone(), HandStats::create());
    }

    outcomes.get_mut(winners).unwrap().add_event(hand);
}

const BOARD_SIZE: usize = 5;
fn pick_random_board(all_hole_cards: &[[Card; 2]]) -> [Card; BOARD_SIZE] {
    let mut board = [Card{rank: Rank::Ace, suit: Suit::Spades}; BOARD_SIZE]; // Dummies

    let mut used_indexes: Vec<u8> = Vec::with_capacity(all_hole_cards.len() + BOARD_SIZE);
    used_indexes.extend(
        all_hole_cards.iter().
        flat_map(|cards| cards). // Flatten all hands into one iterator
        map(|card| (*card).into() ));

    let mut board_index = 0;
    let mut rng = rand::thread_rng();
    while board_index < BOARD_SIZE {
        /*
        Generate random cards and skip them if they're used already.
        The assumption is that few cards will be used compared to the
        possible 52, so it should skip rarely and be efficient.
        */
        let card = rng.gen::<Card>();
        let card_index = card.into();
        if used_indexes.contains(&card_index) {
            continue;
        }
        used_indexes.push(card_index);
        board[board_index] = card;
        board_index += 1;
    }
    board
}

struct HandStats {
    events: [i32; NUM_HANDS], // Number of times each hand happened
}

impl HandStats {
    fn create() -> HandStats {
        HandStats{events: [0; NUM_HANDS]}
    }

    fn add_event(&mut self, hand: &Hand) {
        let event_index: u8 = (*hand).into();
        self.events[event_index as usize] += 1;
    }

    fn total_events(&self) -> i32 {
        self.events.iter().fold(0, |aggregate, event| aggregate + event)
    }
}

fn name_outcome(outcome: &Vec<i32>) -> String {
    if outcome.len() == 1 {
        return format!("Hand {} wins", outcome[0]);
    }
    if outcome.len() > 0 {
        return format!("Chop between hands {}", vec_to_string(outcome));
    }
    panic!("Empty outcome")
}

fn vec_to_string(vec: &Vec<i32>) -> String {
    let mut string = format!("{}", vec[0]);
    for index in 1..vec.len() {
        string = string + &format!(", {}", vec[index]);
    }
    string
}
