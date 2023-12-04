use crate::{Solution, SolutionPair};
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let mut sol1: u64 = 0;
    //Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 5
    let input = read_to_string("input/days/day04.txt").unwrap();
    let card_input = input.lines();
    let mut card_count: Vec<u32> = vec![0; 212];
    let mut currend_game_index = 0;
    for card_line in card_input {
        if let Some((_, game_input)) = card_line.split_once(":") {
            if let Some((winning_input, scratch_input)) = game_input.split_once("|") {
                // parse the input into a set
                let winning_card:HashSet<u32> = HashSet::from_iter(
                    winning_input.split_whitespace().map(|n| n.parse::<u32>().unwrap()),
                );
                let scratch_card:HashSet<u32> = HashSet::from_iter(
                    scratch_input.split_whitespace().map(|n| n.parse::<u32>().unwrap()),
                );

                // logic to check if the scratch card is a winner
                let mut winning_count = 0;
                for number in scratch_card.iter() {
                    if winning_card.contains(number) {
                        winning_count += 1;
                    }
                }

                card_count[currend_game_index] += 1;
                if winning_count > 0 {
                    sol1 += u64::pow(2, winning_count - 1);
                    // sol2 keeping track of the number of cards in the game
                    let current_num_cards = card_count[currend_game_index];
                    for copy_game_index in 1..(winning_count + 1) {
                        let lookahead_index = currend_game_index + copy_game_index as usize;
                        card_count[lookahead_index] =
                            current_num_cards + card_count[lookahead_index];
                    }
                }
            }
        }
        currend_game_index += 1;
    }
    let sol2: u32 = card_count.iter().sum();
    (Solution::from(sol1), Solution::from(sol2))
}