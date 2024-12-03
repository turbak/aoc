const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d03";
const allocator = std.heap.page_allocator;

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    const input = try file.readToEndAlloc(allocator, 1 * 1024 * 1024);
    defer allocator.free(input);

    var mul_iter = std.mem.tokenizeSequence(u8, input, "mul(");

    var operations = ArrayList(Operation).init(allocator);
    defer operations.deinit();

    while (mul_iter.next()) |token| {
        const inner_mult: []const u8 = std.mem.sliceTo(token, ')');

        var split = std.mem.tokenizeScalar(u8, inner_mult, ',');
        const left = split.next();
        const right = split.next();
        if (left == null or right == null or split.next() != null) {
            continue;
        }

        const a: ?isize = std.fmt.parseInt(isize, left.?, 10) catch null;
        const b: ?isize = std.fmt.parseInt(isize, right.?, 10) catch null;

        if (a != null and b != null) {
            try operations.append(Operation{ .Mult = MultOperation{
                .a = a.?,
                .b = b.?,
            } });
        }

        var do_indexes = ArrayList(usize).init(allocator);
        defer do_indexes.deinit();
        var dont_indexes = ArrayList(usize).init(allocator);
        defer dont_indexes.deinit();

        var prev_idx: usize = 0;
        while (std.mem.indexOfPos(u8, token, prev_idx, "do()")) |idx| {
            try do_indexes.append(idx);
            prev_idx = idx + 1;
        }

        prev_idx = 0;
        while (std.mem.indexOfPos(u8, token, prev_idx, "don't()")) |idx| {
            try dont_indexes.append(idx);
            prev_idx = idx + 1;
        }

        var do_idx: usize = 0;
        var dont_idx: usize = 0;
        while (do_idx + dont_idx < do_indexes.items.len + dont_indexes.items.len) {
            if (do_idx >= do_indexes.items.len) {
                try operations.append(Operation{ .Dont = DontOperation{} });
                dont_idx += 1;
            } else if (dont_idx >= dont_indexes.items.len) {
                try operations.append(Operation{ .Do = DoOperation{} });
                do_idx += 1;
            } else if (do_indexes.items[do_idx] < dont_indexes.items[dont_idx]) {
                try operations.append(Operation{ .Do = DoOperation{} });
                do_idx += 1;
            } else {
                try operations.append(Operation{ .Dont = DontOperation{} });
                dont_idx += 1;
            }
        }
    }
    
    part_1(operations.items);
    part_2(operations.items);
}

const MultOperation = struct {
    a: isize,
    b: isize,
};

const DoOperation = struct {};

const DontOperation = struct {};

const Operation = union(OperationEnum) {
    Mult: MultOperation,
    Do: DoOperation,
    Dont: DontOperation,
};

const OperationEnum = enum {
    Mult,
    Do,
    Dont,
};

fn part_1(operations: []Operation) void {
    var res: isize = 0;
    for (operations) |operation| {
        switch (operation) {
            .Mult => res += operation.Mult.a * operation.Mult.b,
            else => continue,
        }
    }
    std.debug.print("1. Total res of multiplications is {d}\n", .{res});
}

fn part_2(operations: []Operation) void {
    var enabled = true;
    var res: isize = 0;
    for (operations) |operation| {
        switch (operation) {
            .Mult => {
                if (!enabled) {
                    continue;
                }
                res += operation.Mult.a * operation.Mult.b;
            },
            .Do => enabled = true,
            .Dont => enabled = false,
        }
    }
    std.debug.print("2. Total res of multiplications is {d}\n", .{res});
}
