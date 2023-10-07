const std = @import("std");

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

fn getMostCalories(caloriesList: []const u8) i32 {
    var lines = std.mem.split(u8, caloriesList, "\n");

    var max: i32 = 0;
    var acc: i32 = 0;
    while (lines.next()) |line| {
        const cals = std.fmt.parseInt(i32, line, 10) catch {
            max = @max(max, acc);
            acc = 0;
            continue;
        };
        acc += cals;
    }

    max = @max(max, acc);
    return max;
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "case 0" {
    const input =
        \\1000
        \\2000
        \\3000
        \\
        \\4000
        \\
        \\5000
        \\6000
        \\
        \\7000
        \\8000
        \\9000
        \\
        \\10000
    ;

    try std.testing.expectEqual(getMostCalories(input), @as(i32, 24000));
}
