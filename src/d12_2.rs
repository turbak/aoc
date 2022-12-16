#![feature(let_chains)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::{Hash, Hasher},
};
#[derive(Eq, Debug)]
struct Vertex {
    height: usize,
    i: usize,
    j: usize,
    is_end: bool,
}

#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

impl Vertex {
    fn new(c: char, i: usize, j: usize) -> Vertex {
        Vertex {
            height: height_from_char(&c),
            i: i,
            j: j,
            is_end: c == 'E',
        }
    }
}
impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state)
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.i == other.i && self.j == other.j
    }
}

fn main() {
    let grid: Vec<Vec<Vertex>> = include_str!("../inputs/d12")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| Vertex::new(c, i, j))
                .collect()
        })
        .collect();

    let mut vec_map = HashMap::<(i32, i32), &Vertex>::new();
    grid.iter().for_each(|v| {
        v.iter().for_each(|vrtx| {
            vec_map.insert((vrtx.i as i32, vrtx.j as i32), vrtx);
        })
    });

    let mut adjacency_list = HashMap::new();

    for (i1, row) in grid.iter().enumerate() {
        for (j1, vrtx) in row.iter().enumerate() {
            let j = j1 as i32;
            let i = i1 as i32;
            let entry = adjacency_list.entry(vrtx).or_insert(Vec::new());
            for pos in vec![(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
                if let Some(n) = vec_map.get(&pos) {
                    entry.push(*n)
                }
            }
        }
    }

    let end = *adjacency_list.iter().find(|(v, _)| (**v).is_end).unwrap().0;
    let starts: Vec<&Vertex> = adjacency_list
        .iter()
        .filter(|(v, _)| (**v).height == 1)
        .map(|(v, _)| *v)
        .collect();

    let mut min_steps = usize::MAX;
    for start in starts {
        let current_steps = dijkstra(start, end, &adjacency_list).len();
        {
            if current_steps > 0 {
                min_steps = min_steps.min(current_steps)
            }
        }
    }

    println!("steps taken: {}", min_steps - 1);
}

fn dijkstra<'a>(
    start: &'a Vertex,
    end: &'a Vertex,
    adjacency_list: &HashMap<&'a Vertex, Vec<&'a Vertex>>,
) -> Vec<&'a Vertex> {
    let mut unexplored = BinaryHeap::new();
    let mut distances = HashMap::<&Vertex, usize>::new();
    let mut visited = HashSet::new();
    let mut path = HashMap::new();

    distances.insert(start, 0);
    unexplored.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = unexplored.pop() {
        if !visited.insert(vertex) {
            continue;
        }

        if vertex == end {
            let mut path_vec = vec![vertex];
            let mut current_vertex = vertex;

            while let Some(prev_vertex) = path.get(current_vertex) {
                path_vec.push(*prev_vertex);
                current_vertex = prev_vertex;
            }
            return path_vec;
        }

        for neighbour in adjacency_list.get(vertex).unwrap().iter() {
            if neighbour.height > vertex.height && neighbour.height.abs_diff(vertex.height) > 1 {
                continue;
            }

            let new_distance = distance + neighbour.height;
            let is_shorter = distances
                .get(*neighbour)
                .map_or(true, |&current| new_distance < current);

            if is_shorter {
                distances.insert(*neighbour, new_distance);
                unexplored.push(Visit {
                    vertex: *neighbour,
                    distance: new_distance,
                });
                path.insert(*neighbour, vertex);
            }
        }
    }

    return vec![];
}

fn height_from_char(c: &char) -> usize {
    match *c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'S' => 1,
        'E' => 26,
        _ => todo!(),
    }
}
