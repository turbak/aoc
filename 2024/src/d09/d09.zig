const std = @import("std");
const ArrayList = std.ArrayList;

const PATH = "inputs/d09";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(PATH, .{});
    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var sys_map = SystemMap.init(allocator);
    defer sys_map.deinit();

    while (in_stream.readByte() catch null) |char| {
        const num = try std.fmt.parseInt(usize, &[_]u8{char}, 10);
        try sys_map.add(num);
    }

    var sys_map_2 = try sys_map.clone();
    defer sys_map_2.deinit();

    try part_1(&sys_map);
    try part_2(&sys_map_2);
}

fn part_1(map: *SystemMap) !void {
    try map.compact_fragment();
    std.debug.print("1. Checksum: {d}\n", .{map.checksum()});
}

fn part_2(map: *SystemMap) !void {
    try map.compact();
    std.debug.print("2. Checksum: {d}\n", .{map.checksum()});
}

const SystemMap = struct {
    allocator: std.mem.Allocator,
    memory: std.ArrayList(MemType),
    current_id: usize,

    pub fn init(allocator: std.mem.Allocator) SystemMap {
        return SystemMap{
            .allocator = allocator,
            .memory = std.ArrayList(MemType).init(allocator),
            .current_id = 0,
        };
    }

    pub fn deinit(self: *SystemMap) void {
        self.memory.deinit();
    }

    fn add(self: *SystemMap, mem_len: usize) !void {
        const is_free_space: bool = if (self.memory.getLastOrNull() == null) false else switch (self.memory.getLast()) {
            .FreeSpace => false,
            .File => true,
        };

        if (is_free_space) {
            try self.memory.append(MemType{ .FreeSpace = FreeSpace{ .len = mem_len } });
        } else {
            try self.memory.append(MemType{ .File = File{ .id = self.current_id, .len = mem_len } });
            self.current_id += 1;
        }
    }

    fn print(self: SystemMap) void {
        for (self.memory.items) |item| {
            switch (item) {
                .FreeSpace => {
                    for (0..item.FreeSpace.len) |_| {
                        std.debug.print(".", .{});
                    }
                },
                .File => {
                    for (0..item.File.len) |_| {
                        std.debug.print("{d}", .{item.File.id});
                    }
                },
            }
        }
        std.debug.print("\n", .{});
    }

    fn compact_fragment(self: *SystemMap) !void {
        while (self.memory.popOrNull()) |last_item_const| {
            var last_item = last_item_const;
            switch (last_item) {
                .FreeSpace => continue,
                .File => {},
            }

            while (last_item.File.len > 0) {
                const target_free_space_idx = findFirstFreePlaceIdx(self);
                if (target_free_space_idx == null) {
                    if (last_item.File.len > 0) {
                        try self.memory.append(last_item);
                    }
                    return;
                }

                const freeSpace = &self.memory.items[target_free_space_idx.?].FreeSpace;
                if (freeSpace.len < last_item.File.len) {
                    last_item.File.len -= freeSpace.len;
                    const len = self.memory.items[target_free_space_idx.?].FreeSpace.len;
                    self.memory.items[target_free_space_idx.?].FreeSpace.len = 0;
                    try self.memory.insert(target_free_space_idx.?, MemType{ .File = File{
                        .len = len,
                        .id = last_item.File.id,
                    } });
                } else {
                    self.memory.items[target_free_space_idx.?].FreeSpace.len -= last_item.File.len;
                    try self.memory.insert(target_free_space_idx.?, MemType{ .File = File{
                        .len = last_item.File.len,
                        .id = last_item.File.id,
                    } });
                    last_item.File.len = 0;
                }
            }
        }
    }

    fn compact(self: *SystemMap) !void {
        var remove_id: usize = 0;
        for (self.memory.items) |item| {
            switch (item) {
                .File => remove_id = @max(remove_id, item.File.id),
                .FreeSpace => continue,
            }
        }
        while (remove_id > 0) {
            const remove_idx = self.findFileIdxById(remove_id);
            const item = self.memory.items[remove_idx];

            const target_free_space_idx = self.findFirstFreePlaceIdxOfSize(item.File.len);
            if (target_free_space_idx == null or target_free_space_idx.? > remove_idx) {
                remove_id -= 1;
                continue;
            }

            self.memory.items[remove_idx] = MemType{ .FreeSpace = FreeSpace{ .len = item.File.len } };
            var mem_to_remove = item.File.len;
            var current_idx = target_free_space_idx.?;
            while (mem_to_remove > 0) {
                var new_item = self.memory.items[current_idx];
                if (new_item.FreeSpace.len < mem_to_remove) {
                    mem_to_remove -= item.FreeSpace.len;
                    new_item.FreeSpace.len = 0;
                } else {
                    self.memory.items[current_idx].FreeSpace.len -= mem_to_remove;
                    mem_to_remove = 0;
                }
                if (new_item.FreeSpace.len == 0) {
                    _ = self.memory.orderedRemove(current_idx);
                } else {
                    current_idx += 1;
                }
            }

            try self.memory.insert(target_free_space_idx.?, item);
            remove_id -= 1;
        }
    }

    fn findFileIdxById(self: SystemMap, id: usize) usize {
        for (0..self.memory.items.len, self.memory.items) |idx, item| {
            switch (item) {
                .File => {
                    if (item.File.id == id) {
                        return idx;
                    }
                    continue;
                },
                .FreeSpace => {
                    continue;
                },
            }
        }
        return 0;
    }

    fn checksum(self: SystemMap) usize {
        var current_pos: usize = 0;
        var sum: usize = 0;
        for (self.memory.items) |item| {
            switch (item) {
                .File => {
                    for (0..item.File.len) |_| {
                        sum += item.File.id * current_pos;
                        current_pos += 1;
                    }
                },
                .FreeSpace => {
                    current_pos += item.FreeSpace.len;
                    continue;
                },
            }
        }

        return sum;
    }

    fn findFirstFreePlaceIdx(self: *SystemMap) ?usize {
        for (self.memory.items, 0..self.memory.items.len) |item, new_idx| {
            switch (item) {
                .FreeSpace => {
                    if (item.FreeSpace.len == 0) {
                        _ = self.memory.orderedRemove(new_idx);
                        return self.findFirstFreePlaceIdx();
                    }
                    return new_idx;
                },
                .File => continue,
            }
        }

        return null;
    }

    fn findFirstFreePlaceIdxOfSize(self: *SystemMap, target_size: usize) ?usize {
        for (self.memory.items, 0..self.memory.items.len) |item, new_idx| {
            switch (item) {
                .FreeSpace => {
                    var current_size: usize = 0;
                    var new_new_idx = new_idx;
                    while (current_size < target_size and new_new_idx < self.memory.items.len) {
                        const new_item = self.memory.items[new_new_idx];
                        switch (new_item) {
                            .FreeSpace => current_size += new_item.FreeSpace.len,
                            .File => break,
                        }
                        new_new_idx += 1;
                    }
                    if (current_size < target_size) {
                        continue;
                    }
                    return new_idx;
                },
                .File => continue,
            }
        }

        return null;
    }

    fn clone(self: *SystemMap) !SystemMap {
        return SystemMap{
            .allocator = self.allocator,
            .memory = try self.memory.clone(),
            .current_id = self.current_id,
        };
    }
};

const File = struct {
    id: usize,
    len: usize,
};

const FreeSpace = struct { len: usize };

const MemType = union(MemTypeEnum) { File: File, FreeSpace: FreeSpace };

const MemTypeEnum = enum {
    File,
    FreeSpace,
};
