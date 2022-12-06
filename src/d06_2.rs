use std::collections::{HashSet, VecDeque};

fn main() {
    let file_str = include_str!("../inputs/d06");

    println!("found_subsequence index: {} ", calculate_start_of_the_message_marker(file_str, 14));
}

fn calculate_start_of_the_message_marker(file_str: &str, num_of_chars: usize) -> i32 {
    let mut count = 0;
    let mut char_set = HashSet::<char>::new();
    let mut subseq = VecDeque::<char>::new();
    for c in file_str.chars() {
        count += 1;
        if char_set.contains(&c) {
            while let Some(ch) = subseq.pop_front() {
                char_set.remove(&ch);

                if ch == c {
                    break;
                }
            }
        }

        char_set.insert(c);
        subseq.push_back(c);

        if subseq.len() == num_of_chars {
            break;
        }
    }

    return count;
}