const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d10";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var grid = ArrayList([]usize).init(allocator);
    defer grid.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var row = try ArrayList(usize).initCapacity(allocator, line.len);
        for (line) |char| {
            row.appendAssumeCapacity(char - '0');
        }

        try grid.append(try row.toOwnedSlice());
    }

    try part_1(allocator, grid.items);
    try part_2(allocator, grid.items);
}

fn part_1(allocator: std.mem.Allocator, map: [][]usize) !void {
    var total: usize = 0;
    for (0..map.len, map) |x, row| {
        for (0..row.len, row) |y, item| {
            if (item == 0) {
                total += try dfs(allocator, Map{ .map = map }, XandY{ .x = x, .y = y }, true);
            }
        }
    }
    std.debug.print("1. Sum of the scores of all trailheads is {d}\n", .{total});
}

fn part_2(allocator: std.mem.Allocator, map: [][]usize) !void {
    var total: usize = 0;
    for (0..map.len, map) |x, row| {
        for (0..row.len, row) |y, item| {
            if (item == 0) {
                total += try dfs(allocator, Map{ .map = map }, XandY{ .x = x, .y = y }, false);
            }
        }
    }
    std.debug.print("2. Sum of the scores of all trailheads is {d}\n", .{total});
}

fn dfs(allocator: std.mem.Allocator, map: Map, start: XandY, unique: bool) !usize {
    var total_paths: usize = 0;
    var queue = ArrayList(XandY).init(allocator);
    defer queue.deinit();

    var visited = std.AutoArrayHashMap(XandY, bool).init(allocator);
    defer visited.deinit();

    const start_neighbors = try map.neighbors(allocator, start);
    defer allocator.free(start_neighbors);
    try queue.appendSlice(start_neighbors);

    try visited.put(start, true);
    while (queue.popOrNull()) |current| {
        if (unique and visited.get(current) != null) {
            continue;
        }

        try visited.put(current, true);

        if (map.get(current) == 9) {
            total_paths += 1;
        }

        const neighbors = try map.neighbors(allocator, current);
        defer allocator.free(neighbors);

        try queue.appendSlice(neighbors);
    }
    return total_paths;
}

const XandY = struct {
    x: usize,
    y: usize,
};

const Map = struct {
    map: [][]usize,

    fn neighbors(self: Map, allocator: std.mem.Allocator, x_and_y: XandY) ![]XandY {
        var neighbor_list = std.ArrayList(XandY).init(allocator);
        defer neighbor_list.deinit();

        const current = self.get(x_and_y);
        if (x_and_y.x > 0) {
            const new = XandY{
                .x = x_and_y.x - 1,
                .y = x_and_y.y,
            };
            const new_val = self.get(new);
            if (current < new_val and new_val - current == 1) {
                try neighbor_list.append(new);
            }
        }

        if (x_and_y.y > 0) {
            const new = XandY{ .x = x_and_y.x, .y = x_and_y.y - 1 };
            const new_val = self.get(new);
            if (current < new_val and new_val - current == 1) {
                try neighbor_list.append(new);
            }
        }

        if (x_and_y.x < self.map.len - 1) {
            const new = XandY{ .x = x_and_y.x + 1, .y = x_and_y.y };
            const new_val = self.get(new);
            if (current < new_val and new_val - current == 1) {
                try neighbor_list.append(new);
            }
        }

        if (x_and_y.y < self.map[0].len - 1) {
            const new = XandY{ .x = x_and_y.x, .y = x_and_y.y + 1 };
            const new_val = self.get(new);
            if (current < new_val and new_val - current == 1) {
                try neighbor_list.append(new);
            }
        }
        return neighbor_list.toOwnedSlice();
    }

    fn get(self: Map, x_and_y: XandY) usize {
        return self.map[x_and_y.x][x_and_y.y];
    }
};
