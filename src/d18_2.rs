extern crate kiss3d;

use kiss3d::window::Window;
use kiss3d::{light::Light, nalgebra::Translation3};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const NUMBER_OF_SIDES: usize = 6;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
    num_sides_exposed: usize,
}

#[derive(Debug)]
enum CubeType {
    Lava(Cube),
    Air,
}

#[derive(Debug)]
struct ParseCubeErr;

impl FromStr for Cube {
    type Err = ParseCubeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        Ok(Cube {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
            num_sides_exposed: 0,
        })
    }
}

impl Cube {
    fn is_adjacent(&self, other: &Cube) -> bool {
        return (self.x.max(other.x) - self.x.min(other.x))
            + (self.y.max(other.y) - self.y.min(other.y))
            + (self.z.max(other.z) - self.z.min(other.z))
            == 1;
    }
}

fn main() {
    let mut cubes: Vec<Cube> = include_str!("../inputs/d18")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;

    for c in cubes.iter_mut() {
        c.x +=1;
        c.y+=1;
        c.z+=1;
        x_max = x_max.max(c.x);
        y_max = y_max.max(c.y);
        z_max = z_max.max(c.z);
    }

    let mut area = Vec::<Vec<Vec<CubeType>>>::with_capacity(x_max + 1);
    for _x in 0..x_max + 4 {
        let mut y_dim = Vec::with_capacity(y_max + 1);
        for _y in 0..y_max + 4 {
            let mut z_dim = Vec::with_capacity(z_max + 1);
            for _z in 0..z_max + 4 {
                z_dim.push(CubeType::Air);
            }
            y_dim.push(z_dim);
        }
        area.push(y_dim);
    }

    for c in cubes.iter() {
        area[c.x][c.y][c.z] = CubeType::Lava(c.clone())
    }

    bfs(&mut area);
    println!("{:?}", area);

    let total_sides_exposed: usize = area
        .iter()
        .flatten()
        .flatten()
        .filter_map(|at| {
            if let CubeType::Lava(cube) = at {
                return Some(cube.num_sides_exposed);
            }
            None
        })
        .sum();

    println!("total sides exposed: {}", total_sides_exposed);

    let to_display = area
        .iter()
        .flatten()
        .flatten()
        .filter_map(|at| {
            if let CubeType::Lava(cube) = at {
                if cube.num_sides_exposed > 0 {
                    return Some(cube);
                }
            }
            None
        })
        .collect();

    #[cfg(debug_assertions)]
    display(&to_display);
}

fn bfs(area: &mut Vec<Vec<Vec<CubeType>>>) {
    let mut visited = HashSet::new();
    let mut to_see = Vec::new();
    to_see.push((0, 0, 0));

    while let Some((x, y, z)) = to_see.pop() {
        if !visited.insert((x, y, z)) {
            continue;
        }

        for (x, y, z) in get_neighbours(area, x, y, z) {
            if let CubeType::Lava(cube) = &mut area[x][y][z] {
                cube.num_sides_exposed += 1;
                continue;
            }

            to_see.push((x, y, z))
        }
    }
}

fn get_neighbours(
    area: &Vec<Vec<Vec<CubeType>>>,
    x: usize,
    y: usize,
    z: usize,
) -> Vec<(usize, usize, usize)> {
    let mut neighbours = Vec::new();

    if x > 0 {
        neighbours.push((x - 1, y, z));
    }

    if y > 0 {
        neighbours.push((x, y - 1, z))
    }

    if z > 0 {
        neighbours.push((x, y, z - 1));
    }

    if x < area.len() - 1 {
        neighbours.push((x + 1, y, z))
    }

    let y_dim = &area[x];

    if y < y_dim.len() - 1 {
        neighbours.push((x, y + 1, z));
    }

    let z_dim = &y_dim[y];

    if z < z_dim.len() - 1 {
        neighbours.push((x, y, z + 1))
    }

    return neighbours;
}

fn display(cubes: &Vec<&Cube>) {
    let mut window = Window::new("aoc d18");
    let mut g = window.add_group();

    //g.set_color(0.8,0.8,0.8);
    cubes.iter().for_each(|c| {
        let mut cb = g.add_cube(1.0, 1.0, 1.0);
        cb.append_translation(&Translation3::new(c.x as f32, c.y as f32, c.z as f32));
        cb.set_color(c.num_sides_exposed as f32 / 10.0, 0.8, 0.8)
    });

    window.set_light(Light::StickToCamera);

    while window.render() {}
}
