struct Pair {
    first: (i32, i32),
    second: (i32, i32),
}

impl Pair {
    fn is_full_overlap(&self) -> bool {
        for i in self.first.0..self.first.1 + 1 {
            for j in self.second.0..self.second.1 + 1 {
                if i == j {
                    println!(
                        "overlap found: {} between {} and {}",
                        i, self.second.0, self.second.1
                    );
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let overlaps_count: usize = include_str!("../inputs/d04")
        .lines()
        .map(|s| {
            let mut split = s.split(",");
            let mut first = split
                .next()
                .unwrap()
                .split("-")
                .map(|item| item.parse::<i32>().unwrap());
            let mut second = split
                .next()
                .unwrap()
                .split("-")
                .map(|item| item.parse::<i32>().unwrap());
            Pair {
                first: (first.next().unwrap(), first.next().unwrap()),
                second: (second.next().unwrap(), second.next().unwrap()),
            }
        })
        .filter(|pair| pair.is_full_overlap())
        .count();

    println!("overlaps_count: {}", overlaps_count);
}
