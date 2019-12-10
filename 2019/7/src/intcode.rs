use std::sync::mpsc::{Receiver, Sender};
use Instruction::*;

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
            Input(pos) => program.vec[*pos] = program.input.recv().unwrap(),
            Output(value) => program.output.send(*value).unwrap(),
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

pub struct Program {
    vec: Vec<i32>,
    cursor: usize,
    input: Receiver<i32>,
    output: Sender<i32>,
}

impl Program {
    pub fn new(vec: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> Program {
        Program {
            vec,
            input,
            output,
            cursor: 0,
        }
    }

    pub fn run(mut self) -> Receiver<i32> {
        if self.peek(0) == 99 { return self.input; }

        let instruction = Instruction::parse(&self);
        instruction.execute(&mut self);
        self.run()
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
