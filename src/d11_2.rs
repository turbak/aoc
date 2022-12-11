use std::{str::FromStr, collections::{HashMap, HashSet}};

#[derive(Debug)]
enum Operation {
    Multiplication(i64),
    Addition(i64),
    Square
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperationError;

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim()
        .split(" ")
        .skip(3);

        let first_item = split.next().unwrap();
        let second_item = split.next().unwrap();
        let last_item = split.next().unwrap();

        if first_item != "old" {
            return Err(ParseOperationError);
        }

        match second_item {
            "+" => Ok(Operation::Addition(last_item.parse().unwrap())),
            "*" => {
                match last_item {
                    "old" => Ok(Operation::Square),
                    _ => Ok(Operation::Multiplication(last_item.parse().unwrap()))
                }
            }
            _ => todo!()
        }
    }
}

#[derive(Debug)]
enum Test {
    Divisible(i64)
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTestError;

impl FromStr for Test {
    type Err = ParseTestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim()
        .split(" ")
        .skip(1);

        match split.next().unwrap() {
            "divisible" => Ok(Test::Divisible(split.skip(1).next().unwrap().parse().unwrap())),
            _ => Err(ParseTestError)
        }
    }
}

#[derive(Debug)]
struct Action {
    test: Test,
    throw_if_true: usize,
    throw_if_false: usize
}

#[derive(Debug)]
struct Monkey {
    number: usize,
    items: Vec<i64>,
    operation: Operation,
    action: Action,
    inspection_count: usize
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMonkeyErr;

impl FromStr for Monkey {
    type Err = ParseMonkeyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let name = lines.next()
        .unwrap()
        .split(" ")
        .skip(1)
        .next()
        .unwrap()
        .trim_end_matches(':')
        .parse()
        .unwrap();

        let starting_items = lines.next()
        .unwrap()
        .trim()
        .split(" ")
        .skip(2)
        .map(|x| x.trim_end_matches(","))
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

        let operation: Operation = lines.next().unwrap().parse().unwrap();

        let test: Test = lines.next().unwrap().parse().unwrap();

        let if_true: usize = lines.next()
        .unwrap()
        .trim()
        .split(" ")
        .skip(5)
        .next()
        .unwrap()
        .parse()
        .unwrap();

        let if_false: usize = lines.next()
        .unwrap()
        .trim()
        .split(" ")
        .skip(5)
        .next()
        .unwrap()
        .parse()
        .unwrap();

        Ok(
            Self{
                number: name,
                items: starting_items,
                operation: operation,
                action: Action { 
                    test: test, 
                    throw_if_true: if_true,
                    throw_if_false: if_false 
                },
                inspection_count: 0
            }
        )

    }
}

impl Monkey {
    fn inspect(&mut self, old_item: i64, worry_val: i64) -> (i64, usize){
        let mut item = old_item;
        self.inspection_count += 1;

        self.operation.perform(&mut item);

        //try not to worry
        item = item % worry_val;

        let throw_to: usize;
        if self.action.test.test(&item) {
            throw_to = self.action.throw_if_true
        } else {
            throw_to = self.action.throw_if_false
        }

        (item, throw_to)
    }

    fn receive_items(&mut self, items: Option<&mut Vec<i64>>) {
        if items.is_some() {
            self.items.append(items.unwrap());
        }
    }
}

impl Operation {
    fn perform(&self, item: &mut i64) {
        match self {
            Operation::Multiplication(x) => *item = *item * x,
            Operation::Addition(x) => *item = *item + x,
            Operation::Square => *item = *item * *item,
        }
    }
}

impl Test {
    fn test(&self, item: &i64) -> bool {
        match self {
            Test::Divisible(x) => item % x == 0,
        }
    }

    fn get_divisor(&self) -> i64 {
        match self {
            Test::Divisible(x) => *x,
        }
    }
}


fn main() {
    let mut monkeys: Vec<Monkey> = include_str!("../inputs/d11")
    .split("\n\n")
    .map(|m| m.parse::<Monkey>().unwrap())
    .collect();

    let mut highest_common_divisor = 1;
    monkeys.iter().for_each(|m| highest_common_divisor *= m.action.test.get_divisor());

    let mut items_in_the_air = HashMap::<usize, Vec<i64>>::new();
    for _ in 0..10000 {
        for m in monkeys.iter_mut().enumerate() {

            m.1.receive_items(items_in_the_air.remove(&m.0).as_mut());

            while let Some(item) = m.1.items.pop() {
                let res = m.1.inspect(item, highest_common_divisor);

                items_in_the_air.entry(res.1).or_insert(Vec::new()).push(res.0);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m1.inspection_count.cmp(&m2.inspection_count));

    let monkey_business = monkeys.pop().unwrap().inspection_count * monkeys.pop().unwrap().inspection_count;

    println!("monkey business: {}", monkey_business);
}