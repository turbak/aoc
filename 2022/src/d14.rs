use core::time;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display, thread::sleep, env::Args,
};

#[derive(Debug, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
    Source,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Air => write!(f, "."),
            Material::Rock => write!(f, "#"),
            Material::Sand => write!(f, "o"),
            Material::Source => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid_map: HashMap<(usize, usize), Material>,
}

const SOURCE_OF_SAND: (usize, usize) = (500, 0);

impl Grid {
    fn new() -> Self {
        let mut grid_map = HashMap::new();
        grid_map.insert((SOURCE_OF_SAND.1, SOURCE_OF_SAND.0), Material::Source);
        Self { grid_map: grid_map }
    }

    fn apply_rock_vector(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        if start.0 == end.0 && start.1 != end.1 {
            for i in start.1.min(end.1)..start.1.max(end.1) + 1 {
                self.grid_map.insert((i, start.0), Material::Rock);
            }
            return;
        }

        if start.0 != end.0 && start.1 == end.1 {
            for j in start.0.min(end.0)..start.0.max(end.0) + 1 {
                self.grid_map.insert((start.1, j), Material::Rock);
            }
            return;
        }
    }

    fn drop_sand_grain(&mut self) -> bool {
        let i_max = *self
            .grid_map
            .iter()
            .map(|((i, _), _)| i)
            .max()
            .expect("should have found max_i val");
        let j_max = *self
            .grid_map
            .iter()
            .map(|((_, j), _)| j)
            .max()
            .expect("should have found max_j val");
        let j_min = *self
            .grid_map
            .iter()
            .map(|((_, j), _)| j)
            .min()
            .expect("should have found min_j val");

        let mut i = SOURCE_OF_SAND.1;
        let mut j = SOURCE_OF_SAND.0;
        let mut is_fallen = false;

        while !is_fallen {
            if i > i_max || j < j_min || j > j_max {
                is_fallen = true;
                break;
            }

            if !self.grid_map.get(&(i + 1, j)).is_some() {
                i += 1;
            } else if !self.grid_map.get(&(i + 1, j - 1)).is_some() {
                i += 1;
                j -= 1;
            } else if !self.grid_map.get(&(i + 1, j + 1)).is_some() {
                i += 1;
                j += 1;
            } else {
                self.grid_map.insert((i, j), Material::Sand);
                break;
            }
        }

        is_fallen
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i_max = *self
            .grid_map
            .iter()
            .map(|((i, _), _)| i)
            .max()
            .expect("should have found max_i val");
        let j_max = *self
            .grid_map
            .iter()
            .map(|((_, j), _)| j)
            .max()
            .expect("should have found max_j val");
        let i_min = *self
            .grid_map
            .iter()
            .map(|((i, _), _)| i)
            .min()
            .expect("should have found min_i val");
        let j_min = *self
            .grid_map
            .iter()
            .map(|((_, j), _)| j)
            .min()
            .expect("should have found min_j val");

        std::process::Command::new("clear").status().unwrap();

        for i in i_min..i_max + 1 {
            for j in j_min..j_max + 1 {
                write!(
                    f,
                    "{}",
                    self.grid_map.get(&(i, j)).unwrap_or(&Material::Air)
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn main() {
    let mut directions: Vec<VecDeque<(usize, usize)>> = include_str!("../inputs/d14")
        .trim()
        .lines()
        .map(|l| l.split(" -> "))
        .map(|split| {
            split
                .map(|s| {
                    let mut inner_split = s.split(",");
                    (
                        inner_split.next().unwrap().parse().unwrap(),
                        inner_split.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut grid = Grid::new();

    for row in directions.iter_mut() {
        let mut start = &row.pop_front().unwrap();
        for end in row.iter() {
            grid.apply_rock_vector(start, end);
            start = end;
        }
    }

    while !grid.drop_sand_grain() {
        println!("{}", grid);
        sleep(time::Duration::from_millis(50));
    }

    println!(
        "sand_count: {}",
        grid.grid_map
            .iter()
            .filter(|(_, m)| **m == Material::Sand)
            .count()
    );
}
