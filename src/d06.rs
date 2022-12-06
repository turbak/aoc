use std::collections::HashSet;



fn main() {
    let file_str = include_str!("../input_d06.txt");

    let mut count = 0;
    let mut char_set = HashSet::<char>::new();
    for c in file_str.chars() {
        if char_set.get(&c).is_some() {
            char_set.clear();
        }

        char_set.insert(c);

        count += 1;
        if char_set.len() == 4 {
            break;
        }
    }

    print!("count: {}", count);
}