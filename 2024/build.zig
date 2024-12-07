const std = @import("std");

pub fn build(b: *std.Build) !void {
    const allocator = std.heap.page_allocator;

    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});

    var srcDir = try std.fs.cwd().openDir("src", .{ .iterate = true });
    defer srcDir.close();

    var srcDirWalker = try std.fs.Dir.walk(srcDir, allocator);
    defer srcDirWalker.deinit();

    while (try srcDirWalker.next()) |entry| {
        if (entry.kind != .file) {
            continue;
        }

        if (entry.basename.len < 4 or !std.mem.eql(u8, entry.basename[entry.basename.len - 4 ..], ".zig")) {
            continue;
        }
        std.debug.print("{s}\n", .{entry.path});

        const exe = b.addExecutable(.{
            .name = entry.basename[0 .. entry.basename.len - 4],
            .root_source_file = b.path(try std.fs.path.join(allocator, &[_][]const u8{ "src", entry.path })),
            .target = target,
            .optimize = optimize,
        });

        b.installArtifact(exe);
        const run_cmd = b.addRunArtifact(exe);

        run_cmd.step.dependOn(b.getInstallStep());

        if (b.args) |args| {
            run_cmd.addArgs(args);
        }

        const step_name = try std.fmt.allocPrint(allocator, "{s} {s}", .{ "run", entry.basename });
        defer allocator.free(step_name);

        const run_step = b.step(step_name, "Run the app");
        run_step.dependOn(&run_cmd.step);
    }
}
