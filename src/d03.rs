#![feature(iter_next_chunk)]

use std::{fs,collections::{HashMap, HashSet}};

fn main() {
    let char_scores: HashMap<char, u32> = {
        let mut m = HashMap::new();
        let mut start_count: u32 = 1;
        for c in b'a'..=b'z' {
            m.insert(c as char, start_count);
            start_count += 1;
        }

        for c in b'A'..=b'Z' {
            m.insert(c as char, start_count);
            start_count += 1;
        }

        m
    };

    let file_path = std::env::args().nth(1).expect("no filepath given");
    let str_file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lines = str_file.lines();
    let mut total_count: u32 = 0;
    while let Ok(chunk) = lines.next_chunk::<3>() {
        let mut sets: Vec<HashSet<char>> = Vec::with_capacity(3);
        for s in chunk {
            sets.push(s.chars().collect())
        }

        let found_char = sets.get(0).unwrap().into_iter().find(|c| {
            sets.get(1).unwrap().contains(c) && sets.get(2).unwrap().contains(c)
        }).unwrap();

        total_count += char_scores.get(found_char).unwrap();
        println!("current_score: {}, found_char: {}, chars: {:?}", total_count, found_char, chunk)
    }
    

    println!("total_score: {}", total_count);
}