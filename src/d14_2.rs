use std::{collections::{HashMap, VecDeque}, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
    Source
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
struct  Grid {
    grid_map: HashMap<(usize, usize), Material>,
    i_max: usize,
    j_max: usize,
    i_min: usize,
    j_min: usize,
    floor: usize
}

const SOURCE_OF_SAND: (usize, usize) = (500, 0);

impl Grid {
    fn new() -> Self {
        let mut grid_map = HashMap::new();

        grid_map.insert((SOURCE_OF_SAND.1, SOURCE_OF_SAND.0), Material::Source);

        Self{ grid_map: grid_map, i_max: SOURCE_OF_SAND.1, j_max: SOURCE_OF_SAND.0, i_min: SOURCE_OF_SAND.1, j_min: SOURCE_OF_SAND.0, floor: 0}
    }

    fn apply_rock_vector(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        if start.0 == end.0 && start.1 != end.1 {
            for i in start.1.min(end.1)..start.1.max(end.1)+1 {
                self.grid_map.insert((i, start.0), Material::Rock);
                self.i_max = self.i_max.max(i);
                self.i_min = self.i_min.min(i);
                self.floor = self.i_max+2;
            }
            return;
        }

        if start.0 != end.0 && start.1 == end.1 {
            for j in start.0.min(end.0)..start.0.max(end.0)+1 {
                self.grid_map.insert((start.1, j), Material::Rock);
                self.j_max = self.j_max.max(j);
                self.j_min = self.j_min.min(j);
            }
            return;
        }
    }

    fn get(&self, key: &(usize, usize)) -> Option<&Material> {
        if key.0 == self.floor {
            return Some(&Material::Rock);
        }

        return self.grid_map.get(key);
    }

    fn drop_sand_grain(&mut self) {
        let mut i = SOURCE_OF_SAND.1;
        let mut j = SOURCE_OF_SAND.0;

       loop {            
            if !self.get(&(i+1, j)).is_some() {
                i += 1;
            } else if !self.get(&(i+1, j-1)).is_some() {
                i += 1;
                j -= 1;
            } else if !self.get(&(i+1, j+1)).is_some() {
                i += 1;
                j += 1;
            } else {
                self.grid_map.insert((i, j), Material::Sand);
                self.j_max = self.j_max.max(j);
                self.j_min = self.j_min.min(j);
                self.i_max = self.i_max.max(i);
                self.i_min = self.i_min.min(i);
                break;
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.i_min..self.i_max+1 {
            for j in self.j_min..self.j_max+1 {
                write!(f, "{}", self.grid_map.get(&(i, j)).unwrap_or(&Material::Air))?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
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
                (inner_split.next().unwrap().parse().unwrap(), inner_split.next().unwrap().parse().unwrap())
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

    
    while !(grid.grid_map.get(&(SOURCE_OF_SAND.1, SOURCE_OF_SAND.0)).unwrap() == &Material::Sand) {
        grid.drop_sand_grain();
        //println!("{}", grid);
    }

    println!("sand_count: {}", grid.grid_map.iter().filter(|(_, m)| **m == Material::Sand).count());

}