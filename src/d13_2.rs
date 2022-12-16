#![feature(slice_take)]

use std::{str::FromStr, fmt::Display, collections::BinaryHeap};

#[derive(Debug)]
enum Value {
    Integer(usize),
    List(Vec<Box<Value>>)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(x) => write!(f, "{}", x),
            Value::List(items) => {
                write!(f, "[");
                for (i, item) in items.iter().enumerate() {
                    item.fmt(f);
                    if i < items.len() - 1 {
                        write!(f, ",");
                    }
                }
                write!(f, "]")
            },
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
                Value::List(_) => Value::List(vec!(Box::new(Value::Integer(*x)))).cmp(other),
            }
            Value::List(x) => match other {
                Value::Integer(y) => self.cmp(&Value::List(vec!(Box::new(Value::Integer(*y))))),
                Value::List(y) => {
                    for (i, item_x) in x.iter().enumerate() {
                        if let Some(item_y) = y.get(i) {
                            if item_x > item_y {
                                //println!("{} > {}", self, other);
                                return std::cmp::Ordering::Greater;
                            }
                            if item_x < item_y {
                                return std::cmp::Ordering::Less;
                            }
                        } else {
                            //println!("{} > {}", self, other);
                            return std::cmp::Ordering::Greater; //right size runs out first
                        }
                    }

                    if x.len() == y.len() {
                        //println!("{} == {}", self, other);
                        return std::cmp::Ordering::Equal;
                    }
                    //println!("{} < {}", self, other);
                    return std::cmp::Ordering::Less;
                },
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
        let mut current_num = String::new();
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

fn main() {
    let mut ordered_signals: BinaryHeap<std::cmp::Reverse<Value>> = include_str!("../inputs/d13")
    .trim()
    .split("\n\n")
    .map(|l| l.split("\n"))
    .flatten()
    .map(|l| std::cmp::Reverse(l.parse().unwrap()))
    .collect();

    let first = "[[2]]";
    let second = "[[6]]";
    ordered_signals.push(std::cmp::Reverse(first.parse().unwrap()));
    ordered_signals.push(std::cmp::Reverse(second.parse().unwrap()));

    let mut first_idx = 0;
    let mut second_idx = 0;
    let mut idx = 0;
    while first_idx == 0 || second_idx == 0 {
        idx += 1;
        let found_str = ordered_signals.pop().unwrap().0.to_string();
        if found_str == first {
            first_idx = idx
        }
        if found_str == second {
            second_idx = idx
        }
    }
    println!("decoder_key: {}", first_idx*second_idx)
}