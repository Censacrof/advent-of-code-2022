const std = @import("std");
const ArrayList = std.ArrayList;
const allocator = std.heap.page_allocator;

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var inputFile = try std.fs.cwd().openFile("src/d01/input.txt", .{ .mode = .read_only });
    defer inputFile.close();

    const file_size = (try inputFile.stat()).size;

    var buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);
    try inputFile.reader().readNoEof(buffer);

    const result = try get_top_3_total_calories(buffer);

    try stdout.print("{d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}

fn get_top_3_total_calories(caloriesList: []const u8) !i32 {
    var lines = std.mem.split(u8, caloriesList, "\n");

    var elvsCalories = ArrayList(i32).init(allocator);
    defer elvsCalories.deinit();

    var acc: i32 = 0;
    while (lines.next()) |line| {
        const cals = std.fmt.parseInt(i32, line, 10) catch {
            try elvsCalories.append(acc);
            acc = 0;
            continue;
        };
        acc += cals;
    }

    try elvsCalories.append(acc);

    var elvesCaloriesSlice = try elvsCalories.toOwnedSlice();
    std.sort.pdq(i32, elvesCaloriesSlice, {}, std.sort.desc(i32));

    var tot: i32 = 0;
    for (elvesCaloriesSlice, 0..) |cals, i| {
        if (i > 2) {
            break;
        }

        tot += cals;
    }

    return tot;
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

    try std.testing.expectEqual(
        @as(i32, 45000),
        try get_top_3_total_calories(input),
    );
}
