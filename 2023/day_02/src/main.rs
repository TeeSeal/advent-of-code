use std::fs;
use std::str::FromStr;

enum Cubes {
    Red(i32),
    Green(i32),
    Blue(i32),
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Cubes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [count, color] = s.trim().split(' ').collect::<Vec<&str>>()[..] else {
            return Err(ParseError);
        };
        let parsed_count = count.parse::<i32>().map_err(|_| ParseError)?;

        match color {
            "red" => Ok(Cubes::Red(parsed_count)),
            "green" => Ok(Cubes::Green(parsed_count)),
            "blue" => Ok(Cubes::Blue(parsed_count)),
            _ => Err(ParseError),
        }
    }
}

struct CubeSet {
    cube_groups: Vec<Cubes>,
}

impl FromStr for CubeSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_groups: Vec<Cubes> = s
            .trim()
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(CubeSet { cube_groups })
    }
}

struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [game_str, cube_sets_str] = s.trim().split(": ").collect::<Vec<&str>>()[..] else {
            return Err(ParseError);
        };

        let id: i32 = game_str
            .chars()
            .skip(5)
            .collect::<String>()
            .parse()
            .map_err(|_| ParseError)?;

        let cube_sets: Vec<CubeSet> = cube_sets_str
            .trim()
            .split("; ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Game { id, cube_sets })
    }
}

impl Game {
    fn cube_groups_iter(&self) -> impl Iterator<Item = &Cubes> {
        self.cube_sets.iter().flat_map(|set| &set.cube_groups)
    }

    fn is_possible(&self, red_limit: i32, green_limit: i32, blue_limit: i32) -> bool {
        for cube_group in self.cube_groups_iter() {
            match cube_group {
                Cubes::Red(count) => {
                    if count > &red_limit {
                        return false;
                    }
                }
                Cubes::Green(count) => {
                    if count > &green_limit {
                        return false;
                    }
                }
                Cubes::Blue(count) => {
                    if count > &blue_limit {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn minimum_set_power(&self) -> i32 {
        let mut min_red = None;
        let mut min_green = None;
        let mut min_blue = None;

        for cube_group in self.cube_groups_iter() {
            match cube_group {
                Cubes::Red(count) => match min_red {
                    Some(value) => {
                        if count > value {
                            min_red = Some(count)
                        }
                    }
                    None => min_red = Some(count),
                },
                Cubes::Green(count) => match min_green {
                    Some(value) => {
                        if count > value {
                            min_green = Some(count)
                        }
                    }
                    None => min_green = Some(count),
                },
                Cubes::Blue(count) => match min_blue {
                    Some(value) => {
                        if count > value {
                            min_blue = Some(count)
                        }
                    }
                    None => min_blue = Some(count),
                },
            }
        }

        min_red.unwrap() * min_green.unwrap() * min_blue.unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let games: Vec<Game> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let part_1: i32 = games
        .iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.id)
        .sum();

    let part_2: i32 = games.iter().map(|g| g.minimum_set_power()).sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
