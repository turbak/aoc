const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d12";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var grid = ArrayList([]Plot).init(allocator);
    defer grid.deinit();

    var buf: [1024]u8 = undefined;

    var x: usize = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var plots = try allocator.alloc(Plot, line.len);
        for (0..plots.len) |y| {
            plots[y] = Plot{
                .x = x,
                .y = y,
                .id = line[y],
            };
        }

        try grid.append(plots);
        x += 1;
    }

    var added = try ArrayList([]bool).initCapacity(allocator, grid.items.len);
    defer added.deinit();
    for (grid.items) |row| {
        const row_b = try allocator.alloc(bool, row.len);
        @memset(row_b, false);

        added.appendAssumeCapacity(row_b);
    }

    const map = PlotMap{ .map = grid.items };
    var regions_list = ArrayList(Region).init(allocator);
    defer regions_list.deinit();

    var plot_stack = ArrayList(Plot).init(allocator);
    defer plot_stack.deinit();

    for (grid.items, 0..grid.items.len) |row, row_idx| {
        for (row, 0..row.len) |item, col_idx| {
            if (added.items[row_idx][col_idx]) {
                continue;
            }

            var reg = try Region.init(allocator, item);

            try plot_stack.append(item);
            added.items[row_idx][col_idx] = true;
            while (plot_stack.popOrNull()) |plot| {
                const neighbours = try map.neighbors(allocator, plot);
                defer allocator.free(neighbours);
                for (neighbours) |neighbour| {
                    if (!added.items[neighbour.x][neighbour.y]) {
                        try plot_stack.append(neighbour);
                        added.items[neighbour.x][neighbour.y] = true;
                        try reg.addPlot(neighbour);
                    }
                }
            }
            try regions_list.append(reg);
        }
    }

    part_1(regions_list.items);
    part_2(allocator, regions_list.items);
}

fn part_1(regions: []Region) void {
    var total_price: usize = 0;
    for (regions) |region| {
        total_price += region.area() * region.perimeter();
    }

    std.debug.print("1. Total price of fencing is {d} .\n", .{total_price});
}

fn part_2(_: std.mem.Allocator, regions: []Region) void {
    var total_price: usize = 0;
    for (regions) |region| {
        total_price += region.area() * region.numberOfSides();
    }

    std.debug.print("2. Total price of fencing is {d} .\n", .{total_price});
}

const Region = struct {
    allocator: std.mem.Allocator,
    plots: ArrayList(Plot),
    id: u8,

    fn isPartOfRegion(self: Region, new_plot: Plot) bool {
        if (new_plot.id != self.id) {
            return false;
        }

        for (self.plots.items) |plot| {
            if (plot.isAdjacent(new_plot)) {
                return true;
            }
        }
        return false;
    }

    fn addPlot(self: *Region, new_plot: Plot) !void {
        for (self.plots.items) |plot| {
            if (plot.eql(new_plot)) {
                return;
            }
        }

        try self.plots.append(new_plot);
    }

    fn init(allocator: std.mem.Allocator, init_plot: Plot) !Region {
        var plots = try ArrayList(Plot).initCapacity(allocator, 1);
        plots.appendAssumeCapacity(init_plot);

        return Region{
            .allocator = allocator,
            .plots = plots,
            .id = init_plot.id,
        };
    }

    fn deinit(self: Region) void {
        self.plots.deinit();
    }

    fn print(self: Region) void {
        var min_x: usize = std.math.maxInt(usize);
        var min_y: usize = std.math.maxInt(usize);
        var max_x: usize = 0;
        var max_y: usize = 0;

        for (self.plots.items) |plot| {
            min_x = @min(plot.x, min_x);
            min_y = @min(plot.y, min_y);
            max_y = @max(plot.y, max_y);
            max_x = @max(plot.x, max_x);
        }

        for (min_x..max_x + 1) |x| {
            for (min_y..max_y + 1) |y| {
                if (self.getPlot(x, y)) |plot| {
                    std.debug.print("{c}", .{plot.id});
                } else {
                    std.debug.print(".", .{});
                }
            }
            std.debug.print("\n", .{});
        }

        std.debug.print("\n", .{});
    }

    fn area(self: Region) usize {
        return self.plots.items.len;
    }

    fn perimeter(self: Region) usize {
        var total_perimeter: usize = 0;

        for (self.plots.items) |plot| {
            total_perimeter += 4;
            for (self.plots.items) |plot_2| {
                if (plot.eql(plot_2)) {
                    continue;
                } else if (plot.isAdjacent(plot_2)) {
                    total_perimeter -= 1;
                }
            }
        }
        return total_perimeter;
    }

    fn numberOfSides(self: Region) usize {
        var min_x: isize = std.math.maxInt(isize);
        var min_y: isize = std.math.maxInt(isize);
        var max_x: isize = 0;
        var max_y: isize = 0;

        for (self.plots.items) |plot| {
            const x: isize = @intCast(plot.x);
            const y: isize = @intCast(plot.y);
            min_x = @min(x, min_x);
            min_y = @min(y, min_y);
            max_y = @max(y, max_y);
            max_x = @max(x, max_x);
        }

        var total_sides: usize = 0;
        var x = min_x;
        while (x < max_x + 1) {
            //up
            var left_y: isize = min_y;
            while (left_y < max_y + 1) {
                if (self.getPlot(x - 1, left_y) != null or self.getPlot(x, left_y) == null) {
                    left_y += 1;
                    continue;
                }
                total_sides += 1;
                while (self.getPlot(x - 1, left_y) == null and self.getPlot(x, left_y) != null) {
                    left_y += 1;
                }
            }

            //down
            var right_y = min_y;
            while (right_y < max_y + 1) {
                if (self.getPlot(x + 1, right_y) != null or self.getPlot(x, right_y) == null) {
                    right_y += 1;
                    continue;
                }

                total_sides += 1;
                while (self.getPlot(x + 1, right_y) == null and self.getPlot(x, right_y) != null) {
                    right_y += 1;
                }
            }
            x += 1;
        }

        var y = min_y;
        while (y < max_y + 1) {
            //left
            var up_x = min_x;
            while (up_x < max_x + 1) {
                if (self.getPlot(up_x, y - 1) != null or self.getPlot(up_x, y) == null) {
                    up_x += 1;
                    continue;
                }
                total_sides += 1;
                while (self.getPlot(up_x, y - 1) == null and self.getPlot(up_x, y) != null) {
                    up_x += 1;
                }
            }

            //right
            var down_x = min_x;
            while (down_x < max_x + 1) {
                if (self.getPlot(down_x, y + 1) != null or self.getPlot(down_x, y) == null) {
                    down_x += 1;
                    continue;
                }
                total_sides += 1;
                while (self.getPlot(down_x, y + 1) == null and self.getPlot(down_x, y) != null) {
                    down_x += 1;
                }
            }
            y += 1;
        }

        return total_sides;
    }

    fn getPlot(self: Region, x: isize, y: isize) ?Plot {
        for (self.plots.items) |plot| {
            if (plot.x == x and plot.y == y) {
                return plot;
            }
        }
        return null;
    }
};

const Plot = struct {
    x: usize,
    y: usize,
    id: u8,

    fn isAdjacent(self: Plot, other: Plot) bool {
        if (self.id != other.id) return false;

        const y: isize = @intCast(self.y);
        const x: isize = @intCast(self.x);
        const o_y: isize = @intCast(other.y);
        const o_x: isize = @intCast(other.x);

        return @abs(y - o_y) + @abs(x - o_x) == 1;
    }

    fn eql(self: Plot, other: Plot) bool {
        return self.id == other.id and self.x == other.x and self.y == other.y;
    }
};

const PlotMap = struct {
    map: [][]Plot,

    fn neighbors(self: PlotMap, allocator: std.mem.Allocator, current: Plot) ![]Plot {
        var neighbor_list = ArrayList(Plot).init(allocator);

        if (current.x > 0) {
            const new = self.get(current.x - 1, current.y);
            if (new.id == current.id) {
                try neighbor_list.append(new);
            }
        }

        if (current.y > 0) {
            const new = self.get(current.x, current.y - 1);
            if (new.id == current.id) {
                try neighbor_list.append(new);
            }
        }

        if (current.x < self.map.len - 1) {
            const new = self.get(current.x + 1, current.y);
            if (new.id == current.id) {
                try neighbor_list.append(new);
            }
        }

        if (current.y < self.map[0].len - 1) {
            const new = self.get(current.x, current.y + 1);
            if (new.id == current.id) {
                try neighbor_list.append(new);
            }
        }
        return neighbor_list.toOwnedSlice();
    }

    fn get(self: PlotMap, x: usize, y: usize) Plot {
        return self.map[x][y];
    }
};
