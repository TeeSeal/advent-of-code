use Instruction::*;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

enum Instruction {
    Add(i64, i64, usize),
    Mul(i64, i64, usize),
    Input(usize),
    Output(i64),
    JumpIfTrue(i64, usize),
    JumpIfFalse(i64, usize),
    LessThan(i64, i64, usize),
    Equals(i64, i64, usize),
    RelativeBaseOffset(i64)
}

impl Instruction {
    fn parse(program: &mut Program) -> Instruction {
        let mut operation = format!("{:0>5}", program.peek(0).to_string());
        let opcode: u32 = operation.split_off(3).parse().unwrap();
        let modes: Vec<u32> = operation
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let a = program.peek(1);
        let b = program.peek(2);
        let c = program.peek(3);

        match opcode {
            1 => Add(
                program.read_param(a, modes[0]),
                program.read_param(b, modes[1]),
                program.get_position(c, modes[2]),
            ),
            2 => Mul(
                program.read_param(a, modes[0]),
                program.read_param(b, modes[1]),
                program.get_position(c, modes[2]),
            ),
            3 => Input(program.get_position(a, modes[0])),
            4 => Output(program.read_param(a, modes[0])),
            5 => JumpIfTrue(
                program.read_param(a, modes[0]),
                program.read_param(b, modes[1]) as usize,
            ),
            6 => JumpIfFalse(
                program.read_param(a, modes[0]),
                program.read_param(b, modes[1]) as usize,
            ),
            7 => LessThan(
                program.read_param(a, modes[0]),
                program.read_param(b, modes[1]),
                program.get_position(c, modes[2]),
            ),
            8 => Equals(
                program.read_param(a, modes[0]),
                program.read_param(b, modes[1]),
                program.get_position(c, modes[2]),
            ),
            9 => RelativeBaseOffset(
                program.read_param(a, modes[0])
            ),
            _ => panic!("invalid opcode"),
        }
    }

    fn jump_size(&self) -> usize {
        match self {
            Input(..) | Output(..) | RelativeBaseOffset(..) => 2,
            JumpIfTrue(..) | JumpIfFalse(..) => 3,
            Add(..) | Mul(..) | LessThan(..) | Equals(..) => 4,
        }
    }

    fn execute(&self, program: &mut Program) {
        let mut jumped = false;

        match self {
            Add(a, b, pos) => { program.intcode.insert(*pos, a + b); },
            Mul(a, b, pos) => { program.intcode.insert(*pos, a * b); },
            Input(pos) => { program.intcode.insert(*pos, program.input.recv().unwrap()); },
            Output(value) => { program.output.send(*value).unwrap(); },
            LessThan(a, b, pos) => { program.intcode.insert(*pos, if a < b { 1 } else { 0 }); },
            Equals(a, b, pos) => { program.intcode.insert(*pos, if a == b { 1 } else { 0 }); },
            RelativeBaseOffset(offset) => { program.relative_base = program.relative_base + *offset; },
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
    intcode: HashMap<usize, i64>,
    cursor: usize,
    relative_base: i64,
    input: Receiver<i64>,
    output: Sender<i64>,
    pub halted: bool,
}

impl Program {
    pub fn new(vec: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> Program {
        let intcode = vec.into_iter().enumerate().collect();
        Program {
            intcode,
            input,
            output,
            cursor: 0,
            relative_base: 0,
            halted: false,
        }
    }

    pub fn run(&mut self) {
        while self.peek(0) != 99 {
            let instruction = Instruction::parse(self);
            instruction.execute(self);
        }

        self.halted = true;
        self.output.send(-1).unwrap();
    }

    fn peek(&mut self, length: usize) -> i64 {
        self.read(self.cursor + length)
    }

    fn read(&mut self, index: usize) -> i64 {
        if !self.intcode.contains_key(&index) {
            self.intcode.insert(index, 0);
        }

        self.intcode.get(&index).unwrap().to_owned()
    }

    fn read_param(&mut self, value: i64, mode: u32) -> i64 {
        match mode {
            0 => self.read(value as usize),
            1 => value,
            2 => self.read((self.relative_base + value) as usize),
            _ => panic!("invalid mode"),
        }
    }

    fn get_position(&self, value: i64, mode: u32) -> usize {
        let result = match mode {
            2 => self.relative_base + value,
            _ => value
        };

        result as usize
    }
}
