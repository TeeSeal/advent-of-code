use std::fs;

fn extrapolate(sequence: &Vec<i32>) -> (i32, i32) {
    let mut next = 0;
    let mut prev = 0;
    let mut flag = true;

    let mut sequence = sequence.to_owned();

    loop {
        next += sequence.last().unwrap();
        prev = if flag {
            prev + sequence.first().unwrap()
        } else {
            prev - sequence.first().unwrap()
        };
        flag = !flag;

        sequence = sequence.windows(2).map(|w| w[1] - w[0]).collect();

        if sequence.iter().all(|&n| n == 0) {
            break;
        }
    }

    (prev, next)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let extrapolations: Vec<_> = sequences.iter().map(extrapolate).collect();
    let part_1: i32 = extrapolations.iter().map(|t| t.1).sum();
    let part_2: i32 = extrapolations.iter().map(|t| t.0).sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
