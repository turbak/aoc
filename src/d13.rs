#![feature(slice_take)]

use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
enum Value {
    Integer(usize),
    List(Vec<Box<Value>>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(x) => write!(f, "{}", x),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    item.fmt(f)?;
                    if i < items.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValueError;

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str_vec: Vec<char> = s.chars().collect();
        let len = str_vec.len();
        return Ok(Self::from_slice(&mut &mut str_vec[1..len]));
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Value::Integer(x) => match other {
                Value::Integer(y) => x.cmp(y),
                Value::List(_) => Value::List(vec![Box::new(Value::Integer(*x))]).cmp(other),
            },
            Value::List(x) => match other {
                Value::Integer(y) => self.cmp(&Value::List(vec![Box::new(Value::Integer(*y))])),
                Value::List(y) => {
                    for (i, item_x) in x.iter().enumerate() {
                        if let Some(item_y) = y.get(i) {
                            if item_x > item_y {
                                println!("{} > {}", self, other);
                                return std::cmp::Ordering::Greater;
                            }
                            if item_x < item_y {
                                return std::cmp::Ordering::Less;
                            }
                        } else {
                            println!("{} > {}", self, other);
                            return std::cmp::Ordering::Greater; //right size runs out first
                        }
                    }

                    if x.len() == y.len() {
                        println!("{} == {}", self, other);
                        return std::cmp::Ordering::Equal;
                    }
                    println!("{} < {}", self, other);
                    return std::cmp::Ordering::Less;
                }
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Value {}

impl Value {
    fn from_slice(mut s: &mut &mut [char]) -> Self {
        let mut list = Vec::<Box<Value>>::new();
        let mut current_num = String::from("");
        while let Some(c) = (*s).take_first_mut() {
            if c.is_numeric() {
                current_num.push(*c);
                continue;
            }

            if !current_num.is_empty() {
                list.push(Box::new(Value::Integer(current_num.parse().unwrap())));
                current_num.clear();
            }

            if c == &',' {
                continue;
            }

            if c == &'[' {
                list.push(Box::new(Self::from_slice(&mut s)));

                continue;
            }

            if c == &']' {
                return Value::List(list);
            }
        }

        return Value::List(list);
    }
}

#[derive(Debug)]
struct SignalPair {
    s1: Value,
    s2: Value,
}

impl SignalPair {
    fn new(s1: &str, s2: &str) -> Self {
        Self {
            s1: s1.parse().unwrap(),
            s2: s2.parse().unwrap(),
        }
    }

    fn is_ordered(&self) -> bool {
        self.s1 <= self.s2
    }
}

fn main() {
    let ordered_sum: usize = include_str!("../inputs/d13")
        .split("\n\n")
        .enumerate()
        .map(|(i, l)| {
            let mut split = l.lines();
            if SignalPair::new(split.next().unwrap(), split.next().unwrap()).is_ordered() {
                println!("{} is ordered", i + 1);
                return i + 1;
            }
            println!("{} is not ordered", i + 1);
            return 0;
        })
    .sum();

    println!("ordered_sum: {}", ordered_sum)
}
