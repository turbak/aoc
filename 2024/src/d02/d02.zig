const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d02";
const allocator = std.heap.page_allocator;

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

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
    try part_2(reportsList);
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

fn isSafeRemovingOne(report: []const isize) !bool {
    var decreasing = false;
    var increasing = false;
    for (0..report.len - 1) |i| {
        const current = report[i];
        const next = report[i + 1];

        var tmpDecreasing = decreasing;
        var tmpIncreasing = increasing;
        if (current < next) {
            tmpIncreasing = true;
        } else if (current > next) {
            tmpDecreasing = true;
        }

        if (tmpDecreasing and tmpIncreasing or
            @abs(current - next) < 1 or @abs(current - next) > 3)
        {
            var reportWithRemovedCurrent = try ArrayList(isize).initCapacity(allocator, report.len - 1);
            defer reportWithRemovedCurrent.deinit();

            for (report, 0..report.len) |item, j| {
                if (i == j) {
                    continue;
                }
                try reportWithRemovedCurrent.append(item);
            }

            var reportWithRemovedNext = try ArrayList(isize).initCapacity(allocator, report.len - 1);
            defer reportWithRemovedNext.deinit();

            for (report, 0..report.len) |item, j| {
                if (i + 1 == j) {
                    continue;
                }
                try reportWithRemovedNext.append(item);
            }

            var reportWithRemovedPrevious = try ArrayList(isize).initCapacity(allocator, report.len - 1);
            defer reportWithRemovedPrevious.deinit();

            for (report, 0..report.len) |item, j| {
                if (i > 0 and i - 1 == j) {
                    continue;
                }
                try reportWithRemovedPrevious.append(item);
            }

            return isSafe(reportWithRemovedNext.items) or isSafe(reportWithRemovedCurrent.items) or isSafe(reportWithRemovedPrevious.items);
        }

        decreasing = tmpDecreasing;
        increasing = tmpIncreasing;
    }
    return true;
}

fn part_1(reportsList: ArrayList(ArrayList(isize))) void {
    var total: usize = 0;
    for (reportsList.items) |report| {
        if (isSafe(report.items)) {
            //std.debug.print("SAFE:   {any}\n", .{report.items});
            total += 1;
        } else {
            //std.debug.print("UNSAFE: {any}\n", .{report.items});
        }
    }
    std.debug.print("1. Total number of safe reports is {d}\n", .{total});
}

fn part_2(reportsList: ArrayList(ArrayList(isize))) !void {
    var total: usize = 0;
    for (reportsList.items) |report| {
        if (try isSafeRemovingOne(report.items)) {
            std.debug.print("SAFE:   {any}\n", .{report.items});
            total += 1;
        } else {
            std.debug.print("UNSAFE: {any}\n", .{report.items});
        }
    }

    std.debug.print("2. Total number of safe reports is {d}\n", .{total});
}
