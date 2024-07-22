use std::{str::FromStr, collections::{HashMap, HashSet}};

use kiss3d::{light::Light, window::Window, nalgebra::Translation3};

const NUMBER_OF_SIDES:usize = 6;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct ParseCubeErr;

impl FromStr for Cube {
    type Err = ParseCubeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        Ok(Cube{
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        })
    }
}

impl Cube {
    fn is_adjacent(&self, other: &Cube) -> bool {
        return (self.x.max(other.x) - self.x.min(other.x)) +
        (self.y.max(other.y) - self.y.min(other.y)) +
        (self.z.max(other.z) - self.z.min(other.z)) == 1;
    }
    
}

fn main() {
    let cubes: Vec<Cube> = include_str!("../inputs/d18")
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();

    let mut cube_adjacent_count: HashMap<&Cube, HashSet<&Cube>> = cubes.iter().map(|c| (c, HashSet::new())).collect();

    for c1 in cubes.iter() {
        for c2 in cubes.iter() {
            if c1 == c2 {
                continue;
            }
            if  c1.is_adjacent(c2) {
                cube_adjacent_count.entry(c1).and_modify(|hs| {hs.insert(c2);});
                cube_adjacent_count.entry(c2).and_modify(|hs| {hs.insert(c1);});
            }
        }
    }
    println!("{:#?}", cube_adjacent_count);
    let res: usize = cube_adjacent_count.iter().map(|(_, neighbours)| NUMBER_OF_SIDES - neighbours.len()).sum();
    println!("total surface area: {}", res);
    #[cfg(debug_assertions)]
    display(&cubes.iter().collect())
}

fn display(cubes: &Vec<&Cube>) {
    let mut window = Window::new("aoc d18");
    let mut g = window.add_group();

    g.set_color(0.8,0.8,0.8);
    cubes.iter().for_each(|c| {
        let mut cb = g.add_cube(1.0, 1.0, 1.0);
        cb.append_translation(&Translation3::new(c.x as f32, c.y as f32, c.z as f32))
    });

    window.set_light(Light::StickToCamera);

    while window.render() {}
}
