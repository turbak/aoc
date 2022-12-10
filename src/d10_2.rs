use std::{str::FromStr, fmt::Display};

enum Op {
    Noop,
    Addx(i32)
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOpError;

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Op::Noop);
        }

        let mut split = s.split(" ");
        if split.next().unwrap() == "addx" {
            return Ok(Op::Addx(split.next().unwrap().parse().unwrap()));
        }

        return Err(ParseOpError);
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Noop => write!(f, "noop"),
            Op::Addx(x) => write!(f, "addx {}", x),
        }
    }
}

impl Op {
    pub fn execute(&self, register: &mut i32) {
        match self {
            Op::Noop => return,
            Op::Addx(x) => *register+=x
        }
    }

    pub fn num_cycles(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

struct CPU {
    op_queue: Vec<Op>,
    x_register: i32,
    current_op: Op,
    num_cycles_for_current_op: usize
}

impl CPU {
    pub fn new(op_queue: Vec<Op>) -> Self {
        Self { op_queue: op_queue, x_register: 1, current_op: Op::Noop, num_cycles_for_current_op: 0 }
    }

    pub fn run_cycle(&mut self) {
        self.start_cycle();
        self.end_cycle();
    }

    fn start_cycle(&mut self) {
        if self.num_cycles_for_current_op == 0 {
            self.current_op = self.op_queue.pop().unwrap_or(Op::Noop);
            self.num_cycles_for_current_op = self.current_op.num_cycles();
        }

        self.num_cycles_for_current_op-=1;
    }

    fn end_cycle(&mut self) {
        if self.num_cycles_for_current_op == 0 {
            self.current_op.execute(&mut self.x_register);
        }
    }

    pub fn is_finished(&self) -> bool {
        self.op_queue.is_empty() && self.num_cycles_for_current_op == 0
    }
}

struct CRT {
    current_column: i32,
    current_row: i32,
}

impl CRT {
    fn new() -> Self {
        Self { current_column: 0, current_row: 0 }
    }

    fn draw_pixel(&mut self, register_value: i32) {
        self.current_column+= 1;

        if register_value - 1 <= self.current_column && register_value + 1 >= self.current_column {
            print!("#")
        } else {
            print!(".")
        }

        if self.current_column == 40 {
            print!("\n");
            self.current_column = 0;
            self.current_row += 1;
        }

        if self.current_row == 6 {
            print!("\n");
            self.current_row = 0;
        }
    }
}

struct Device {
    cpu: CPU,
    crt: CRT,
}

impl Device {
    fn run(&mut self) {
        while !self.cpu.is_finished() {
            self.cpu.run_cycle();
            self.crt.draw_pixel(self.cpu.x_register);
        }
    }
}



fn main() {
    let operations = include_str!("../inputs/d10")
    .lines()
    .map(|l| l.parse::<Op>().unwrap())
    .rev()
    .collect();

    Device{
        cpu: CPU::new(operations),
        crt: CRT::new()
    }.run()
}
