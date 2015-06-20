extern crate rand;
extern crate getopts;
extern crate num_cpus;
extern crate cards;
extern crate poker_hands;

use std::env;
use std::collections::HashMap;
use std::str::FromStr;
use std::thread;
use std::sync::*;
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

    let initial_board = get_initial_board(&arg_matches);
    let total_num_sims =
    if initial_board.len() == BOARD_SIZE {
        println!("The given board is full, so there's no uncertainty.");
        1
    } else {
        get_num_sims(&arg_matches)
    };
    let all_hole_cards = get_hole_cards(&arg_matches);
    let num_threads = get_num_threads(&arg_matches);

    println!("Simulating {} hands", total_num_sims);
    if initial_board.len() > 0 {
        println!("For board {:?}", initial_board);
    }
    println!("Using {} threads", num_threads);
    let board_ref = Arc::new(initial_board);
    let hole_cards_ref = Arc::new(all_hole_cards);

    let outcomes = Arc::new(Mutex::new(HashMap::new()));
    let mut children = Vec::with_capacity(num_threads as usize);
    for thread_index in 0..num_threads {
        let this_num_sims = get_num_sims_for_thread(total_num_sims, num_threads as i32, thread_index as i32);
        let this_board_ref = board_ref.clone();
        let this_hole_cards_ref = hole_cards_ref.clone();
        let this_outcomes = outcomes.clone();
        let child_thread = thread::spawn(move || {
                simulate_hands(this_num_sims, &this_board_ref, &this_hole_cards_ref, &this_outcomes)
        });
        children.push(child_thread);
    }
    for child_thread in children {
        match child_thread.join() {
            Ok(_) => continue,
            Err(e) => panic!("Worker thread died! {:?}", e)
        }
    }

    let final_outcomes = outcomes.lock().unwrap();

    let sorted_outcomes = sort_descending(
        final_outcomes.iter().map(|(outcome, stats)| (outcome.clone(), stats.total_events())).collect());

    for outcome in sorted_outcomes {
        let stats = final_outcomes.get(&outcome).unwrap();
        let total_events = stats.total_events();
        let outcome_percent = (total_events as f64 / total_num_sims as f64) * 100f64;
        let outcome_name = name_outcome(&outcome, &hole_cards_ref);
        println!("{} ({} times, {}%)", outcome_name, total_events, outcome_percent);
        let sorted_hand_indices = sort_descending(
            (0..NUM_HANDS).map(|index| (index, stats.events[index])).collect());
        for hand_index in sorted_hand_indices {
            let hand_events = stats.events[hand_index];
            if hand_events == 0 {
                continue;
            }
            let hand_percent = (hand_events as f64 / total_events as f64) * 100f64;
            println!("\t{}: {} times, {}%", Hand::name_hand_index(hand_index), hand_events, hand_percent);
        }
    }
}

fn simulate_hands(num_sims: i32, initial_board: &[Card], all_hole_cards: &[[Card; 2]], outcomes: &Mutex<HashMap<Vec<i32>, HandStats>>) {
    for _ in 0..num_sims {
        let board = pick_random_board(initial_board, all_hole_cards);
        assert!(board.len() == BOARD_SIZE);
        let mut hands = Vec::with_capacity(all_hole_cards.len());
        for hole_cards in all_hole_cards {
            let mut cards: Vec<Card> = Vec::with_capacity(hole_cards.len() + board.len());
            cards.extend(board.iter().cloned());
            cards.extend(hole_cards.iter().cloned());
            // Sort descending - best_hand_of() requires this.
            cards.sort_by(|first, second| second.cmp(first));
            let hand = Hand::best_hand_of(&cards);
            hands.push(hand);
        }
        assert!(hands.len() == all_hole_cards.len());

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
        insert_outcome(&mut outcomes.lock().unwrap(), &winners, &best_hand);
    }
}

fn sort_descending<T: Clone>(mut items: Vec<(T, i32)>) -> Vec<T> {
    // Switch the order to get greatest-first.
    items.sort_by(|&(_, first), &(_, second)| second.cmp(&first));
    items.iter().map(|&(ref item, _)| item.clone()).collect()
}

const HOLE_CARDS_ARG: &'static str = "h";
const NUM_SIMS_ARG: &'static str = "n";
const NUM_THREADS_ARG: &'static str = "t";
const BOARD_ARG: &'static str = "b";
fn create_opts() -> Options {
    // Unfortunately, there doesn't seem to be a way to require that an option appears at least once.
    let mut opts = Options::new();
    opts.opt(HOLE_CARDS_ARG, "hole cards", "A single player's hole cards", "XxYy", HasArg::Yes, Occur::Multi);
    opts.opt(NUM_SIMS_ARG, "number of simulations", "The number of hands to simulate in order to approximate the true distribution.", "n", HasArg::Yes, Occur::Optional);
    opts.opt(NUM_THREADS_ARG, "number of threads to use", "The number of threads to use simultaneously to run the simulations.", "t", HasArg::Yes, Occur::Optional);
    opts.opt(BOARD_ARG, "board cards", "The cards already on the board.", "XxYyZz", HasArg::Yes, Occur::Optional);
    opts
}

fn get_initial_board(matches: &Matches) -> Vec<Card> {
    if !matches.opt_present(BOARD_ARG) {
        return Vec::new();
    }
    let board_string = matches.opt_str(&BOARD_ARG).unwrap();
    let initial_board = parse_cards_string(&board_string);
    assert!(initial_board.len() <= BOARD_SIZE, "Initial board has more than {} cards! {}", BOARD_SIZE, board_string);
    initial_board
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

const DEFAULT_NUM_SIMS: i32 = 10 * 1000;
fn get_num_sims(matches: &Matches) -> i32 {
    get_numeric_arg(matches, NUM_SIMS_ARG, DEFAULT_NUM_SIMS)
}

fn get_num_threads(matches: &Matches) -> i32 {
    get_numeric_arg(matches, NUM_THREADS_ARG, num_cpus::get() as i32)
}

fn get_numeric_arg(matches: &Matches, arg: &str, default: i32) -> i32 {
    if !matches.opt_present(arg) {
        return default;
    }
    let num_str = matches.opt_str(arg).unwrap();
    let num_maybe: Result<i32, _> = FromStr::from_str(&num_str);
    match num_maybe {
        Ok(num) => num,
        Err(_) => {
            println!("Could not parse {} arg as a number: {}; ignoring it.", arg, num_str);
            default
        }
    }
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
fn pick_random_board(initial_board: &[Card], all_hole_cards: &[[Card; 2]]) -> [Card; BOARD_SIZE] {
    let mut board = [card(Ace, Spades); BOARD_SIZE]; // Dummies
    for index in 0..initial_board.len() {
        board[index] = initial_board[index];
    }

    let mut used_indexes: Vec<u8> = Vec::with_capacity(all_hole_cards.len() + BOARD_SIZE);
    let card_to_index = |card: &Card| (*card).into();
    used_indexes.extend(
        initial_board.iter().map(&card_to_index));
    used_indexes.extend(
        all_hole_cards.iter().
        flat_map(|cards| cards). // Flatten all hands into one iterator
        map(&card_to_index));

    let mut board_index = initial_board.len();
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

fn get_num_sims_for_thread(total_num_sims: i32, total_num_threads: i32, thread_index: i32) -> i32 {
    assert!(total_num_threads > thread_index);
    let base_num_sims = total_num_sims / total_num_threads;
    let threads_with_extra = total_num_sims % total_num_threads;
    let this_threads_extra =
        if thread_index < threads_with_extra {
            1
        } else {
            0
        };
    base_num_sims + this_threads_extra
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

fn name_outcome(outcome: &Vec<i32>, all_hole_cards: &[[Card; 2]]) -> String {
    if outcome.len() == 1 {
        let hand_index = outcome[0];
        return format!("Hand {} {:?} wins", outcome[0], all_hole_cards[hand_index as usize]);
    }
    if outcome.len() > 0 {
        return format!("Chop between hands {}", hands_to_string(all_hole_cards, &outcome));
    }
    panic!("Empty outcome")
}

fn hands_to_string(hands: &[[Card; 2]], indices: &[i32]) -> String {
    let mut string = format!("{:?}", hands[indices[0] as usize]);
    for index in 1..indices.len() {
        string = string + &format!(", {:?}", hands[indices[index as usize] as usize]);
    }
    string
}
