use std::{str::FromStr, collections::HashSet};

struct Move {
    direction: Direction,
    num_of_steps: i64
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct RopeEnd {
    i: i64,
    j: i64
}

impl RopeEnd {
    fn new() -> Self {
        Self { i: 0, j: 0 }
    }
    
    fn execute_move(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.i += 1,
            Direction::Down => self.i -= 1,
            Direction::Left => self.j -= 1,
            Direction::Right => self.j += 1,
        }
    }

    fn move_to(&mut self, rope_end: &RopeEnd) {
        if self.is_adjacent(rope_end) {
            return;
        }

        if self.i < rope_end.i {
            self.execute_move(&Direction::Up)
        } else if self.i > rope_end.i {
            self.execute_move(&Direction::Down)
        }

        if self.j < rope_end.j {
            self.execute_move(&Direction::Right)
        } else if self.j > rope_end.j{
            self.execute_move(&Direction::Left)
        }
    }

    fn is_adjacent(&self, rope_end: &RopeEnd) -> bool {
        (self.i - rope_end.i).abs() <= 1 && (self.j - rope_end.j).abs() <= 1
    }
}

fn main() {
    let moves = include_str!("../inputs/d09")
    .lines()
    .map(|l| {
        let mut split = l.split(" ");
        Move{
            direction: split.next().unwrap().parse().unwrap(),
            num_of_steps: split.next().unwrap().parse().unwrap(),
        }
    });

    let mut head = RopeEnd::new();
    let mut tail = RopeEnd::new();
    let mut unique_positions = HashSet::<RopeEnd>::new();

    unique_positions.insert(tail);
    for m in moves {
        for _ in 0..m.num_of_steps {
            head.execute_move(&m.direction);
            tail.move_to(&head);
            unique_positions.insert(tail);
        }
    }

    println!("total unique tail positions: {}", unique_positions.len());
}