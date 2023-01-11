use std::{collections::HashSet, fmt::{Display, self}, thread, time::Duration, fs};

const LINE: [(usize, usize); 4] = [(0, 3), (0, 4), (0, 5), (0, 6)];
const CROSS: [(usize, usize); 5] = [(0, 4), (1, 3), (1, 4), (1, 5), (2, 4)];
const L: [(usize, usize); 5] = [(0, 5), (1, 5), (0, 3), (0, 4), (2, 5)];
const I: [(usize, usize); 4] = [(0, 3), (1, 3), (2, 3), (3, 3)];
const BOX: [(usize, usize); 4] = [(0, 3), (0, 4), (1, 3), (1, 4)];

struct TetrominoGenerator<'a> {
    current: usize,
    max: usize,
    list: Vec<&'a [(usize, usize)]>,
}

impl<'a> TetrominoGenerator<'a> {
    fn new(max: usize) -> Self {
        Self {
            current: 0,
            max: max,
            list: vec![&LINE, &CROSS, &L, &I, &BOX],
        }
    }
}

impl<'a> Iterator for TetrominoGenerator<'a> {
    type Item = &'a [(usize, usize)];

    fn next(self: &mut TetrominoGenerator<'a>) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }
        let res = self.list.get(self.current % self.list.len()).copied();
        self.current += 1;
        res
    }
}

enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            '>' => Self::Right,
            '<' => Self::Left,
            _ => panic!("invalid direction"),
        }
    }

    fn apply_to_piece(&self, piece: &mut Vec::<(usize, usize)>) {
        match self {
            Direction::Left => piece.iter_mut().for_each(|pos| pos.1-=1),
            Direction::Right => piece.iter_mut().for_each(|pos| pos.1+=1),
            Direction::Down => piece.iter_mut().for_each(|pos| pos.0-=1),
        }
    }
}

struct Chamber {
    directions: Vec<Direction>,
    rocks: HashSet<(usize, usize)>,
    max_height: usize,
    current_direction: usize
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.max_height+1).rev() {
            for j in 0..Self::WIDTH+1 {
                if self.rocks.contains(&(i, j)) {
                    write!(f, "#")?
                } else if i == 0 {
                    write!(f, "-")?
                } else if j == 0 || j == Self::WIDTH {
                    write!(f, "|")?
                } else {
                    write!(f, ".")?
                }
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}

impl Chamber {
    fn new(directions: Vec<Direction>) -> Self {
        Self { directions: directions, rocks: HashSet::new(), max_height: 0, current_direction: 0 }
    }

    fn drop_piece(&mut self, piece: &[(usize, usize)]) {
        let mut new_piece = Vec::with_capacity(piece.len());
        for p in piece.iter() {
            new_piece.push(*p)
        }
        self.setup_piece(&mut new_piece);

        loop {
            let next_direction = self.get_next_direction();
            let mut temp_piece = new_piece.clone();
            next_direction.apply_to_piece(&mut temp_piece);

            if !temp_piece.iter().any(|pos| self.is_occupied(pos)) {
                new_piece = temp_piece.clone()
            }

            temp_piece = new_piece.clone();
            Direction::Down.apply_to_piece(&mut temp_piece);
            if temp_piece.iter().any(|pos| self.is_occupied(pos)) {
                break;
            }
            new_piece=temp_piece
        }
        
        new_piece.iter().for_each(|pos| {self.rocks.insert(*pos);});
        self.max_height = self.max_height.max(new_piece.iter().map(|pos| pos.0).max().unwrap())       
    }

    fn setup_piece(&self, piece: &mut Vec<(usize, usize)>){
        let vertical_location = self.max_height+4;
        piece.iter_mut().for_each(|el| el.0+= vertical_location);
    }

    const WIDTH:usize = 8;

    fn is_occupied(&self, pos: &(usize, usize)) -> bool {
        if pos.0 == 0 || pos.1 == 0 || pos.1 == Self::WIDTH {
            return true;
        }

        return self.rocks.contains(pos);
    }

    fn get_next_direction(&mut self) -> &Direction {
        if self.current_direction >= self.directions.len() {
            self.current_direction = 0;
        }

        let direction = self.directions.get(self.current_direction).unwrap();
        self.current_direction+=1;

        direction
    }
}

const NUM_OF_ROCKS: usize = 1000000000000;
fn main() {
    let directions: Vec<Direction> = include_str!("../inputs/d17_test")
        .chars()
        .map(|c| Direction::new(c))
        .collect();

    let mut chamber = Chamber::new(directions);
    for shape in TetrominoGenerator::new(NUM_OF_ROCKS-1) {
        chamber.drop_piece(shape);
    }

    //fs::write("d17_output", fmt::format(format_args!("{}", chamber))).expect("should write to file");
    println!("chamber height: {}", chamber.max_height);
}