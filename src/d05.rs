#![feature(drain_filter)]

fn main() {
    let file_str = include_str!("../inputs/d05");

    let mut split_input = file_str.split("\n\n");

    let mut arrangement = parse_arrangement(split_input.next().unwrap());
    println!("parsed input {:#?}", arrangement);
    let moves = parse_moves(split_input.next().unwrap());

    for m in moves {
        let final_len = arrangement.get(m.from - 1).unwrap().len().saturating_sub(m.count);
        let mut elems = arrangement.get_mut(m.from - 1).unwrap().split_off(final_len);
        arrangement.get_mut(m.to - 1).unwrap().append(&mut elems);
    }

    let res: String = arrangement.iter_mut().map(|item| item.pop()).filter(|opt| opt.is_some()).flatten().collect();
    println!("total_combination: {}", res)
}

fn parse_arrangement(arrangement_input: &str) -> Vec<Vec<char>> {
    let mut res = Vec::<Vec<char>>::new();
    let lines: Vec<Vec<char>> = arrangement_input
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    for _ in 0..lines.get(0).unwrap().len() {
        res.push(Vec::new())
    }

    for l in lines.iter().rev().skip(1) {
        for (j, c) in l.iter().enumerate() {
            res.get_mut(j).unwrap().push(*c);
        }
    }

    res.drain_filter(|v| v.contains(&&'[') || v.contains(&&']'));

    res.iter_mut().for_each(|item| {
        item.drain_filter(|c| c == &' ');
    });
    return res.drain_filter(|item| !item.is_empty()).collect();
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_moves(moves_str: &str) -> Vec<Move> {
    return moves_str
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            Move {
                count: split.nth(1).unwrap().parse::<usize>().unwrap(),
                from: split.nth(1).unwrap().parse::<usize>().unwrap(),
                to: split.nth(1).unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect();
}
