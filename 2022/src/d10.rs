use std::{fmt::Display, str::FromStr};

enum Op {
    Noop,
    Addx(i32),
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
    fn execute(&self, register: &mut i32) {
        println!("{} ", self);
        match self {
            Op::Noop => return,
            Op::Addx(x) => *register += x,
        }
    }

    fn num_cycles(&self) -> usize {
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
    num_cycles_for_current_op: usize,
}

impl CPU {
    fn new(op_queue: Vec<Op>) -> Self {
        Self {
            op_queue: op_queue,
            x_register: 1,
            current_op: Op::Noop,
            num_cycles_for_current_op: 0,
        }
    }

    fn run_cycle(&mut self) {
        if self.num_cycles_for_current_op == 0 {
            self.current_op.execute(&mut self.x_register);
            self.current_op = self.op_queue.pop().unwrap_or(Op::Noop);
            self.num_cycles_for_current_op = self.current_op.num_cycles();
        }
        self.num_cycles_for_current_op -= 1;
    }
}

fn main() {
    let operations = include_str!("../inputs/d10")
        .lines()
        .map(|l| l.parse::<Op>().unwrap())
        .rev()
        .collect();

    let mut cpu = CPU::new(operations);
    let mut sum_of_signal_strength = 0;

    let cycle_check_vec = vec![20, 60, 100, 140, 180, 220];
    let mut cycle_num = 0;
    for cycle_check in cycle_check_vec.iter() {
        for _ in cycle_num..*cycle_check {
            cpu.run_cycle();
            cycle_num += 1;
        }
        let signal_str = cpu.x_register * cycle_num;
        sum_of_signal_strength += signal_str;
    }

    println!("sum of signal strength: {}", sum_of_signal_strength)
}
