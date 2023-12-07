mod poker;

use poker::Hand;
use std::fs;

fn get_total_winnings(s: &str, with_jokers: bool) -> u32 {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|s| Hand::parse(s, with_jokers).unwrap())
        .collect();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", get_total_winnings(&input, false));
    println!("Part 2: {}", get_total_winnings(&input, true));
}
