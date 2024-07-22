const GRID_SIZE: usize = 99;

fn main() {
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let lines = include_str!("../inputs/d08").lines().map(|l| l.chars());

    for (i, l) in lines.enumerate() {
        for (j, c) in l.enumerate() {
            grid[i][j] = c.to_string().parse().unwrap()
        }
    }

    let mut tree_count: usize = (GRID_SIZE - 1) * 4;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if check_all_directions(grid, i, j) {
                tree_count += 1;
            }
        }
    }

    //(1, 1), (1, 2),
    println!("visible tree count: {}", tree_count);
}

fn check_all_directions(grid: [[u8; GRID_SIZE]; GRID_SIZE], mid_i: usize, mid_j: usize) -> bool {
    return check_left(grid, mid_i, mid_j)
        || check_right(grid, mid_i, mid_j)
        || check_bottom(grid, mid_i, mid_j)
        || check_top(grid, mid_i, mid_j);
}

fn check_right(grid: [[u8; GRID_SIZE]; GRID_SIZE], mid_i: usize, mid_j: usize) -> bool {
    let mut max: u8 = 0;
    for j in mid_j + 1..grid[mid_i].len() {
        max = max.max(grid[mid_i][j]);
    }

    return max < grid[mid_i][mid_j];
}

fn check_left(grid: [[u8; GRID_SIZE]; GRID_SIZE], mid_i: usize, mid_j: usize) -> bool {
    let mut max: u8 = 0;
    for j in 0..mid_j {
        max = max.max(grid[mid_i][j]);
    }

    return max < grid[mid_i][mid_j];
}

fn check_top(grid: [[u8; GRID_SIZE]; GRID_SIZE], mid_i: usize, mid_j: usize) -> bool {
    let mut max: u8 = 0;
    for i in 0..mid_i {
        max = max.max(grid[i][mid_j]);
    }

    return max < grid[mid_i][mid_j];
}

fn check_bottom(grid: [[u8; GRID_SIZE]; GRID_SIZE], mid_i: usize, mid_j: usize) -> bool {
    let mut max: u8 = 0;
    for i in mid_i + 1..grid[mid_i].len() {
        max = max.max(grid[i][mid_j]);
    }

    return max < grid[mid_i][mid_j];
}
