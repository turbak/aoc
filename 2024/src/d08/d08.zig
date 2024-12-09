const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d08";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var grid = ArrayList([]u8).init(allocator);
    defer grid.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        try grid.append(try allocator.dupe(u8, line));
    }

    var antennas_list = ArrayList(Antenna).init(allocator);
    defer antennas_list.deinit();

    for (grid.items, 0..grid.items.len) |row, row_idx| {
        for (row, 0..row.len) |item, col_idx| {
            if (item != '.') {
                try antennas_list.append(Antenna{ .freq = item, .pos = RowAndColIdx{
                    .row_idx = row_idx,
                    .col_idx = col_idx,
                } });
            }
        }
    }

    try part_1(allocator, grid.items, antennas_list.items);
    try part_2(allocator, grid.items, antennas_list.items);
}

fn part_1(allocator: std.mem.Allocator, grid: [][]u8, antennas: []Antenna) !void {
    var antinodes = ArrayList(RowAndColIdx).init(allocator);
    defer antinodes.deinit();

    for (antennas) |antenna_1| {
        for (antennas) |antenna_2| {
            if (antenna_1.pos.eql(antenna_2.pos) or antenna_1.freq != antenna_2.freq) {
                continue;
            }

            const antinode = antenna_1.antinode(antenna_2);
            if (antinode != null and is_in_bounds(antinode.?, grid.len, grid[0].len)) {
                try antinodes.append(antinode.?);
            }
        }
    }

    var total_count: usize = 0;
    for (grid, 0..grid.len) |row, row_idx| {
        for (row, 0..row.len) |item, col_idx| {
            var found = false;
            for (antinodes.items) |node| {
                if (node.col_idx == col_idx and node.row_idx == row_idx) {
                    found = true;
                    break;
                }
            }
            if (found) {
                total_count += 1;
            }
            if (found and item == '.') {
                std.debug.print("#", .{});
            } else {
                std.debug.print("{c}", .{item});
            }
        }
        std.debug.print("\n", .{});
    }

    std.debug.print("1. {d} unique locations within the bounds of the map contain an antinode.\n", .{total_count});
}

fn part_2(allocator: std.mem.Allocator, grid: [][]u8, antennas: []Antenna) !void {
    var antinodes = ArrayList(RowAndColIdx).init(allocator);
    defer antinodes.deinit();

    for (antennas) |antenna_1| {
        for (antennas) |antenna_2| {
            if (antenna_1.pos.eql(antenna_2.pos) or antenna_1.freq != antenna_2.freq) {
                continue;
            }

            const new_antinodes = try antenna_1.pos.all_antinodes_within_bounds(allocator, antenna_2.pos, grid.len, grid[0].len);
            try antinodes.appendSlice(new_antinodes);
        }
    }

    var total_count: usize = 0;
    for (grid, 0..grid.len) |row, row_idx| {
        for (row, 0..row.len) |item, col_idx| {
            var found = false;
            for (antinodes.items) |node| {
                if (node.col_idx == col_idx and node.row_idx == row_idx) {
                    found = true;
                    break;
                }
            }
            if (found) {
                total_count += 1;
            }
            if (found and item == '.') {
                std.debug.print("#", .{});
            } else {
                std.debug.print("{c}", .{item});
            }
        }
        std.debug.print("\n", .{});
    }

    std.debug.print("1. {d} unique locations within the bounds of the map contain an antinode.\n", .{total_count});
}

fn is_in_bounds(node: RowAndColIdx, max_row: usize, max_col: usize) bool {
    return node.col_idx < max_col and node.row_idx < max_row;
}

fn is_in_bounds_raw(row_idx: isize, col_idx: isize, max_row: usize, max_col: usize) bool {
    return (row_idx >= 0 and col_idx >= 0 and col_idx < max_col and row_idx < max_row);
}

const EMPTY = '.';

const Antenna = struct {
    freq: u8,
    pos: RowAndColIdx,

    fn antinode(self: Antenna, other: Antenna) ?RowAndColIdx {
        return self.pos.antinode(other.pos);
    }
};

const AntennaPair = struct { a: Antenna, b: Antenna };

const RowAndColIdx = struct {
    row_idx: usize,
    col_idx: usize,

    fn eql(self: RowAndColIdx, other: RowAndColIdx) bool {
        return self.row_idx == other.row_idx and self.col_idx == other.col_idx;
    }

    fn antinode(self: RowAndColIdx, other: RowAndColIdx) ?RowAndColIdx {
        const row_cast: isize = @intCast(self.row_idx);
        const col_cast: isize = @intCast(self.col_idx);

        const other_row_cast: isize = @intCast(other.row_idx);
        const other_col_cast: isize = @intCast(other.col_idx);

        const row_delta: isize = row_cast - other_row_cast;
        const col_delta: isize = col_cast - other_col_cast;

        const new_row = row_cast + row_delta;
        const new_col = col_cast + col_delta;
        if (row_delta > self.row_idx or
            col_delta > self.col_idx or
            new_col < 0 or
            new_row < 0)
        {
            return null;
        }

        return RowAndColIdx{
            .row_idx = @intCast(new_row),
            .col_idx = @intCast(new_col),
        };
    }

    fn all_antinodes_within_bounds(self: RowAndColIdx, allocator: std.mem.Allocator, other: RowAndColIdx, max_row: usize, max_col: usize) ![]RowAndColIdx {
        const row_cast: isize = @intCast(self.row_idx);
        const col_cast: isize = @intCast(self.col_idx);

        const other_row_cast: isize = @intCast(other.row_idx);
        const other_col_cast: isize = @intCast(other.col_idx);

        const row_delta: isize = row_cast - other_row_cast;
        const col_delta: isize = col_cast - other_col_cast;

        var new_row = row_cast + row_delta;
        var new_col = col_cast + col_delta;
        var antinodes = ArrayList(RowAndColIdx).init(allocator);
        while (is_in_bounds_raw(new_row, new_col, max_row, max_col)) {
            try antinodes.append(RowAndColIdx{
                .row_idx = @intCast(new_row),
                .col_idx = @intCast(new_col),
            });

            new_col += col_delta;
            new_row += row_delta;
        }

        try antinodes.appendSlice(&[_]RowAndColIdx{ self, other });

        return antinodes.toOwnedSlice();
    }
};
