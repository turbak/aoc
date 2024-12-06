const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d06";

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

    var guard_col_idx: usize = 0;
    var guard_row_idx: usize = 0;
    for (grid.items, 0..grid.items.len) |row, row_idx| {
        for (row, 0..row.len) |object, col_idx| {
            if (object == GUARD) {
                guard_col_idx = col_idx;
                guard_row_idx = row_idx;
                grid.items[row_idx][col_idx] = '.';
                break;
            }
        }
    }

    const clone_1 = try cloneGrid(allocator, grid.items);
    defer allocator.free(clone_1);
    part_1(clone_1, RowAndColIdx{ .row_idx = guard_row_idx, .col_idx = guard_col_idx });

    const clone_2 = try cloneGrid(allocator, grid.items);
    defer allocator.free(clone_2);
    try part_2(allocator, clone_2, RowAndColIdx{ .row_idx = guard_row_idx, .col_idx = guard_col_idx });
}

fn cloneGrid(allocator: std.mem.Allocator, grid: [][]u8) ![][]u8 {
    var clone = try allocator.dupe([]u8, grid);
    for (0..clone.len) |clone_row_idx| {
        clone[clone_row_idx] = try allocator.dupe(u8, grid[clone_row_idx]);
    }

    return clone;
}

const GUARD = '^';
const OBSTACLE = '#';
const VISITED_MARK = 'X';
const UNVISITED_MARK = '.';

fn part_1(map: [][]u8, guard_pos: RowAndColIdx) void {
    var total_visited_cells: usize = 0;

    var iter = GridIter{ .grid = map, .pos = guard_pos, .dir = Direction.Up };
    while (iter.next()) |row_and_col| {
        if (map[row_and_col.row_idx][row_and_col.col_idx] == UNVISITED_MARK) {
            map[row_and_col.row_idx][row_and_col.col_idx] = VISITED_MARK;
            total_visited_cells += 1;
        }
    }

    std.debug.print("1. Guard will visit {d} distinct positions.\n", .{total_visited_cells});
}

fn part_2(allocator: std.mem.Allocator, map: [][]u8, guard_pos: RowAndColIdx) !void {
    var total_blocks_placed: usize = 0;
    for (map, 0..map.len) |row, row_idx| {
        for (row, 0..row.len) |elem, col_idx| {
            if (elem != UNVISITED_MARK) {
                continue;
            }
            map[row_idx][col_idx] = OBSTACLE;
            if (try isCyclic(allocator, map, guard_pos)) {
                total_blocks_placed += 1;
            }
            map[row_idx][col_idx] = UNVISITED_MARK;
        }
    }

    std.debug.print("2. {d} different positions could you choose for this obstruction.\n", .{total_blocks_placed});
}

fn isCyclic(allocator: std.mem.Allocator, map: [][]u8, guard_pos: RowAndColIdx) !bool {
    var visited = std.AutoHashMap(PosKey, bool).init(allocator);
    defer visited.deinit();

    var iter = GridIter{ .grid = map, .pos = guard_pos, .dir = Direction.Up };
    while (iter.next()) |row_and_col| {
        const key = PosKey{ .pos = row_and_col, .dir = iter.dir };
        if (visited.get(key) == null) {
            if (try isCyclicUntil(key, null, &visited, &iter)) {
                return true;
            }
        }
    }
    return false;
}

fn isCyclicUntil(pos: PosKey, prev_pos: ?PosKey, visited: *std.AutoHashMap(PosKey, bool), iter: *GridIter) !bool {
    try visited.put(pos, true);

    while (iter.next()) |row_and_col| {
        const pos_key = PosKey{ .pos = row_and_col, .dir = iter.dir };
        if (visited.get(pos_key) == null) {
            if (try isCyclicUntil(pos_key, pos, visited, iter)) {
                return true;
            }
        } else if (prev_pos == null or !prev_pos.?.eql(pos_key)) {
            return true;
        }
    }
    return false;
}

const RowAndColIdx = struct {
    row_idx: usize,
    col_idx: usize,
};

const Direction = enum {
    Up,
    Down,
    Left,
    Right,

    fn getColAndRowDelta(self: Direction) ColAndRowDelta {
        switch (self) {
            .Up => return ColAndRowDelta{ .row_delta = -1, .col_delta = 0 },
            .Down => return ColAndRowDelta{ .row_delta = 1, .col_delta = 0 },
            .Left => return ColAndRowDelta{ .row_delta = 0, .col_delta = -1 },
            .Right => return ColAndRowDelta{ .row_delta = 0, .col_delta = 1 },
        }
    }

    fn turn_right(self: Direction) Direction {
        switch (self) {
            .Up => return .Right,
            .Down => return .Left,
            .Left => return .Up,
            .Right => return .Down,
        }
    }
};

const ColAndRowDelta = struct {
    col_delta: isize,
    row_delta: isize,
};

const PosKey = struct {
    pos: RowAndColIdx,
    dir: Direction,

    fn eql(self: PosKey, other: PosKey) bool {
        return self.dir == other.dir and
            self.pos.col_idx == other.pos.col_idx and
            self.pos.row_idx == other.pos.row_idx;
    }
};

const GridIter = struct {
    grid: [][]u8,
    pos: RowAndColIdx,
    dir: Direction,

    pub fn next(self: *GridIter) ?RowAndColIdx {
        var delta = self.dir.getColAndRowDelta();

        var tmp_row_idx: isize = @intCast(self.pos.row_idx);
        tmp_row_idx += delta.row_delta;
        var tmp_col_idx: isize = @intCast(self.pos.col_idx);
        tmp_col_idx += delta.col_delta;

        if (tmp_col_idx >= self.grid.len or tmp_row_idx >= self.grid.len or tmp_col_idx < 0 or tmp_row_idx < 0) {
            return null;
        }

        while (self.grid[@intCast(tmp_row_idx)][@intCast(tmp_col_idx)] == OBSTACLE) {
            self.dir = self.dir.turn_right();
            delta = self.dir.getColAndRowDelta();

            tmp_row_idx = @intCast(self.pos.row_idx);
            tmp_row_idx += delta.row_delta;
            tmp_col_idx = @intCast(self.pos.col_idx);
            tmp_col_idx += delta.col_delta;

            if (tmp_col_idx >= self.grid.len or tmp_row_idx >= self.grid.len or tmp_col_idx < 0 or tmp_row_idx < 0) {
                return null;
            }
        }

        self.pos = RowAndColIdx{ .col_idx = @intCast(tmp_col_idx), .row_idx = @intCast(tmp_row_idx) };
        return self.pos;
    }
};
