const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d05";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    const allocator = std.heap.page_allocator;

    var rules = ArrayList(OrderingRule).init(allocator);
    defer rules.deinit();

    var pagesList = ArrayList([]usize).init(allocator);
    defer pagesList.deinit();

    var buf: [1024]u8 = undefined;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (line.len == 0) {
            break;
        }

        var iter = std.mem.tokenizeScalar(u8, line, '|');

        try rules.append(OrderingRule{
            .before_page_number = try std.fmt.parseInt(u8, iter.next().?, 10),
            .after_page_number = try std.fmt.parseInt(u8, iter.next().?, 10),
        });
    }

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var iter = std.mem.tokenizeScalar(u8, line, ',');
        var pages = ArrayList(usize).init(allocator);

        while (iter.next()) |page| {
            try pages.append(try std.fmt.parseInt(u8, page, 10));
        }
        try pagesList.append(pages.items);
    }

    part_1(pagesList.items, rules.items);
    part_2(pagesList.items, rules.items);
}

fn part_1(pagesList: [][]usize, rules: []OrderingRule) void {
    var sum: usize = 0;
    for (pagesList) |pages| {
        if (!is_pages_list_valid(pages, rules)) {
            sum += pages[pages.len / 2];
        }
    }

    std.debug.print("1. Sum of middle page number from those correctly-ordered updates is {d}\n", .{sum});
}

fn part_2(pagesList: [][]usize, rules: []OrderingRule) void {
    var sum: usize = 0;
    for (pagesList) |pages| {
        var reordered = false;
        //suboptimal solution but it works well on input
        while (!is_pages_list_valid(pages, rules)) {
            for (rules) |rule| {
                if (rule.reorder_invalid_bages(pages)) {
                    reordered = true;
                }
            }
        }

        if (reordered) {
            sum += pages[pages.len / 2];
        }
    }

    std.debug.print("2. Sum of middle page numbers after correctly ordering just those updates {d}\n", .{sum});
}

fn is_pages_list_valid(pages: []usize, rules: []OrderingRule) bool {
    for (rules) |rule| {
        if (!rule.is_pages_valid(pages)) {
            return false;
        }
    }

    return true;
}

const OrderingRule = struct {
    before_page_number: usize,
    after_page_number: usize,

    fn reorder_invalid_bages(self: OrderingRule, pages: []usize) bool {
        var after_page_idx: ?usize = null;
        var before_page_idx: ?usize = null;

        for (pages, 0..pages.len) |page, idx| {
            if (page == self.after_page_number) {
                after_page_idx = idx;
            } else if (page == self.before_page_number) {
                before_page_idx = idx;
            }

            if (after_page_idx != null and before_page_idx != null) {
                break;
            }
        }

        if ((after_page_idx != null and before_page_idx != null) and (before_page_idx.? > after_page_idx.?)) {
            std.mem.swap(usize, &pages[before_page_idx.?], &pages[after_page_idx.?]);
            return true;
        }

        return false;
    }

    fn is_pages_valid(self: OrderingRule, pages: []usize) bool {
        var after_page_idx: ?usize = null;
        var before_page_idx: ?usize = null;

        for (pages, 0..pages.len) |page, idx| {
            if (page == self.after_page_number) {
                after_page_idx = idx;
            } else if (page == self.before_page_number) {
                before_page_idx = idx;
            }

            if (after_page_idx != null and before_page_idx != null) {
                break;
            }
        }

        return ((after_page_idx == null and before_page_idx == null) or
            (after_page_idx != null and before_page_idx == null) or
            (after_page_idx == null and before_page_idx != null) or
            (before_page_idx.? < after_page_idx.?));
    }
};
