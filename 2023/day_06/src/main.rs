use std::fs;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn would_win(&self, speed: &u64) -> bool {
        if speed >= &self.time {
            return false;
        }

        (self.time - speed) * speed > self.distance
    }

    fn winning_speeds(&self) -> Vec<u64> {
        (1..self.time - 1)
            .filter(|speed| self.would_win(speed))
            .collect()
    }
}

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_big_number(line: &str) -> u64 {
    line.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let times_line = lines.next().unwrap();
    let distances_line = lines.next().unwrap();

    let times = parse_numbers(times_line);
    let distances = parse_numbers(distances_line);

    let races: Vec<Race> = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    let part_1: u64 = races
        .iter()
        .map(|r| r.winning_speeds().len() as u64)
        .product();
    println!("Part 1: {}", part_1);

    let big_race = Race {
        time: parse_big_number(times_line),
        distance: parse_big_number(distances_line),
    };
    println!("Part 2: {}", big_race.winning_speeds().len());
}
