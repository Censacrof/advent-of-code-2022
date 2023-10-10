const std = @import("std");

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var inputFile = try std.fs.cwd().openFile("src/d02/input.txt", .{ .mode = .read_only });
    defer inputFile.close();

    const file_size = (try inputFile.stat()).size;
    const allocator = std.heap.page_allocator;

    var buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);
    try inputFile.reader().readNoEof(buffer);

    const result = try computePrioritiesSum(buffer);

    try stdout.print("{d}\n", .{result});

    try bw.flush(); // don't forget to flush!
}

fn computePrioritiesSum(input: []const u8) !i32 {
    _ = input;
    return 0;
}

const GetPriorityError = error{InvalidChar};

fn getPriority(c: u8) GetPriorityError!i32 {
    if (c >= 'a' and c <= 'z') {
        return c - 'a' + 1;
    }

    if (c >= 'A' and c <= 'Z') {
        return c - 'A' + 27;
    }

    return GetPriorityError.InvalidChar;
}

test "priority fn" {
    try std.testing.expectEqual(@as(i32, 16), try getPriority('p'));
    try std.testing.expectEqual(@as(i32, 38), try getPriority('L'));
    try std.testing.expectEqual(@as(i32, 42), try getPriority('P'));
    try std.testing.expectEqual(@as(i32, 22), try getPriority('v'));
    try std.testing.expectEqual(@as(i32, 20), try getPriority('t'));
    try std.testing.expectEqual(@as(i32, 19), try getPriority('s'));
}

test "case 0" {
    return error.SkipZigTest;

    //const input =
    //    \\vJrwpWtwJgWrhcsFMMfFFhFp
    //    \\jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    //    \\PmmdzqPrVvPwwTWBwg
    //    \\wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    //    \\ttgJtRGJQctTZtZT
    //    \\CrZsJsPPZsGzwwsLwLmpwMDw
    //    \\
    //;

    //try std.testing.expectEqual(
    //    @as(i32, 157),
    //    try computePrioritiesSum(input),
    //);
}
