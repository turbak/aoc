#![feature(drain_filter)]

use std::{str::FromStr, collections::HashMap, ops::Range, char::MAX};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Sensor {
    sensor_pos: (isize, isize),
    closest_beacon: (isize, isize)
}


#[derive(Debug, PartialEq, Eq)]
struct ParseSensorError;

impl FromStr for Sensor {
    type Err = ParseSensorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");
        let mut first = split.next().unwrap()["Sensor at ".len()..].split(", ");
        let mut second = split.next().unwrap()["closest beacon is at ".len()..].split(", ");

        Ok(
            Self {
                sensor_pos: (first.next().unwrap()["x=".len()..].parse().unwrap(), first.next().unwrap()["y=".len()..].parse().unwrap()),
                closest_beacon: (second.next().unwrap()["x=".len()..].parse().unwrap(), second.next().unwrap()["y=".len()..].parse().unwrap()),
            }
        )
    }
}

impl Sensor {
    fn distance_to_beacon(&self) -> isize {
        manhattan_distance(self.sensor_pos, self.closest_beacon)
    }
}



fn main() {
    let sensors: Vec<Sensor> = include_str!("../inputs/d15")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    const MAX_DISTANCE: isize = 4000000;

    let mut intervals_by_row: HashMap<isize, Vec<Range<isize>>> = HashMap::new();
    for s in sensors.iter() {
        let distance = s.distance_to_beacon();
        intervals_by_row.entry(s.sensor_pos.0).or_insert(Vec::new()).push(Range { start: s.sensor_pos.1-distance, end: s.sensor_pos.1+distance+1});
        for row in 1..distance+1 {
            intervals_by_row.entry(s.sensor_pos.0+row).or_insert(Vec::new()).push(Range { start: s.sensor_pos.1-distance+row, end: s.sensor_pos.1+distance-row+1});
            intervals_by_row.entry(s.sensor_pos.0-row).or_insert(Vec::new()).push(Range { start: s.sensor_pos.1-distance+row, end: s.sensor_pos.1+distance-row+1});
        }
    }

    let found = intervals_by_row
    .iter_mut()
    .filter(|(row, _)| {
        !(**row < 0 || **row > MAX_DISTANCE)
    })
    .find_map(|(row, intervals)| {
        intervals.drain_filter(|i| i.end < 0 || i.start > MAX_DISTANCE);
        for i in intervals.iter() {
            for j in intervals.iter() {
                if i.start - j.end == 1 && 
                i.start > 0 && i.start < MAX_DISTANCE && j.end > 0 && j.end < MAX_DISTANCE &&
                !intervals.iter().any(|i| i.contains(&(j.end))) {
                    return Some((*row, j.end))
                }
            }
        }
        return None;
    }).unwrap();
    println!("found: {:?}", found);
    println!("coords: {:#?}", found);
    println!("freq: {}", found.0*4000000+found.1)
}

//|x1 - x2| - |y1 - y2| 
fn manhattan_distance(pos: (isize, isize), pos2: (isize, isize)) -> isize {
    (pos.0 - pos2.0).abs() + 
    (pos.1 - pos2.1).abs()
}