use intcode::Program;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::mpsc::channel;
use std::thread;
use Orientation::*;

mod intcode;

const INPUT_PATH: &str = "./input.txt";
const BLACK: u8 = 0;
const WHITE: u8 = 1;

type Color = u8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn parse(num: i64) -> TurnDirection {
        match num {
            0 => TurnDirection::Left,
            1 => TurnDirection::Right,
            _ => panic!("invalid turn direction"),
        }
    }
}

struct Ship {
    surface: HashMap<Point, u8>,
}

impl Ship {
    fn new(start_panel_color: Color) -> Ship {
        let mut surface = HashMap::new();
        surface.insert(Point { x: 0, y: 0 }, start_panel_color);
        Ship { surface }
    }

    fn color_at(&self, point: &Point) -> Color {
        match self.surface.get(point) {
            Some(&color) => color,
            None => BLACK,
        }
    }

    fn paint(&mut self, point: &Point, color: Color) {
        self.surface.insert(point.clone(), color);
    }
}

struct Robot {
    position: Point,
    orientation: Orientation,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            position: Point { x: 0, y: 0 },
            orientation: Up,
        }
    }

    fn move_forward(&mut self) {
        match self.orientation {
            Up => self.position.y = self.position.y + 1,
            Down => self.position.y = self.position.y - 1,
            Left => self.position.x = self.position.x - 1,
            Right => self.position.x = self.position.x + 1,
        }
    }

    fn turn(&mut self, direction: TurnDirection) {
        self.orientation = match direction {
            TurnDirection::Left => match self.orientation {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            TurnDirection::Right => match self.orientation {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
        }
    }
}

fn paint_registration_identifier(intcode_input: &Vec<i64>, start_panel_color: Color) -> Ship {
    let intcode_input = intcode_input.clone();
    let (input_tx, input_rx) = channel();
    let (output_tx, output_rx) = channel();

    thread::spawn(move || Program::new(intcode_input, input_rx, output_tx).run());

    let mut ship = Ship::new(start_panel_color);
    let mut robot = Robot::new();

    loop {
        let current_color = ship.color_at(&robot.position);
        input_tx.send(current_color as i64).unwrap();
        match output_rx.recv().unwrap() {
            -1 => break,
            color => {
                ship.paint(&robot.position, color as Color);
                let direction = TurnDirection::parse(output_rx.recv().unwrap());
                robot.turn(direction);
                robot.move_forward();
            }
        }
    }

    ship
}

fn format_surface(surface: &HashMap<Point, Color>) -> String {
    let points: Vec<&Point> = surface.keys().collect();
    let xs: Vec<i32> = points.iter().map(|p| p.x).collect();
    let ys: Vec<i32> = points.iter().map(|p| p.y).collect();
    let min_x = xs.iter().fold(0, |a, &b| i32::min(a, b));
    let max_x = xs.iter().fold(0, |a, &b| i32::max(a, b));
    let min_y = ys.iter().fold(0, |a, &b| i32::min(a, b));
    let max_y = ys.iter().fold(0, |a, &b| i32::max(a, b));
    let mut output = String::new();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let point = Point { x, y };

            let chr = match surface.get(&point) {
                Some(&color) => match color {
                    WHITE => '*',
                    BLACK => ' ',
                    _ => panic!("unexpected color")
                },
                None => ' ',
            };

            output.push(chr);
        }

        output.push('\n');
    }

    output
}

fn main() {
    let intcode_input: Vec<i64> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .split(',')
        .map(|string| string.parse().expect("couldn't parse an integer"))
        .collect();

    let part1 = paint_registration_identifier(&intcode_input, BLACK);
    let part2 = paint_registration_identifier(&intcode_input, WHITE);

    println!("Part 1: {}", part1.surface.len());
    println!("Part 2: \n{}", format_surface(&part2.surface));
}
