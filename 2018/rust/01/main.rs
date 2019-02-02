use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut f = File::open("input.txt").expect("input not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("coudln't read input");
    input.pop(); // remove trailing newline

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let final_freq = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Part 1: {}", final_freq);


    let mut current_freq = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(0);

    for freq in numbers.iter().cycle() {
        current_freq += freq;

        if frequencies.contains(&current_freq) {
            println!("Part 2: {}", current_freq);
            break;
        }

        frequencies.insert(current_freq);
    }
}
