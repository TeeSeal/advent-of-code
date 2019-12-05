use std::fs::read_to_string;
use std::io;
use Instruction::*;

const INPUT_PATH: &str = "./input.txt";

fn read_i32() -> i32 {
    println!("Input: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

enum Instruction {
    Add(i32, i32, usize),
    Mul(i32, i32, usize),
    Input(usize),
    Output(i32),
    JumpIfTrue(i32, usize),
    JumpIfFalse(i32, usize),
    LessThan(i32, i32, usize),
    Equals(i32, i32, usize),
}

impl Instruction {
    fn parse(program: &Program) -> Instruction {
        let mut operation = format!("{:0>5}", program.peek(0).to_string());
        let opcode: u32 = operation.split_off(3).parse().unwrap();
        let modes: Vec<u32> = operation
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        match opcode {
            1 => Add(
                program.read_param(program.peek(1), modes[0]),
                program.read_param(program.peek(2), modes[1]),
                program.peek(3) as usize,
            ),
            2 => Mul(
                program.read_param(program.peek(1), modes[0]),
                program.read_param(program.peek(2), modes[1]),
                program.peek(3) as usize,
            ),
            3 => Input(program.peek(1) as usize),
            4 => Output(program.read_param(program.peek(1), modes[0])),
            5 => JumpIfTrue(
                program.read_param(program.peek(1), modes[0]),
                program.read_param(program.peek(2), modes[1]) as usize,
            ),
            6 => JumpIfFalse(
                program.read_param(program.peek(1), modes[0]),
                program.read_param(program.peek(2), modes[1]) as usize,
            ),
            7 => LessThan(
                program.read_param(program.peek(1), modes[0]),
                program.read_param(program.peek(2), modes[1]),
                program.peek(3) as usize,
            ),
            8 => Equals(
                program.read_param(program.peek(1), modes[0]),
                program.read_param(program.peek(2), modes[1]),
                program.peek(3) as usize,
            ),
            _ => panic!("invalid opcode"),
        }
    }

    fn jump_size(&self) -> usize {
        match self {
            Input(..) | Output(..) => 2,
            JumpIfTrue(..) | JumpIfFalse(..) => 3,
            Add(..) | Mul(..) | LessThan(..) | Equals(..) => 4,
        }
    }

    fn execute(&self, program: &mut Program) {
        let mut jumped = false;

        match self {
            Add(a, b, pos) => program.vec[*pos] = a + b,
            Mul(a, b, pos) => program.vec[*pos] = a * b,
            Input(pos) => program.vec[*pos] = read_i32(),
            Output(value) => println!("Output: {}", value),
            LessThan(a, b, pos) => program.vec[*pos] = if a < b { 1 } else { 0 },
            Equals(a, b, pos) => program.vec[*pos] = if a == b { 1 } else { 0 },
            JumpIfTrue(value, pos) => {
                if *value != 0 {
                    program.cursor = *pos;
                    jumped = true;
                }
            }
            JumpIfFalse(value, pos) => {
                if *value == 0 {
                    program.cursor = *pos;
                    jumped = true;
                }
            }
        }

        if !jumped {
            program.cursor += self.jump_size();
        }
    }
}

struct Program {
    vec: Vec<i32>,
    cursor: usize,
}

impl Program {
    fn new(vec: Vec<i32>) -> Program {
        Program { vec, cursor: 0 }
    }

    fn run(&mut self) {
        if self.vec[self.cursor] == 99 {
            return;
        }

        let instruction = Instruction::parse(&self);
        instruction.execute(self);

        self.run();
    }

    fn peek(&self, length: usize) -> i32 {
        self.vec[self.cursor + length]
    }

    fn read_param(&self, value: i32, mode: u32) -> i32 {
        match mode {
            0 => self.vec[value as usize],
            1 => value,
            _ => panic!("invalid mode"),
        }
    }
}

fn main() {
    let input: Vec<i32> = read_to_string(INPUT_PATH)
        .expect("couldn't read input")
        .trim()
        .split(',')
        .map(|string| string.parse().expect("couldn't parse an integer"))
        .collect();

    let mut program = Program::new(input);
    program.run();
}
