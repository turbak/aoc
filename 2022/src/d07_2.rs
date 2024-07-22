use std::collections::VecDeque;

#[derive(Debug)]
struct File {
    name: String,
    children: Vec<usize>,
    parent: usize,
    is_dir: bool,
    size: usize,
}

impl File {
    fn size(&self, all_files: &VecDeque<File>) -> usize {
        if !self.is_dir {
            return self.size;
        }

        return self
            .children
            .iter()
            .map(|c| all_files.get(*c).unwrap().size(all_files))
            .sum();
    }
}

fn main() {
    let mut split_lines = include_str!("../inputs/d07").lines().map(|l| l.split(" "));

    let mut all_files = VecDeque::<File>::new();

    all_files.push_front(File {
        name: "/".into(),
        children: Vec::new(),
        parent: 0,
        is_dir: true,
        size: 0,
    });

    let mut current_dir: usize = 0;

    split_lines.next();
    for line in split_lines {
        let mut split: VecDeque<&str> = line.collect();

        println!("handling line {:?}", split);

        if split.front().unwrap() == &"$" {
            if split.get(1).unwrap() == &"cd" {
                let name = split.pop_back().unwrap();
                if name == ".." {
                    current_dir = all_files.get(current_dir).unwrap().parent.clone()
                } else {
                    current_dir = all_files
                        .get(current_dir)
                        .unwrap()
                        .children
                        .iter()
                        .find(|idx| all_files.get(**idx).unwrap().name == name)
                        .unwrap()
                        .clone();
                }
            }
            continue;
        }

        let new_file = parse_file_from_split(&mut split, current_dir);
        let len = all_files.len();
        let f = all_files.get_mut(current_dir).unwrap();
        f.children.push(len);
        all_files.push_back(new_file);
    }

    let total_size: usize = all_files
        .iter()
        .filter_map(|f| {
            if !f.is_dir {
                return None;
            }
            let size = f.size(&all_files);

            if size < 100000 {
                return Some(size);
            };
            return None;
        })
        .sum();

    let current_unused_size = 70000000 - all_files.front().unwrap().size(&all_files);
    let size_to_free = 30000000 - current_unused_size;

    println!("sum: {}", total_size);
    println!(
        "unused_size: {}, size_to_free {}",
        current_unused_size, size_to_free
    );

    let mut files_to_free: Vec<usize> = all_files
        .iter()
        .filter(|f| f.is_dir)
        .filter_map(|f| {
            let size = f.size(&all_files);
            if size > size_to_free {
                return Some(size);
            }
            return None;
        })
        .collect();

    files_to_free.sort_by(|a, b| b.cmp(a));

    println!("min file: {}", files_to_free.pop().unwrap())
}

fn parse_file_from_split(split: &mut VecDeque<&str>, parent: usize) -> File {
    let front = split.pop_front().unwrap();
    let is_dir = front == "dir";
    let mut size = 0;
    if !is_dir {
        size = front.parse::<usize>().unwrap();
    }
    return File {
        name: split.pop_back().unwrap().into(),
        children: Vec::new(),
        parent: parent,
        is_dir: is_dir,
        size: size,
    };
}
