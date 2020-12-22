use std::collections::{VecDeque, HashSet};

macro_rules! trace {
    // ($( $args:expr ),*) => { println!( $( $args ),* ); }
    ($( $args:expr ),*) => { }
}

const PRIMES: [usize; 50] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229];

fn get_deck_hash(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .enumerate()
        .fold(1, |acc, (index, card)| {
            let prime: usize = PRIMES[index];
            acc * prime.pow(*card as u32)
        })
}

#[derive(Debug)]
enum Player {
    One,
    Two,
}

fn sub_deck(deck: &VecDeque<usize>, length: usize) -> VecDeque<usize> {
    let mut result = VecDeque::with_capacity(length);
    for i in 0..length {
        result.push_back(deck.get(i).unwrap().clone());
    }
    result
}

fn play_sub_game(mut deck_1: VecDeque<usize>, mut deck_2: VecDeque<usize>, mut game_counter: usize) -> (Player, Option<VecDeque<usize>>) {
    let this_game = game_counter.clone();
    trace!("\n=== Game {} ===", this_game);

    let mut prev_states_1 = HashSet::new();
    let mut prev_states_2 = HashSet::new();

    let mut round_num = 0;
    loop {
        round_num += 1;
        trace!("\n--- Round {} (Game {}) ---", round_num, this_game);
        trace!("Player 1's deck {:?}", deck_1);
        trace!("Player 2's deck {:?}", deck_2);

        let hash_1 = get_deck_hash(&deck_1);
        let hash_2 = get_deck_hash(&deck_2);
        if prev_states_1.contains(&hash_1) && prev_states_2.contains(&hash_2) {
            // Infinite loop in this game, so player 1 wins
            trace!("Infinite loop detected - player 1 wins!");
            return (Player::One, None);
        }
        prev_states_1.insert(hash_1);
        prev_states_2.insert(hash_2);

        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        trace!("Player one plays: {}", card_1);
        trace!("Player two plays: {}", card_2);

        if card_1 > deck_1.len() || card_2 > deck_2.len() {
            // Can't recurse any more, so wins are based on highest card
            if card_1 > card_2 {
                trace!("Player one wins round {} of game {}", round_num, this_game);
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            } else {
                trace!("Player two wins round {} of game {}", round_num, this_game);
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }

            // If we've got an empty deck, someone's won this game
            if deck_1.is_empty() {
                trace!("The winner of game {} is player two", this_game);
                return (Player::Two, Some(deck_2));
            }
            if deck_2.is_empty() {
                trace!("The winner of game {} is player one", this_game);
                return (Player::One, Some(deck_1));
            }

            // Otherwise, continue to play the next round`
            continue;
        }

        // Need to recurse into new sub-game to decide victor
        trace!("Playing a sub-game to determine the winner... ");
        let sub_deck_1 = sub_deck(&deck_1, card_1);
        let sub_deck_2 = sub_deck(&deck_2, card_2);
        game_counter += 1;
        let (winner, _) = play_sub_game(sub_deck_1, sub_deck_2, game_counter);
        trace!("\n...anyway, back to game {}", this_game);
        match winner {
            Player::One => {
                trace!("Player one wins round {} of game {}", round_num, this_game);
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            }
            Player::Two => {
                trace!("Player two wins round {} of game {}", round_num, this_game);
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        }
        // Now continue to the next round
    }
}

fn main() {
    let mut deck_1 = VecDeque::from(vec![7,1,9,10,12,4,38,22,18,3,27,31,43,33,47,42,21,24,50,39,8,6,16,46,11]);
    let mut deck_2 = VecDeque::from(vec![49,41,40,35,44,29,30,19,14,2,34,17,25,5,15,32,20,48,45,26,37,28,36,23,13]);

    while deck_1.len() > 0 && deck_2.len() > 0 {
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();
        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }
    }

    if deck_1.is_empty() {
        let score: usize = deck_2.iter().enumerate().map(|(index, card)| (deck_2.len() - index) * *card).sum();
        println!("Part 1: {}", score);
    } else if deck_2.is_empty() {
        let score: usize = deck_1.iter().enumerate().map(|(index, card)| (deck_1.len() - index) * *card).sum();
        println!("Part 1: {}", score);
    } else {
        println!("### Part 1: Error - neither deck empty! ###");
    }

    let deck_1 = VecDeque::from(vec![7,1,9,10,12,4,38,22,18,3,27,31,43,33,47,42,21,24,50,39,8,6,16,46,11]);
    let deck_2 = VecDeque::from(vec![49,41,40,35,44,29,30,19,14,2,34,17,25,5,15,32,20,48,45,26,37,28,36,23,13]);
    // let deck_1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
    // let deck_2 = VecDeque::from(vec![5, 8, 4, 7, 10]);
    let (_, winning_deck) = play_sub_game(deck_1, deck_2, 1);
    if let Some(deck) = winning_deck {
        let score: usize = deck.iter().enumerate().map(|(index, card)| (deck.len() - index) * *card).sum();
        println!("Part 2: {:?}", score);
    }
}
