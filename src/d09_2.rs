use std::{str::FromStr, collections::{HashSet, VecDeque}};

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
struct Knot {
    i: i64,
    j: i64
}

impl Knot {
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

    fn move_to(&mut self, knot: &Knot) {
        if self.is_adjacent(knot) {
            return;
        }

        if self.i < knot.i {
            self.execute_move(&Direction::Up)
        } else if self.i > knot.i {
            self.execute_move(&Direction::Down)
        }

        if self.j < knot.j {
            self.execute_move(&Direction::Right)
        } else if self.j > knot.j{
            self.execute_move(&Direction::Left)
        }
    }

    fn is_adjacent(&self, knot: &Knot) -> bool {
        (self.i - knot.i).abs() <= 1 && (self.j - knot.j).abs() <= 1
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

    let mut rope = VecDeque::<Knot>::new();
    for _ in 0..10 {
        rope.push_back(Knot::new())
    }

    let mut unique_positions = HashSet::<Knot>::new();

    unique_positions.insert(*rope.back().unwrap());
    for m in moves {
        for _ in 0..m.num_of_steps {
            rope.front_mut().unwrap().execute_move(&m.direction);
            
            let mut prev = &rope.front().unwrap().clone();
            for current in rope.iter_mut().skip(1) {
                current.move_to(prev);
                prev = current;
            }

            unique_positions.insert(*rope.back().unwrap());
        }
    }

    println!("total: {}", unique_positions.len());
}