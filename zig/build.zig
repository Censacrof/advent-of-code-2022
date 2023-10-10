const std = @import("std");

const Build = std.Build;
const allocator = std.heap.page_allocator;
const ArrayList = std.ArrayList;

const ExecRun = struct {
    name: []const u8,
    run: *std.Build.Step.Run,
    run_unit_tests: *std.Build.Step.Run,
};

// Although this function looks imperative, note that its job is to
// declaratively construct a build graph that will be executed by an external
// runner.
fn addExecPath(b: *std.Build, target: std.zig.CrossTarget, optimize: std.builtin.OptimizeMode, path: []const u8, name: []const u8) ExecRun {
    const exe = b.addExecutable(.{
        .name = name,
        // In this case the main source file is merely a path, however, in more
        // complicated build scripts, this could be a generated file.
        .root_source_file = .{ .path = path },
        .target = target,
        .optimize = optimize,
    });

    // This declares intent for the executable to be installed into the
    // standard location when the user invokes the "install" step (the default
    // step when running `zig build`).
    b.installArtifact(exe);

    // This *creates* a Run step in the build graph, to be executed when another
    // step is evaluated that depends on it. The next line below will establish
    // such a dependency.
    const run_cmd = b.addRunArtifact(exe);

    // By making the run step depend on the install step, it will be run from the
    // installation directory rather than directly from within the cache directory.
    // This is not necessary, however, if the application depends on other installed
    // files, this ensures they will be present and in the expected location.
    run_cmd.step.dependOn(b.getInstallStep());

    // This allows the user to pass arguments to the application in the build
    // command itself, like this: `zig build run -- arg1 arg2 etc`
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    // Creates a step for unit testing. This only builds the test executable
    // but does not run it.
    const unit_tests = b.addTest(.{
        .root_source_file = .{ .path = path },
        .target = target,
        .optimize = optimize,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);

    return ExecRun{
        .name = name,
        .run = run_cmd,
        .run_unit_tests = run_unit_tests,
    };
}

pub fn build(b: *std.Build) !void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard optimization options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall. Here we do not
    // set a preferred release mode, allowing the user to decide how to optimize.
    const optimize = b.standardOptimizeOption(.{});

    var execRuns = ArrayList(ExecRun).init(allocator);
    defer execRuns.deinit();

    try execRuns.append(addExecPath(b, target, optimize, "src/d01/p01.zig", "d01p01"));
    try execRuns.append(addExecPath(b, target, optimize, "src/d01/p02.zig", "d01p02"));

    try execRuns.append(addExecPath(b, target, optimize, "src/d02/p01.zig", "d02p01"));
    try execRuns.append(addExecPath(b, target, optimize, "src/d02/p02.zig", "d02p02"));

    // This creates a build step. It will be visible in the `zig build --help` menu,
    // and can be selected like this: `zig build run`
    // This will evaluate the `run` step rather than the default, which is "install".
    const run_step = b.step("run", "Run the app");
    const exec_opt = b.option([]const u8, "exec", "filters by executable name");

    // Similar to creating the run step earlier, this exposes a `test` step to
    // the `zig build --help` menu, providing a way for the user to request
    // running the unit tests.
    const test_step = b.step("test", "Run unit tests");
    for (execRuns.items) |execRun| {
        if (exec_opt) |executable| {
            if (!std.mem.eql(u8, executable, execRun.name)) {
                continue;
            }
        }

        run_step.dependOn(&execRun.run.step);
        test_step.dependOn(&execRun.run_unit_tests.step);
    }
}
