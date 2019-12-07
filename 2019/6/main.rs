use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT_PATH: &str = "./input.txt";

struct Space {
    orbits: HashMap<String, String>,
}

impl Space {
    fn parse(input: &str) -> Space {
        let orbits: HashMap<String, String> = input
            .lines()
            .map(|line| {
                let mut parts = line.split(')').rev().map(|s| s.to_string());
                (parts.next().unwrap(), parts.next().unwrap())
            })
            .collect();

        Space { orbits }
    }

    fn orbited_objects(&self, from: &str) -> Vec<&String> {
        let mut result = Vec::new();
        let mut current = from;

        loop {
            match self.orbits.get(current) {
                Some(other) => {
                    result.push(other);
                    current = other;
                }
                None => break,
            }
        }

        result
    }

    fn count_jumps(&self, from: &str, to: &str) -> usize {
        let from_orbited = self.orbited_objects(from);
        let to_orbited = self.orbited_objects(to);

        let common = from_orbited
            .iter()
            .find(|obj| to_orbited.contains(obj))
            .unwrap();

        let from_index = from_orbited.iter().position(|a| a == common).unwrap();
        let to_index = to_orbited.iter().position(|a| a == common).unwrap();
        from_index + to_index
    }
}

fn main() {
    let space = Space::parse(&read_to_string(INPUT_PATH).expect("couldn't read input"));
    let total_orbits = space
        .orbits
        .keys()
        .map(|from| space.orbited_objects(from).len())
        .fold(0, |sum, current| sum + current);

    let jumps_to_santa = space.count_jumps("YOU", "SAN");

    println!("Total orbits: {}", total_orbits);
    println!("Trnsfers to Santa: {}", jumps_to_santa);
}
