const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d07";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    const allocator = std.heap.page_allocator;

    var equations = ArrayList(Equation).init(allocator);
    defer equations.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0) {
            break;
        }

        var iter = std.mem.tokenizeScalar(u8, line, ':');

        const result = try std.fmt.parseInt(isize, iter.next().?, 10);

        iter = std.mem.tokenizeScalar(u8, iter.next().?, ' ');
        var vals = ArrayList(isize).init(allocator);
        while (iter.next()) |val_str| {
            if (val_str.len == 0) {
                continue;
            }

            try vals.append(try std.fmt.parseInt(isize, val_str, 10));
        }

        try equations.append(Equation{
            .result = result,
            .values = vals.items,
        });
    }

    part_1(equations.items);
    part_2(equations.items);
}

fn part_1(equations: []Equation) void {
    var total: isize = 0;
    for (equations) |equation| {
        if (equation.can_be_solved()) {
            total += equation.result;
        }
    }

    std.debug.print("1. Total calibration result is {d}\n", .{total});
}

fn part_2(equations: []Equation) void {
    var total: isize = 0;
    for (equations) |equation| {
        if (equation.can_be_solved_with_concat()) {
            total += equation.result;
        }
    }

    std.debug.print("2. Total calibration result is {d}\n", .{total});
}

const Equation = struct {
    result: isize,
    values: []isize,

    pub fn can_be_solved(self: Equation) bool {
        return self._can_be_solved(self.values[1..], self.values[0]);
    }

    fn _can_be_solved(self: Equation, values_left: []isize, current_result: isize) bool {
        if (values_left.len == 0) {
            return current_result == self.result;
        }

        return self._can_be_solved(values_left[1..], current_result + values_left[0]) or
            self._can_be_solved(values_left[1..], current_result * values_left[0]);
    }

    pub fn can_be_solved_with_concat(self: Equation) bool {
        return self._can_be_solved_with_concat(self.values[1..], self.values[0]);
    }

    fn _can_be_solved_with_concat(self: Equation, values_left: []isize, current_result: isize) bool {
        if (values_left.len == 0) {
            return current_result == self.result;
        }

        return self._can_be_solved_with_concat(values_left[1..], current_result + values_left[0]) or
            self._can_be_solved_with_concat(values_left[1..], current_result * values_left[0]) or
            self._can_be_solved_with_concat(values_left[1..], concant_vals(current_result, values_left[0]));
    }

    fn concant_vals(a: isize, b: isize) isize {
        var tmp_b = b;
        var mult: isize = 1;
        while (tmp_b > 0) {
            tmp_b = @divFloor(tmp_b, 10);
            mult *= 10;
        }

        return a * mult + b;
    }
};
