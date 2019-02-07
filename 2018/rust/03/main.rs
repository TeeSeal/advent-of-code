use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Eq, PartialEq, Hash)]
struct Point(i32, i32);

struct Claim {
    id: i32,
    points: HashSet<Point>
}

impl Claim {
    fn new(params: Vec<i32>) -> Claim {
        let mut points = HashSet::new();

        for x in params[1]..(params[1] + params[3]) {
            for y in params[2]..(params[2] + params[4]) {
                points.insert(Point(x, y));
            }
        }

        Claim { id: params[0], points }
    }

    fn overlapping_points<'a>(&'a self, other: &'a Claim) -> HashSet<&Point> {
        self.points.intersection(&other.points).collect()
    }
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<i32> = s
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Claim::new(params))
    }
}

fn main() {
    let mut f = File::open("input.txt").expect("input not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("coudln't read input");
    input.pop(); // remove trailing newline
    let claims: Vec<Claim> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut overlapping_points: HashSet<&Point> = HashSet::new();
    let mut non_overlapping_claims: HashSet<i32> = claims.iter().map(|c| c.id).collect();

    for i in 0..claims.len() {
        for j in (i + 1)..claims.len() {
            let overlaps = claims[i].overlapping_points(&claims[j]);
            if overlaps.len() > 0 {
                non_overlapping_claims.remove(&claims[i].id);
                non_overlapping_claims.remove(&claims[j].id);
            }
            overlapping_points.extend(overlaps);
        }
    }


    println!("Part 1: {}", overlapping_points.len());
    println!("Part 2: {}", non_overlapping_claims.iter().nth(0).unwrap());
}
