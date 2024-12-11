const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d11";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var nums = ArrayList(usize).init(allocator);
    defer nums.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.tokenizeScalar(u8, line, ' ');

        while (iter.next()) |item| {
            try nums.append(try std.fmt.parseInt(usize, item, 10));
        }
    }

    try part_1(allocator, nums.items);
    try part_2(allocator, nums.items);
}

fn part_1(allocator: std.mem.Allocator, init_nums: []usize) !void {
    var nums = try ArrayList(usize).initCapacity(allocator, init_nums.len);
    try nums.appendSlice(init_nums);
    defer nums.deinit();

    var next_nums = ArrayList(usize).init(allocator);
    defer next_nums.deinit();

    for (0..25) |_| {
        next_nums.clearRetainingCapacity();

        for (nums.items) |num| {
            inline for ([_]Rule{ .Replace, .Split, .Mult }) |rule| {
                if (rule.apply(num)) |res| {
                    switch (res) {
                        .one => {
                            try next_nums.append(res.one);
                        },
                        .two => {
                            try next_nums.append(res.two[0]);
                            try next_nums.append(res.two[1]);
                        },
                    }
                    break;
                }
            }
        }

        std.mem.swap(ArrayList(usize), &nums, &next_nums);
    }

    std.debug.print("1. {d} stones after blinking 25 times.\n", .{nums.items.len});
}

fn part_2(allocator: std.mem.Allocator, init_nums: []usize) !void {
    var num_counts = std.AutoHashMap(usize, usize).init(allocator);
    defer num_counts.deinit();

    for (init_nums) |num| {
        const count = num_counts.get(num) orelse 0;
        try num_counts.put(num, count + 1);
    }

    try num_counts.ensureTotalCapacity(3000);

    var next_counts = std.AutoHashMap(usize, usize).init(allocator);
    defer next_counts.deinit();

    const rounds: usize = 75;
    for (0..rounds) |_| {
        next_counts.clearRetainingCapacity();

        var iter = num_counts.iterator();
        while (iter.next()) |entry| {
            const num = entry.key_ptr.*;
            const count = entry.value_ptr.*;

            inline for (.{ Rule.Replace, Rule.Split, Rule.Mult }) |rule| {
                if (rule.apply(num)) |result| {
                    switch (result) {
                        .one => {
                            const new_count = (next_counts.get(result.one) orelse 0) + count;
                            try next_counts.put(result.one, new_count);
                        },
                        .two => {
                            inline for ([_]usize{ result.two[0], result.two[1] }) |new_num| {
                                const new_count = (next_counts.get(new_num) orelse 0) + count;
                                try next_counts.put(new_num, new_count);
                            }
                        },
                    }
                    break;
                }
            }
        }

        std.mem.swap(std.AutoHashMap(usize, usize), &num_counts, &next_counts);
    }

    var total: usize = 0;
    var iter = num_counts.valueIterator();
    while (iter.next()) |count| {
        total += count.*;
    }

    std.debug.print("2. {d} stones after blinking 75 times.\n", .{total});
}

const Rule = enum {
    Replace,
    Split,
    Mult,

    fn apply(self: Rule, num: usize) ?OneOrTwo {
        return switch (self) {
            .Replace => {
                if (num == 0) {
                    return OneOrTwo{ .one = 1 };
                }
                return null;
            },
            .Split => {
                const digit_count = count_digits(num);
                if (digit_count % 2 == 0) {
                    const mid = digit_count / 2;
                    const left = num / std.math.pow(usize, 10, mid);
                    const right = num % std.math.pow(usize, 10, mid);
                    return OneOrTwo{ .two = .{ left, right } };
                }
                return null;
            },
            .Mult => OneOrTwo{ .one = num * 2024 },
        };
    }

    fn count_digits(num: usize) usize {
        var num_digits: usize = 0;
        var num_mod = num;
        while (num_mod > 0) {
            num_mod = @divFloor(num_mod, 10);
            num_digits += 1;
        }
        return num_digits;
    }
};

const OneOrTwoEnum = enum { one, two };

const OneOrTwo = union(OneOrTwoEnum) { one: usize, two: std.meta.Tuple(&.{ usize, usize }) };
