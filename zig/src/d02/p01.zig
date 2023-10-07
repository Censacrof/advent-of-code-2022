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

    const result = compute_tota_score(buffer);

    try stdout.print("{d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}

fn compute_tota_score(input: []const u8) i32 {
    _ = input;
    return 0;
}

test "case 0" {
    const input =
        \\A Y
        \\B X
        \\C Z
        \\
    ;

    try std.testing.expectEqual(
        @as(i32, 15),
        compute_tota_score(input),
    );
}
