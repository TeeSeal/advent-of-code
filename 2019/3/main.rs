use std::fs::read_to_string;
use std::collections::HashSet;

const INPUT_PATH: &str = "./input.txt";

use Direction::*;
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn parse_direction(input: &str) -> (Direction, u32) {
    let mut input = input.to_string();
    let distance: u32 = input.split_off(1).parse().expect("failed to parse uint");

    let direction = match input.chars().next().unwrap() {
        'U' => Up,
        'D' => Down,
        'L' => Left,
        'R' => Right,
        _ => panic!("unknown direction")
    };

    (direction, distance)
}

fn draw_path(central_port: (i32, i32), directions: &str) -> Vec<(i32, i32)> {
    let directions: Vec<&str> = directions.split(',').collect();
    let mut path = vec![central_port];

    for direction in directions {
        let (direction, distance) = parse_direction(direction);

        for _ in 0..distance {
            let (x, y) = path.last().unwrap().clone();

            let new_point = match direction {
                Up => (x, y - 1),
                Down => (x, y + 1),
                Left => (x - 1, y),
                Right => (x + 1, y)
            };

            path.push(new_point);
        }
    }

    path.remove(0);
    path
}

fn intersections_for_paths(path1: &Vec<(i32, i32)>, path2: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let set1: HashSet<(i32, i32)> = path1.iter().cloned().collect();
    let set2: HashSet<(i32, i32)> = path2.iter().cloned().collect();
    set1.intersection(&set2).cloned().collect()
}

fn all_intersections(paths: &Vec<Vec<(i32, i32)>>) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for pair in paths.windows(2) {
        if let [path1, path2] = pair {
            result.extend(intersections_for_paths(&path1, &path2).into_iter())
        }
    }

    result
}

fn distance(p1: &(i32, i32), p2: &(i32, i32)) -> u32 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u32
}

fn main() {
    let central_port = (0, 0);
    let paths: Vec<Vec<(i32, i32)>> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .lines()
        .map(|line| draw_path(central_port, line))
        .collect();

    let intersections = all_intersections(&paths);

    let mut distances: Vec<u32> = intersections.iter().map(|point| distance(&central_port, point)).collect();
    distances.sort();
    println!("Part 1: {}", distances[0]);

    let mut step_counts: Vec<u32> = intersections.iter().map(|&point| {
        paths.iter().fold(0, |total, path| {
            let steps = match path.iter().position(|&p| p == point) {
                Some(position) => position as u32,
                None => 0
            } + 1; // + 1 to consider the jump from central_port

            total + steps
        })
    }).collect();
    step_counts.sort();
    println!("Part 2: {}", step_counts[0]);
}
