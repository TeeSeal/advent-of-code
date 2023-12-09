use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Path {
    current: String,
    steps: u64,
    done: bool
}

impl Path {
    fn new(start: &str) -> Self {
        Path { current: start.to_string(), steps: 0, done: false }
    }

    fn step(&mut self, new_node: &str) {
        self.current = new_node.to_owned();
        self.steps += 1;
    }
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let instructions = lines.next().unwrap();
    let network: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|line| {
            let [node, left, right, ..] = line
                .split(|c: char| !c.is_alphabetic())
                .filter(|s| s != &"")
                .collect::<Vec<_>>()[..]
            else {
                panic!("couldn't parse node")
            };
            (node, (left, right))
        })
        .collect();

    let mut aaa = Path::new("AAA");
    let mut paths: Vec<Path> = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| Path::new(s.to_owned()))
        .collect();

    for instruction in instructions.chars().cycle() {
        if !aaa.done {
            let node = network.get(&*aaa.current).unwrap();
            let next = match instruction {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("invalid instruction"),
            };

            aaa.step(next);

            if aaa.current == "ZZZ" {
                aaa.done = true;
            }
        }

        for path in &mut paths {
            if path.done {
                continue;
            }

            let node = network.get(&*path.current).unwrap();
            let next = match instruction {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("invalid instruction"),
            };

            path.step(next);

            if path.current.ends_with('Z') {
                path.done = true;
            }
        }

        if aaa.done && paths.iter().all(|p| p.done) {
            break;
        }
    }

    let part_2 = lcm(&paths.iter().map(|p| p.steps).collect::<Vec<_>>());

    println!("Part 1: {}", aaa.steps);
    println!("Part 2: {}", part_2)
}
