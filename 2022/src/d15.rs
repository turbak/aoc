use std::{str::FromStr, collections::{HashMap, HashSet}};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Sensor {
    sensor_pos: (i32, i32),
    closest_beacon: (i32, i32)
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
    fn distance_to_beacon(&self) -> i32 {
        manhattan_distance(self.sensor_pos, self.closest_beacon)
    }
}

fn main() {
    let sensors: HashSet<Sensor> = include_str!("../inputs/d15")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let max_j = sensors.iter().map(|s| s.sensor_pos.0+s.distance_to_beacon()).max().unwrap();
    let min_j = sensors.iter().map(|s| s.sensor_pos.0-s.distance_to_beacon()).min().unwrap();
    const ROW_NUM: i32 = 2000000; 

    let mut count = 0;
    for j in min_j..max_j+1 {
        for sensor in sensors.iter() {
            if (j, ROW_NUM) == sensor.sensor_pos || (j, ROW_NUM) == sensor.closest_beacon {
                continue;
            }
            let sensor_to_beacon = sensor.distance_to_beacon();
            let pos_to_sensor = manhattan_distance((j, ROW_NUM), sensor.sensor_pos);
            if pos_to_sensor <= sensor_to_beacon {
                count+=1;
                break
            }
        }
    }
    println!("count: {}", count);
    
}

fn manhattan_distance(pos: (i32, i32), pos2: (i32, i32)) -> i32 {
    (pos.0 - pos2.0).abs() + 
    (pos.1 - pos2.1).abs()
}