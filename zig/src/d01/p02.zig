const std = @import("std");

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var inputFile = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer inputFile.close();

    const file_size = (try inputFile.stat()).size;
    const allocator = std.heap.page_allocator;

    var buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);
    try inputFile.reader().readNoEof(buffer);

    const result = get_most_calories(buffer);

    try stdout.print("foffo {d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}

fn get_most_calories(caloriesList: []const u8) i32 {
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

    try std.testing.expectEqual(get_most_calories(input), @as(i32, 24000));
}
