#![feature(slice_take)]

use std::str::FromStr;

#[derive(Debug)]
enum Value {
    Integer(usize),
    List(Vec<Box<Value>>)
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
    s2: Value
}

impl SignalPair {
    fn new(s1: &str, s2: &str) -> Self {
        Self { s1: s1.parse().unwrap(), s2: s2.parse().unwrap() }
    }
}

fn main() {
    let mut pairs: Vec<SignalPair> = include_str!("../inputs/d13_test")
    .split("\n\n")
    .map(|l| {
        let mut split = l.lines();
        SignalPair::new(split.next().unwrap(), split.next().unwrap())
    })
    .collect();

    println!("{:#?}", pairs)
}