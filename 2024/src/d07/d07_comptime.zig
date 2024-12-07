const std = @import("std");
const ArrayList = std.ArrayList;

pub fn main() !void {
    @setEvalBranchQuota(1000 * 1000000);
    const equations = comptime try parse_equations();

    std.debug.print("1. Total calibration result is {d}\n", .{comptime part_1(equations)});
    std.debug.print("2. Total calibration result is {d}\n", .{comptime part_2(equations)});
}

fn parse_equations() ![]Equation {
    const file = @embedFile("d07");

    var line_iter = std.mem.tokenizeScalar(u8, file, '\n');

    var equations: [1000]Equation = undefined;

    var equations_idx = 0;
    while (line_iter.next()) |line| {
        if (line.len == 0) {
            break;
        }

        var iter = std.mem.tokenizeScalar(u8, line, ':');

        const result = try std.fmt.parseInt(isize, iter.next().?, 10);

        iter = std.mem.tokenizeScalar(u8, iter.next().?, ' ');
        var vals: [1000]isize = undefined;
        var vals_idx = 0;
        while (iter.next()) |val_str| {
            if (val_str.len == 0) {
                continue;
            }

            vals[vals_idx] = try std.fmt.parseInt(isize, val_str, 10);
            vals_idx += 1;
        }

        equations[equations_idx] = Equation{
            .result = result,
            .values = vals[0..vals_idx],
        };

        equations_idx += 1;
    }

    return equations[0..equations_idx];
}

fn part_1(equations: []Equation) isize {
    var total: isize = 0;
    for (equations) |equation| {
        if (equation.can_be_solved()) {
            total += equation.result;
        }
    }
    return total;
}

fn part_2(equations: []Equation) isize {
    var total: isize = 0;
    for (equations) |equation| {
        if (equation.can_be_solved_with_concat()) {
            total += equation.result;
        }
    }
    return total;
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
