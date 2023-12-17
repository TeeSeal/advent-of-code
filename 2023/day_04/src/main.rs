use std::collections::hash_set::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let match_counts: Vec<usize> = input.lines().map(count_matching_numbers).collect();
    let mut copy_counts: Vec<usize> = vec![1; match_counts.len()];

    for (i, match_count) in match_counts.iter().enumerate() {
        let &copies = copy_counts.get(i).unwrap();

        for j in i + 1..=i + match_count {
            if j >= copy_counts.len() {
                break;
            }

            *copy_counts.get_mut(j).unwrap() += copies
        }
    }

    let part_1: usize = match_counts.iter().map(get_card_worth).sum();
    let part_2: usize = copy_counts.iter().sum();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn count_matching_numbers(line: &str) -> usize {
    let mut iterator = line.split(": ").nth(1).unwrap().split(" | ");
    let winning_numbers = parse_numbers(iterator.next().unwrap());
    let own_numbers = parse_numbers(iterator.next().unwrap());

    winning_numbers.intersection(&own_numbers).count()
}

fn parse_numbers(string: &str) -> HashSet<i32> {
    string
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_card_worth(match_count: &usize) -> usize {
    if match_count == &0 {
        0
    } else {
        2_usize.pow((match_count - 1) as u32)
    }
}
