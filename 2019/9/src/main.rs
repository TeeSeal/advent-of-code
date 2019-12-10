use intcode::Program;
use std::fs::read_to_string;

mod intcode;

const INPUT_PATH: &str = "./input.txt";

fn main() {
    let input: Vec<i64> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .split(',')
        .map(|string| string.parse().expect("couldn't parse an integer"))
        .collect();

    Program::new(input).run();
}
