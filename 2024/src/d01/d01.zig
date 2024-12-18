const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.ArrayHashMap;

const PATH = "inputs/d01";
const allocator = std.heap.page_allocator;

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var left = ArrayList(i32).init(allocator);
    defer left.deinit();
    var right = ArrayList(i32).init(allocator);
    defer right.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var token_iter = std.mem.tokenizeScalar(u8, line, ' ');
        const left_num = try std.fmt.parseInt(i32, token_iter.next().?, 10);
        const right_num = try std.fmt.parseInt(i32, token_iter.next().?, 10);
        try left.append(left_num);
        try right.append(right_num);
    }

    std.mem.sort(i32, left.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right.items, {}, comptime std.sort.asc(i32));

    part_1(left.items, right.items);
    part_2(left.items, right.items);
}

fn part_1(left: []i32, right: []i32) void {
    var res: u32 = 0;
    for (left, 0..) |left_num, i| {
        const distance = @abs(left_num - right[i]);
        res += distance;
    }

    std.debug.print("1. resulting distance is {d}\n", .{res});
}

fn part_2(left: []i32, right: []i32) void {
    var res: isize = 0;
    var right_idx: usize = 0;

    for (left) |left_num| {
        var num_occurances: isize = 0;

        while (right_idx < right.len and right[right_idx] < left_num) {
            right_idx += 1;
        }

        var tmp_idx = right_idx;
        while (tmp_idx < right.len and right[tmp_idx] == left_num) {
            num_occurances += 1;
            tmp_idx += 1;
        }

        res += left_num * num_occurances;
    }

    std.debug.print("2. resulting similarity score is {d}\n", .{res});
}
