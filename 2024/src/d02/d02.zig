const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d02";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var reportsList = ArrayList(ArrayList(isize)).init(allocator);
    defer reportsList.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var reports = ArrayList(isize).init(allocator);

        var token_iter = std.mem.tokenizeScalar(u8, line, ' ');
        while (token_iter.next()) |elem| {
            const num = try std.fmt.parseInt(isize, elem, 10);
            try reports.append(num);
        }

        try reportsList.append(reports);
    }

    part_1(reportsList);
    try part_2(allocator, reportsList);
}

fn isSafe(report: []const isize) bool {
    var current = report[0];
    var decreasing = false;
    var increasing = false;
    for (1..report.len) |i| {
        if (@abs(current - report[i]) < 1 or @abs(current - report[i]) > 3) {
            return false;
        }

        if (current < report[i]) {
            increasing = true;
        } else if (current > report[i]) {
            decreasing = true;
        }

        if (decreasing and increasing) {
            return false;
        }

        current = report[i];
    }
    return true;
}

fn part_1(reportsList: ArrayList(ArrayList(isize))) void {
    var total: usize = 0;
    for (reportsList.items) |report| {
        if (isSafe(report.items)) {
            total += 1;
        }
    }
    std.debug.print("1. Total number of safe reports is {d}\n", .{total});
}

fn part_2(allocator: std.mem.Allocator, reportsList: ArrayList(ArrayList(isize))) !void {
    var total: usize = 0;
    for (reportsList.items) |report| {
        if (isSafe(report.items)) {
            total += 1;
        } else {
            var buf = try allocator.alloc(isize, report.items.len - 1);
            for (0..report.items.len) |idx| {
                @memcpy(buf[0..idx], report.items[0..idx]);
                @memcpy(buf[idx..], report.items[idx + 1 ..]);
                if (isSafe(buf)) {
                    total += 1;
                    break;
                }
            }
            allocator.free(buf);
        }
    }

    std.debug.print("2. Total number of safe reports is {d}\n", .{total});
}
