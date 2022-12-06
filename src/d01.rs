
fn main() {
    let mut all_elves: Vec<i32> = include_str!("../inputs/d01").trim()
    .split("\n\n")
    .map(|item| item.parse::<i32>().unwrap())
    .collect();

    all_elves.sort();
    println!("maximum_calories: {}", all_elves.pop().unwrap()+all_elves.pop().unwrap()+all_elves.pop().unwrap())
}