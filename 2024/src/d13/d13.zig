const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d13_example";

const A_BUTTON_PRICE: usize = 3;
const B_BUTTON_PRICE: usize = 1;
pub fn main() !void {
    var path: []const u8 = PATH[0..];
    if (std.os.argv.len == 2) {
        path = std.mem.sliceTo(std.os.argv[1], 0);
    }

    const file = try std.fs.cwd().openFile(path, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var machines_list = ArrayList(Machine).init(allocator);
    defer machines_list.deinit();

    var buf1: [1024]u8 = undefined;
    var buf2: [1024]u8 = undefined;
    var buf3: [1024]u8 = undefined;
    while (true) {
        const a_button_line = try in_stream.readUntilDelimiterOrEof(&buf1, '\n') orelse break;
        const b_button_line = try in_stream.readUntilDelimiterOrEof(&buf2, '\n') orelse break;
        const prize_line = try in_stream.readUntilDelimiterOrEof(&buf3, '\n') orelse break;

        var a_button_iter = std.mem.tokenizeSequence(u8, a_button_line["Button A: ".len..], ", ");
        var b_button_iter = std.mem.tokenizeSequence(u8, b_button_line["Button B: ".len..], ", ");
        var prize_iter = std.mem.tokenizeSequence(u8, prize_line["Prize: ".len..], ", ");
        try machines_list.append(Machine{
            .button_a = ButtonAction{
                .x_delta = try std.fmt.parseInt(isize, a_button_iter.next().?[2..], 10),
                .y_delta = try std.fmt.parseInt(isize, a_button_iter.next().?[2..], 10),
                .price = A_BUTTON_PRICE,
            },
            .button_b = ButtonAction{
                .x_delta = try std.fmt.parseInt(isize, b_button_iter.next().?[2..], 10),
                .y_delta = try std.fmt.parseInt(isize, b_button_iter.next().?[2..], 10),
                .price = B_BUTTON_PRICE,
            },
            .prize = Prize{
                .x = try std.fmt.parseInt(usize, prize_iter.next().?[2..], 10),
                .y = try std.fmt.parseInt(usize, prize_iter.next().?[2..], 10),
            },
        });

        _ = try in_stream.readUntilDelimiterOrEof(&buf1, '\n') orelse break;
    }
    part_1(machines_list.items);
    part_2(machines_list.items);
}

fn part_1(machines: []Machine) void {
    var total: usize = 0;
    for (machines) |machine| {
        total += machine.solve() orelse 0;
    }
    std.debug.print("1. {d} fewest tokens you would have to spend to win all prizes.\n", .{total});
}

const PART_2_ADDITION: usize = 10000000000000;

// TOO DAMN SLOW
fn part_2(machines: []Machine) void {
    var total: usize = 0;
    for (machines) |machine| {
        var machine_copy = machine;
        machine_copy.prize.x += PART_2_ADDITION;
        machine_copy.prize.y += PART_2_ADDITION;
        total += machine_copy.solve() orelse 0;
    }
    std.debug.print("2. {d} fewest tokens you would have to spend to win all prizes.\n", .{total});
}

const Machine = struct {
    button_a: ButtonAction,
    button_b: ButtonAction,
    prize: Prize,

    fn solve(self: Machine) ?usize {
        var start_x: isize = 0;
        var start_y: isize = 0;
        var start_price: usize = 0;

        var min_price: usize = std.math.maxInt(usize);

        while (start_x < self.prize.x and start_y < self.prize.y) {
            var x = start_x;
            var y = start_y;
            var price = start_price;

            while (x < self.prize.x and y < self.prize.y) {
                x += self.button_b.x_delta;
                y += self.button_b.y_delta;
                price += self.button_b.price;
            }
            if (x == self.prize.x and y == self.prize.y) {
                min_price = @min(price, min_price);
            }

            start_x += self.button_a.x_delta;
            start_y += self.button_a.y_delta;
            start_price += self.button_a.price;
        }

        if (min_price == std.math.maxInt(usize)) {
            return null;
        }

        return min_price;
    }
};

const ButtonAction = struct {
    x_delta: isize,
    y_delta: isize,
    price: usize,
};

const Prize = struct {
    x: usize,
    y: usize,
};

fn gcd(a: isize, b: isize) isize {
    var cd: isize = 1;
    var max_cd: isize = 1;
    while (cd <= @min(a, b)) : (cd += 1) {
        if (@mod(a, cd) == 0 and @mod(b, cd) == 0) {
            max_cd = cd;
        }
    }
    return max_cd;
}
