use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: usize,
    is_open: bool,
}

fn main() {
    let split_lines = include_str!("../inputs/d16_test")
        .lines()
        .map(|l| l.split("; "));

    let mut valves = HashMap::new();
    let mut adjecency_matrix = HashMap::new();
    for mut line in split_lines {
        let first = line.next().unwrap();
        let second = line.next().unwrap();
        let valve_split = &mut first.split(" ");

        let valve = Valve {
            name: valve_split.skip(1).next().unwrap(),
            flow_rate: valve_split.skip(2).next().unwrap()["rate=".len()..]
                .parse()
                .unwrap(),
            is_open: false,
        };

        adjecency_matrix.insert(
            valve.name,
            second
                .split(" ")
                .skip(4)
                .map(|s| s.trim_end_matches(","))
                .collect::<Vec<&str>>(),
        );
        valves.insert(valve.name, valve);
    }
    println!("{:#?}", adjecency_matrix);
    println!("solved: {}", solve(&mut valves, adjecency_matrix))
}

type Path<'a> = Vec<&'a str>;

const MAX_MINUTES: usize = 30;

fn solve<'a>(
    valves: &'a mut HashMap<&'a str, Valve>,
    adjecency_matrix: HashMap<&str, Vec<&'a str>>,
) -> usize {
    let mut paths = Vec::<Path>::new();

    find_all_paths(
        valves.get("AA").unwrap().name,
        vec![],
        valves,
        &adjecency_matrix,
        MAX_MINUTES,
        &mut paths,
    );

    println!("all path found, calculating best");

    let best_path = paths
        .iter()
        .map(|names| {
            names
                .iter()
                .map(|name| valves.get(name).unwrap())
                .collect::<Vec<&Valve>>()
        })
        .max_by(|x, y| calculate_path_flow(x, false).cmp(&calculate_path_flow(y, false)))
        .unwrap();
    println!("best: {:#?}", best_path);
    calculate_path_flow(&best_path, true)
}

fn calculate_path_flow(valves: &Vec<&Valve>, log_path: bool) -> usize {
    let mut open_valves = HashSet::<&Valve>::new();
    let mut total_flow = 0;
    let mut minutes_left = MAX_MINUTES;
    let mut valve_iter = valves.iter().skip(1);

    while minutes_left > 0 {
        let released_pressure = open_valves
        .iter()
        .map(|open_valve| open_valve.flow_rate)
        .sum::<usize>();
        total_flow += released_pressure;

        if log_path {
            println!("\n== Minute {} ==", MAX_MINUTES-minutes_left+1);
            println!("Valves {:?} are open, releasing {} pressure.", open_valves.iter().map(|v| v.name).collect::<Vec<&str>>(), released_pressure);
        }

        if let Some(next_valve) = valve_iter.next() {
            if log_path {
                println!("You move to valve {}.", next_valve.name);
            }
            if next_valve.flow_rate > 0 && !open_valves.contains(next_valve) {
                if minutes_left == 0 {
                    break;
                }
                minutes_left -= 1;
                if log_path {
                    println!("\n== Minute {} ==", MAX_MINUTES-minutes_left+1);
                    println!("Valves {:?} are open, releasing {} pressure.", open_valves.iter().map(|v| v.name).collect::<Vec<&str>>(), released_pressure);
                    println!("You open valve {}.", next_valve.name);
                }
                total_flow += released_pressure;
                open_valves.insert(next_valve);
            }
        }
        minutes_left -= 1;
    }
    total_flow
}

fn find_all_paths<'a>(
    current_valve: &'a str,
    mut path: Path<'a>,
    valves: &mut HashMap<&'a str, Valve>,
    adjecency_matrix: &HashMap<&str, Vec<&'a str>>,
    mut minutes_left: usize,
    visited: &mut Vec<Path<'a>>,
) {
    if minutes_left == 0 {
        visited.push(path);
        return;
    }

    path.push(current_valve);

    for name in adjecency_matrix.get(current_valve).unwrap() {
        if let Some(valve) = valves.get_mut(name) {
            if valve.flow_rate > 0 && !valve.is_open {
                if minutes_left == 1 {
                    continue;
                }
                valve.is_open = true;
                minutes_left -= 1;
            }
        }

        find_all_paths(
            name,
            path.clone(),
            valves,
            adjecency_matrix,
            minutes_left - 1,
            visited,
        );

        if let Some(valve) = valves.get_mut(name) {
            if valve.is_open {
                valve.is_open = false;
                minutes_left += 1;
            }
        }
    }
}
