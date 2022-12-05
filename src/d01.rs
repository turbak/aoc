use std::fs;

fn main() {
    let file_path = std::env::args().nth(1).expect("no filepath given");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let split_str = contents.split("\n\n");

    let mut all_elves = Vec::<i32>::new();
    for all_items_of_elf in split_str {

        let mut elfs_calories = 0;
        for item in all_items_of_elf.split("\n") {
            if item.is_empty() {
                break;
            }
            
            elfs_calories += item.parse::<i32>().unwrap()
        }
        all_elves.push(elfs_calories);
    }

    all_elves.sort();
    println!("maximum_calories: {}", all_elves.pop().unwrap()+all_elves.pop().unwrap()+all_elves.pop().unwrap())
}