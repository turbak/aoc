#![feature(iter_next_chunk)]

use std::str::FromStr;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug)]
struct ParseCostError;

impl FromStr for Cost {
    type Err = ParseCostError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ").filter(|s| *s != "and");

        let mut cost = Cost{ ore: 0, clay: 0, obsidian: 0 };
        while let Ok([count, resource_type]) = split.next_chunk::<2>() {
            let parsed_count:usize = count.parse().expect("should have parsed count");
            match resource_type.trim_end_matches(".") {
                "ore" => cost.ore = parsed_count,
                "clay" => cost.clay = parsed_count,
                "obsidian" => cost.obsidian = parsed_count,
                _ => return Err(ParseCostError)
            }
        }

        Ok(cost)
    }
}

#[derive(Debug)]
struct Blueprint {
    number: usize,
    ore_robot_cost: Cost,
    clay_robot_const: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

#[derive(Debug)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut end = s.find(":").unwrap();
        let mut start = "Blueprint ".len();
        let name = s[start..end].parse().expect("should have parsed name");


        start = end+1+" Each ore robot costs ".len();
        end = start+s[start..].find(".").unwrap();
        let ore_robot_cost = s[start..end].parse().unwrap();

        start = end+1+" Each clay robot costs ".len();
        end = start+s[start..].find(".").unwrap();
        let clay_robot_const = s[start..end].parse().unwrap();

        start = end+1+" Each obsidian robot costs ".len();
        end = start+s[start..].find(".").unwrap();
        let obsidian_robot_const = s[start..end].parse().unwrap();
        
        start = end+1+" Each geode robot costs ".len();
        end = start+s[start..].find(".").unwrap();
        let geode_robot_const = s[start..end].parse().unwrap();
        
        Ok(Blueprint{
            number: name,
            ore_robot_cost: ore_robot_cost,
            clay_robot_const: clay_robot_const,
            obsidian_robot_cost: obsidian_robot_const,
            geode_robot_cost: geode_robot_const,
        })
    }
}

#[derive(Debug, Clone)]
struct ResourcePool {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    robots: Vec<Robot>
}

impl ResourcePool {
    fn new() -> Self {
        Self { ore: 0, clay: 0, obsidian: 0, geode: 0, robots: vec![Robot::Ore] }
    }

    fn can_afford(&self, cost: &Cost) -> bool {
        return self.clay >= cost.clay && self.ore >= cost.ore && self.obsidian >= cost.obsidian;
    }

    fn create_robot(&mut self, robot: &Robot, blueprint: &Blueprint) {
        let cost = blueprint.get_cost(robot);

        if !self.can_afford(cost) {
            return;
        }

        self.clay -= cost.clay;
        self.ore -= cost.ore;
        self.obsidian -= cost.obsidian;
        self.robots.push(robot.clone());
    }
    
    fn run_production(&mut self) {
        for robot in self.robots.clone().iter() {
            robot.produce_resource_to_pool(self);
        }
    }
}

impl Blueprint {
    fn get_cost(&self, robot: &Robot) -> &Cost {
        match robot {
            Robot::Ore => &self.ore_robot_cost,
            Robot::Clay => &self.clay_robot_const,
            Robot::Obsidian => &self.obsidian_robot_cost,
            Robot::Geode => &self.geode_robot_cost,
        }
    }
}

#[derive(Debug, Clone, EnumIter)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    fn produce_resource_to_pool(&self, pool: &mut ResourcePool) {
        match self {
            Robot::Ore => pool.ore += 1,
            Robot::Clay => pool.clay += 1,
            Robot::Obsidian => pool.obsidian += 1,
            Robot::Geode => pool.geode += 1,
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    let input = include_str!("../inputs/d19_test");

    #[cfg(not(debug_assertions))]
    let input = include_str!("../inputs/d19");

    let blueprints: Vec<Blueprint> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("{:#?}", blueprints);

    let mut total_quality = 0;
    for bp in blueprints {
        let mut max_geodes = 0;
        for robot in vec![Robot::Clay, Robot::Ore] {
            let num_geodes = calculate_largest_number_of_geodes(&bp, ResourcePool::new(), robot, 0);
            max_geodes = max_geodes.max(num_geodes);
        }
        println!("blueprint {} produced {} geodes", bp.number, max_geodes);
        total_quality += max_geodes*bp.number;
    }
    println!("total_quality: {}", total_quality)
}

const MAX_MINUTES: usize = 24;

fn calculate_largest_number_of_geodes(blueprint: &Blueprint, mut resource_pool: ResourcePool, robot_to_produce: Robot, mut minutes: usize) -> usize {
    let cost = blueprint.get_cost(&robot_to_produce);
    while !resource_pool.can_afford(cost) && minutes < MAX_MINUTES-1 {
        resource_pool.run_production();
        minutes+=1;
    }

    resource_pool.run_production();
    minutes+=1;
    if minutes == MAX_MINUTES {
        return resource_pool.geode;
    }
    resource_pool.create_robot(&robot_to_produce, blueprint);
    

    let mut max_num = resource_pool.geode;
    for robot in Robot::iter() {
        let num_geodes = calculate_largest_number_of_geodes(blueprint, resource_pool.clone(), robot, minutes);
        max_num = max_num.max(num_geodes);
    }
    return max_num;
}