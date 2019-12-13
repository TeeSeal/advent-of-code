use std::fs::read_to_string;
use std::cmp::Ordering;

const INPUT_PATH: &str = "./input.txt";

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn to_polar(&self) -> PolarPoint {
        PolarPoint {
            r: (self.x.powi(2) + self.y.powi(2)).sqrt(),
            theta: (self.y / self.x).atan() + self.polar_offset(),
        }
    }

    fn origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    fn visible(&self, points: &Vec<Point>) -> Vec<Point> {
        let mut shifted = shift_to_center(&points, self);
        shifted.retain(|p| !p.origin());
        let visible = visible_from_origin(&shifted);
        shift_to_center(&visible, &Point { x: -self.x, y: -self.y })
    }

    fn polar_offset(&self) -> f64 {
        if self.x < 0.0 {
            std::f64::consts::PI
        } else {
            0.0
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PolarPoint {
    r: f64,
    theta: f64,
}

fn shift_to_center(vec: &Vec<Point>, center: &Point) -> Vec<Point> {
    vec.into_iter().map(|p| Point { x: p.x - center.x, y: p.y - center.y }).collect()
}

fn visible_from_origin(points: &Vec<Point>) -> Vec<Point> {
    let mut points: Vec<Point> = points.clone();

    points.sort_by(|a, b| {
        let a = a.to_polar();
        let b = b.to_polar();

        match a.theta.partial_cmp(&b.theta).unwrap() {
            Ordering::Equal => a.r.partial_cmp(&b.r).unwrap(),
            other => other
        }
    });

    points.dedup_by_key(|p| p.to_polar().theta);
    points
}

fn find_base(asteroids: &Vec<Point>) -> Point {
    let mut result = asteroids.clone();
    result.sort_by_cached_key(|a| a.visible(asteroids).len());
    result.last().unwrap().to_owned()
}

fn destroyed_from(base: &Point, asteroids: &Vec<Point>) -> Vec<Point> {
    let mut asteroids = asteroids.clone();
    let mut destroyed = Vec::new();

    while destroyed.len() < 200 {
        destroyed.extend(base.visible(&asteroids));
        asteroids.retain(|a| !destroyed.contains(a));
    }

    return destroyed
}

fn main() {
    let asteroids: Vec<Point> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Point {
                    x: x as f64,
                    y: y as f64,
                })
        })
        .collect();

    let base = find_base(&asteroids);
    let destroyed = destroyed_from(&base, &asteroids);
    println!("Part 1: {}", base.visible(&asteroids).len());
    println!("Part 2: {}", (destroyed[199].x * 100.0 + destroyed[199].y) as i32);
}
