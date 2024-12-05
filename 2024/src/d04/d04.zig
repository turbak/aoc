const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d04";
const allocator = std.heap.page_allocator;

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var grid = ArrayList([]u8).init(allocator);
    defer grid.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        try grid.append(try allocator.dupe(u8, line));
    }

    try part_1(grid.items);
    try part_2(grid.items);
}

fn part_1(grid: [][]u8) !void {
    const ref_grid = try cloneGrid(grid);
    std.debug.print("1. XMAS appears {d} times.\n", .{countAllHorizontal(grid, ref_grid) + countAllVertical(grid, ref_grid) + countAllDiagonal(grid, ref_grid)});
    //printGridFromRefGrid(grid, ref_grid);
}

fn part_2(grid: [][]u8) !void {
    const ref_grid = try cloneGrid(grid);

    var total_count: usize = 0;
    var box: [3][3]u8 = undefined;
    for (0..grid.len - 2) |row_idx| {
        for (0..grid[row_idx].len - 2) |col_idx| {
            for (0..3) |box_row_idx| {
                for (0..3) |box_col_idx| {
                    box[box_row_idx][box_col_idx] = grid[row_idx + box_row_idx][col_idx + box_col_idx];
                }
            }

            if (boxEqlXMAS(box)) {
                total_count += 1;

                ref_grid[row_idx][col_idx] = true;
                ref_grid[row_idx][col_idx + 2] = true;
                ref_grid[row_idx + 1][col_idx + 1] = true;
                ref_grid[row_idx + 2][col_idx] = true;
                ref_grid[row_idx + 2][col_idx + 2] = true;
            }
        }
    }

    std.debug.print("2. X-MAS appears {d} times.\n", .{total_count});
    //printGridFromRefGrid(grid, ref_grid);
}

fn boxEqlXMAS(box: [3][3]u8) bool {
    const diag1 = &[_]u8{ box[0][0], box[1][1], box[2][2] };
    const diag2 = &[_]u8{ box[0][2], box[1][1], box[2][0] };
    return (std.mem.eql(u8, diag1, "SAM") or std.mem.eql(u8, diag1, "MAS")) and
        (std.mem.eql(u8, diag2, "SAM") or std.mem.eql(u8, diag2, "MAS"));
}

fn printGridFromRefGrid(grid: [][]u8, ref_grid: [][]bool) void {
    for (0..grid.len) |col_idx| {
        for (0..grid[col_idx].len) |row_idx| {
            if (ref_grid[col_idx][row_idx]) {
                std.debug.print("{c}", .{grid[col_idx][row_idx]});
            } else {
                std.debug.print(".", .{});
            }
        }
        std.debug.print("\n", .{});
    }
    std.debug.print("\n", .{});
}

fn cloneGrid(grid: [][]u8) ![][]bool {
    var cloned = try allocator.alloc([]bool, grid.len);
    for (0..cloned.len) |col_idx| {
        cloned[col_idx] = try allocator.alloc(bool, grid[col_idx].len);
        for (0..cloned[col_idx].len) |cloned_idx| {
            cloned[col_idx][cloned_idx] = false;
        }
    }

    return cloned;
}

fn countAllHorizontal(grid: [][]u8, ref_grid: [][]bool) usize {
    var total_count: usize = 0;
    for (0..grid.len) |row_idx| {
        const row = grid[row_idx];
        var col_idx: usize = 0;
        while (col_idx < row.len - NEEDLE.len + 1) {
            if (sliceEqlNeedle(row[col_idx .. col_idx + NEEDLE.len])) {
                for (col_idx..col_idx + NEEDLE.len) |copy_idx| {
                    ref_grid[row_idx][copy_idx] = true;
                }
                total_count += 1;
            }
            col_idx += 1;
        }
    }
    return total_count;
}

fn countAllVertical(grid: [][]u8, ref_grid: [][]bool) usize {
    var total_count: usize = 0;
    var buf: [4]u8 = undefined;
    for (0..grid[0].len) |col_idx| {
        var row_idx: usize = 0;
        while (row_idx < grid.len - buf.len + 1) {
            for (0..buf.len) |copy_idx| {
                buf[copy_idx] = grid[row_idx + copy_idx][col_idx];
            }
            if (sliceEqlNeedle(&buf)) {
                for (0..buf.len) |copy_idx| {
                    ref_grid[row_idx + copy_idx][col_idx] = true;
                }
                total_count += 1;
            }
            row_idx += 1;
        }
    }

    return total_count;
}

fn countAllDiagonal(grid: [][]u8, ref_grid: [][]bool) usize {
    var total_count: usize = 0;
    var buf: [4]u8 = undefined;
    for (0..grid.len - buf.len + 1) |col_idx| {
        for (0..grid[col_idx].len - buf.len + 1) |row_idx| {
            var diag_col_idx = col_idx;
            var diag_row_idx = row_idx;
            for (0..buf.len) |copy_idx| {
                buf[copy_idx] = grid[diag_col_idx][diag_row_idx];
                diag_col_idx += 1;
                diag_row_idx += 1;
            }
            if (sliceEqlNeedle(&buf)) {
                diag_col_idx = col_idx;
                diag_row_idx = row_idx;
                for (0..buf.len) |_| {
                    ref_grid[diag_col_idx][diag_row_idx] = true;
                    diag_col_idx += 1;
                    diag_row_idx += 1;
                }
                total_count += 1;
            }
        }
    }

    for (0..grid.len - buf.len + 1) |col_idx| {
        for (buf.len..grid[col_idx].len + 1) |row_idx| {
            var diag_col_idx = col_idx;
            var diag_row_idx = row_idx;
            for (0..buf.len) |copy_idx| {
                diag_row_idx -= 1;
                buf[copy_idx] = grid[diag_col_idx][diag_row_idx];
                diag_col_idx += 1;
            }
            if (sliceEqlNeedle(&buf)) {
                diag_col_idx = col_idx;
                diag_row_idx = row_idx;
                for (0..buf.len) |_| {
                    diag_row_idx -= 1;
                    ref_grid[diag_col_idx][diag_row_idx] = true;
                    diag_col_idx += 1;
                }
                total_count += 1;
            }
        }
    }

    return total_count;
}

const NEEDLE = "XMAS";
const NEEDLE_REV = "SAMX";

fn sliceEqlNeedle(sl: []u8) bool {
    return std.mem.eql(u8, sl, NEEDLE) or std.mem.eql(u8, sl, NEEDLE_REV);
}
