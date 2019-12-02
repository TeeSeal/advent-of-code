use std::fs::read_to_string;
use std::ops::{Mul, Add};

const INPUT_PATH: &str = "./input.txt";
const DESIRED_OUTPUT: i32 = 19690720;

fn run_program(program: &mut Vec<i32>, cursor: usize) -> i32 {
    match program[cursor] {
        99 => program[0],
        1 | 2 => {
            let op = if program[cursor] == 1 { Add::add } else { Mul::mul };

            let i = program[cursor + 1] as usize;
            let j = program[cursor + 2] as usize;
            let k = program[cursor + 3] as usize;

            program[k] = op(program[i], program[j]);
            run_program(program, cursor + 4)
        },
        _ => panic!("got invalid op code")
    }
}

fn restore_and_run(program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut clone = program.clone();
    clone[1] = noun;
    clone[2] = verb;
    run_program(&mut clone, 0)
}

fn main() {
    let program: Vec<i32> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .split(',')
        .map(|string| string.parse().expect("couldn't parse an integer"))
        .collect();

    println!("Part 1: {}", restore_and_run(&program, 12, 2));

    for noun in 1..=99 {
        for verb in 1..=99 {
            if restore_and_run(&program, noun, verb) == DESIRED_OUTPUT {
                let result = noun * 100 + verb;
                println!("Part 2: {}", result);
                return;
            }
        }
    }
}
