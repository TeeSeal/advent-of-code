use intcode::Program;
use std::fs::read_to_string;
use std::sync::mpsc::{channel};
use std::thread;

mod intcode;

const INPUT_PATH: &str = "./input.txt";

fn attempt(intcode_input: &Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut threads = Vec::new();
    let (base_tx, base_rx) = channel();

    let mut old_rx = base_rx;
    let mut old_tx = base_tx.clone();

    for (i, &setting) in phase_settings.iter().enumerate() {
        old_tx.send(setting).unwrap();
        let (new_tx, new_rx) = channel();
        let input = intcode_input.clone();

        let tx = if i == phase_settings.len() - 1 { base_tx.clone() } else { new_tx.clone() };

        threads.push(thread::spawn(move || Program::new(input, old_rx, tx).run()));

        old_tx = new_tx;
        old_rx = new_rx;
    }

    base_tx.send(0).unwrap();

    threads.into_iter().map(|thread| thread.join().unwrap()).nth(0).unwrap().recv().unwrap()
}

fn find_max_output(input: &Vec<i32>, possible_settings: &mut [i32]) -> i32 {
    let mut settings = Vec::new();
    permutohedron::heap_recursive(possible_settings, |p| settings.push(p.to_vec()));

    settings
        .into_iter()
        .map(|setting| attempt(input, setting))
        .fold(0, i32::max)
}

fn main() {
    let input: Vec<i32> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .split(',')
        .map(|string| string.parse().expect("couldn't parse an integer"))
        .collect();

    let part1 = find_max_output(&input, &mut [0, 1, 2, 3, 4]);
    let part2 = find_max_output(&input, &mut [5, 6, 7, 8, 9]);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
