use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Ground,
    Start,
    Pipe { ends: (Direction, Direction) },
}

impl From<char> for TileKind {
    fn from(value: char) -> Self {
        let ends = match value {
            '.' => return Self::Ground,
            'S' => return Self::Start,
            '|' => (Direction::North, Direction::South),
            '-' => (Direction::East, Direction::West),
            'L' => (Direction::North, Direction::East),
            'J' => (Direction::North, Direction::West),
            '7' => (Direction::South, Direction::West),
            'F' => (Direction::South, Direction::East),
            _ => panic!("invalid tile"),
        };

        Self::Pipe { ends }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    x: usize,
    y: usize,
    kind: TileKind,
}

impl Tile {
    fn has_end(&self, direction: &Direction) -> bool {
        match &self.kind {
            TileKind::Start => false,
            TileKind::Ground => false,
            TileKind::Pipe { ends: (d1, d2) } => d1 == direction || d2 == direction,
        }
    }

    fn other_end(&self, direction: &Direction) -> Option<Direction> {
        match &self.kind {
            TileKind::Start => None,
            TileKind::Ground => None,
            TileKind::Pipe { ends: (d1, d2) } => {
                if d1 == direction {
                    Some(d2.clone())
                } else {
                    Some(d1.clone())
                }
            }
        }
    }
}

struct Map {
    start: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<(usize, usize)> = None;
        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile = Tile {
                            x,
                            y,
                            kind: c.into(),
                        };

                        match tile.kind {
                            TileKind::Start => start = Some((x, y)),
                            _ => (),
                        };

                        tile
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            start: start.unwrap(),
            tiles,
        })
    }
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y)?.get(x)
    }

    fn next_tile(&self, tile: &Tile, came_from: &Direction) -> Option<Tile> {
        let direction = tile.other_end(came_from)?;

        let (x, y) = match direction {
            Direction::North => (tile.x, tile.y.checked_sub(1)?),
            Direction::South => (tile.x, tile.y + 1),
            Direction::East => (tile.x + 1, tile.y),
            Direction::West => (tile.x.checked_sub(1)?, tile.y),
        };

        Some(self.get(x, y)?.clone())
    }

    fn connected_pipes(&self, tile: &Tile) -> Vec<(Tile, Direction)> {
        let mut result = Vec::new();

        if let Some(neighbor) = self.get(tile.x + 1, tile.y) {
            result.push((neighbor.clone(), Direction::East));
        }

        if let Some(neighbor) = self.get(tile.x, tile.y + 1) {
            result.push((neighbor.clone(), Direction::South));
        }

        if tile.x > 0 {
            result.push((
                self.get(tile.x - 1, tile.y).unwrap().clone(),
                Direction::West,
            ));
        }

        if tile.y > 0 {
            result.push((
                self.get(tile.x, tile.y - 1).unwrap().clone(),
                Direction::North,
            ));
        }

        result
            .into_iter()
            .filter(|(tile, direction)| tile.has_end(&direction.opposite()))
            .collect()
    }
}

struct PipeLoop {
    tiles: Vec<Tile>,
}

impl PipeLoop {
    fn x_extremities(&self) -> (usize, usize) {
        let min = self.tiles.iter().map(|t| t.x).min().unwrap();
        let max = self.tiles.iter().map(|t| t.x).max().unwrap();

        (min, max)
    }

    fn y_extremities(&self) -> (usize, usize) {
        let min = self.tiles.iter().map(|t| t.y).min().unwrap();
        let max = self.tiles.iter().map(|t| t.y).max().unwrap();

        (min, max)
    }

    fn contains(&self, tile: &Tile) -> bool {
        let mut north_count = 0;
        let mut south_count = 0;
        for t in &self.tiles {
            if t.y != tile.y {
                continue;
            }

            if t.x == tile.x {
                return false;
            }

            if t.x > tile.x {
                if t.has_end(&Direction::North) {
                    north_count += 1;
                }

                if t.has_end(&Direction::South) {
                    south_count += 1;
                }
            }
        }

        std::cmp::min(north_count, south_count) % 2 == 1
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map: Map = input.parse().unwrap();

    let starting_tile = map.get(map.start.0, map.start.1).unwrap();
    let neighbors = map.connected_pipes(starting_tile);
    let mut loop_tiles: Vec<Tile> = Vec::new();
    let mut part_1: Option<usize> = None;

    for (mut tile, mut direction) in neighbors {
        loop_tiles = vec![tile.clone()];
        direction = direction.opposite();

        loop {
            let Some(new_tile) = map.next_tile(&tile, &direction) else {
                break;
            };
            let Some(new_direction) = tile.other_end(&direction) else {
                break;
            };

            direction = new_direction.opposite();
            tile = new_tile;
            loop_tiles.push(tile);

            if tile.kind == TileKind::Start {
                part_1 = Some(loop_tiles.len() / 2);
                break;
            }
        }

        if part_1.is_some() {
            break;
        }
    }

    println!("Part 1: {}", part_1.unwrap());

    let pipe_loop = PipeLoop { tiles: loop_tiles };
    let (min_x, max_x) = pipe_loop.x_extremities();
    let (min_y, max_y) = pipe_loop.y_extremities();

    let mut area: usize = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let tile = map.get(x, y).unwrap();

            if pipe_loop.contains(tile) {
                area += 1;
            }
        }
    }

    println!("Part 2: {}", area);
}
